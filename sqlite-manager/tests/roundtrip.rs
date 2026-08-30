use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use rusqlite::Connection;

const BINARY: &str = env!("CARGO_BIN_EXE_sqlite-manager");

struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new(label: &str) -> Self {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("sqlite-manager-test-{label}-{stamp}"));
        std::fs::create_dir_all(&root).unwrap();
        Self { root }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn run(arguments: &[&str]) -> Output {
    Command::new(BINARY).args(arguments).output().unwrap()
}

fn run_shell(arguments: &[&str], input: &str) -> String {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new(BINARY)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn fingerprint(path: &Path) -> Vec<u8> {
    std::fs::read(path).unwrap()
}

fn build_source(path: &Path) {
    let connection = Connection::open(path).unwrap();
    connection
        .execute_batch(
            r#"
            CREATE TABLE author(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE);
            CREATE TABLE "order"(
                id INTEGER PRIMARY KEY,
                author_id INTEGER REFERENCES author(id),
                total REAL,
                qty INTEGER,
                line REAL GENERATED ALWAYS AS (total * qty) STORED,
                payload BLOB
            );
            CREATE INDEX idx_order_author ON "order"(author_id);
            CREATE VIEW big AS SELECT * FROM "order" WHERE total > 100;
            CREATE TRIGGER bump AFTER INSERT ON "order"
                BEGIN UPDATE author SET name = name WHERE id = NEW.author_id; END;
            CREATE TABLE "od""d"("a b" TEXT);

            INSERT INTO author(name) VALUES ('Ana'), ('it''s Bob'), ('Zoë ünïcode');
            INSERT INTO "order"(author_id, total, qty, payload) VALUES
                (1, 250.0, 2, x'DEADBEEF'),
                (2, 0.1, 1, NULL),
                (3, 1e300, 3, x'');
            INSERT INTO "od""d" VALUES ('multi
line');
            "#,
        )
        .unwrap();
}

fn scalar(path: &Path, query: &str) -> String {
    let connection = Connection::open(path).unwrap();
    connection
        .query_row(query, [], |row| row.get::<_, String>(0))
        .unwrap()
}

#[test]
fn a_dump_reconstructs_the_database_without_touching_the_source() {
    let sandbox = Sandbox::new("roundtrip");
    let source = sandbox.path("shop.db");
    build_source(&source);
    let before = fingerprint(&source);

    let dump = sandbox.path("dump");
    let output = run(&[
        "dump",
        source.to_str().unwrap(),
        "-o",
        dump.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "dump failed: {output:?}");

    assert_eq!(
        before,
        fingerprint(&source),
        "the source database was modified"
    );
    assert!(
        !sandbox.path("shop.db-wal").exists(),
        "a write-ahead log was left behind"
    );
    assert!(
        !sandbox.path("shop.db-shm").exists(),
        "a shared memory file was left behind"
    );

    let restored = sandbox.path("restored.db");
    let output = run(&[
        "import",
        dump.to_str().unwrap(),
        "-o",
        restored.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "import failed: {output:?}");

    let query = "SELECT group_concat(name, '|') FROM (SELECT name FROM author ORDER BY id)";
    assert_eq!(scalar(&source, query), scalar(&restored, query));

    let totals = "SELECT group_concat(total || ':' || qty || ':' || line, '|') \
                  FROM (SELECT total, qty, line FROM \"order\" ORDER BY id)";
    assert_eq!(scalar(&source, totals), scalar(&restored, totals));

    let blobs = "SELECT group_concat(coalesce(hex(payload), 'NULL'), '|') \
                 FROM (SELECT payload FROM \"order\" ORDER BY id)";
    assert_eq!(scalar(&source, blobs), scalar(&restored, blobs));

    let odd = "SELECT \"a b\" FROM \"od\"\"d\"";
    assert_eq!(scalar(&source, odd), scalar(&restored, odd));

    assert_eq!(scalar(&restored, "PRAGMA integrity_check"), "ok");
}

#[test]
fn a_trigger_does_not_fire_again_while_the_data_loads() {
    let sandbox = Sandbox::new("triggers");
    let source = sandbox.path("counts.db");
    let connection = Connection::open(&source).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE score(id INTEGER PRIMARY KEY, value INTEGER NOT NULL DEFAULT 0);
             CREATE TABLE hit(id INTEGER PRIMARY KEY);
             CREATE TRIGGER climb AFTER INSERT ON hit
                 BEGIN UPDATE score SET value = value + 1 WHERE id = 1; END;
             INSERT INTO score(id, value) VALUES (1, 0);
             INSERT INTO hit(id) VALUES (1), (2), (3);",
        )
        .unwrap();
    drop(connection);

    let dump = sandbox.path("dump");
    assert!(
        run(&[
            "dump",
            source.to_str().unwrap(),
            "-o",
            dump.to_str().unwrap()
        ])
        .status
        .success()
    );

    let restored = sandbox.path("restored.db");
    assert!(
        run(&[
            "import",
            dump.to_str().unwrap(),
            "-o",
            restored.to_str().unwrap()
        ])
        .status
        .success()
    );

    assert_eq!(
        scalar(
            &source,
            "SELECT cast(value AS TEXT) FROM score WHERE id = 1"
        ),
        scalar(
            &restored,
            "SELECT cast(value AS TEXT) FROM score WHERE id = 1"
        ),
        "the trigger fired again during the restore"
    );
}

#[test]
fn a_hot_write_ahead_log_is_included_and_the_source_is_left_alone() {
    let sandbox = Sandbox::new("wal");
    let source = sandbox.path("live.db");
    let connection = Connection::open(&source).unwrap();
    connection
        .query_row("PRAGMA journal_mode=WAL", [], |row| row.get::<_, String>(0))
        .unwrap();
    connection
        .execute_batch("CREATE TABLE t(a INTEGER); INSERT INTO t(a) VALUES (1), (2), (3);")
        .unwrap();

    let before: Vec<Vec<u8>> = ["", "-wal", "-shm"]
        .iter()
        .map(|suffix| {
            let mut name = source.clone().into_os_string();
            name.push(suffix);
            std::fs::read(PathBuf::from(name)).unwrap_or_default()
        })
        .collect();

    let dump = sandbox.path("dump");
    let output = run(&[
        "dump",
        source.to_str().unwrap(),
        "-o",
        dump.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "dump failed: {output:?}");

    let after: Vec<Vec<u8>> = ["", "-wal", "-shm"]
        .iter()
        .map(|suffix| {
            let mut name = source.clone().into_os_string();
            name.push(suffix);
            std::fs::read(PathBuf::from(name)).unwrap_or_default()
        })
        .collect();
    assert_eq!(before, after, "the live database or its log was modified");

    let data = std::fs::read_to_string(dump.join("data.sql")).unwrap();
    assert_eq!(
        data.matches("INSERT INTO").count(),
        3,
        "rows in the log were lost"
    );
    drop(connection);
}

#[test]
fn check_passes_a_sound_dump_and_fails_a_damaged_one() {
    let sandbox = Sandbox::new("check");
    let source = sandbox.path("shop.db");
    build_source(&source);

    let dump = sandbox.path("dump");
    assert!(
        run(&[
            "dump",
            source.to_str().unwrap(),
            "-o",
            dump.to_str().unwrap()
        ])
        .status
        .success()
    );
    assert!(run(&["check", dump.to_str().unwrap()]).status.success());

    let data = dump.join("data.sql");
    let sound = std::fs::read_to_string(&data).unwrap();

    std::fs::write(&data, sound.replace("INSERT INTO", "INSRT INTO")).unwrap();
    assert!(
        !run(&["check", dump.to_str().unwrap()]).status.success(),
        "a syntax error slipped through"
    );

    std::fs::write(&data, &sound[..sound.len() / 2]).unwrap();
    assert!(
        !run(&["check", dump.to_str().unwrap()]).status.success(),
        "a truncated dump slipped through"
    );

    std::fs::write(&data, sound.replace("(1,1,250.0,2", "(1,404,250.0,2")).unwrap();
    assert!(
        !run(&["check", dump.to_str().unwrap()]).status.success(),
        "a broken reference slipped through"
    );
}

#[test]
fn import_refuses_to_overwrite_unless_it_is_told_to() {
    let sandbox = Sandbox::new("overwrite");
    let source = sandbox.path("shop.db");
    build_source(&source);

    let dump = sandbox.path("dump");
    assert!(
        run(&[
            "dump",
            source.to_str().unwrap(),
            "-o",
            dump.to_str().unwrap()
        ])
        .status
        .success()
    );

    let restored = sandbox.path("restored.db");
    assert!(
        run(&[
            "import",
            dump.to_str().unwrap(),
            "-o",
            restored.to_str().unwrap()
        ])
        .status
        .success()
    );
    assert!(
        !run(&[
            "import",
            dump.to_str().unwrap(),
            "-o",
            restored.to_str().unwrap()
        ])
        .status
        .success()
    );
    assert!(
        run(&[
            "import",
            dump.to_str().unwrap(),
            "-o",
            restored.to_str().unwrap(),
            "--force"
        ])
        .status
        .success()
    );
}

#[test]
fn no_arguments_prints_the_help_and_unknown_commands_fail() {
    let help = run(&[]);
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("COMMANDS:"));

    let bogus = run(&["frobnicate"]);
    assert!(!bogus.status.success());
    assert!(String::from_utf8_lossy(&bogus.stderr).contains("unknown command"));
}

#[test]
fn safe_mode_refuses_every_command_that_would_write_a_database() {
    let sandbox = Sandbox::new("safe");
    let source = sandbox.path("shop.db");
    build_source(&source);
    let before = fingerprint(&source);

    let dump = sandbox.path("dump");
    assert!(
        run(&[
            "--safe",
            "dump",
            source.to_str().unwrap(),
            "-o",
            dump.to_str().unwrap()
        ])
        .status
        .success(),
        "dump only reads the database, safe mode should allow it"
    );

    let restored = sandbox.path("restored.db");
    let blocked = run(&[
        "--safe",
        "import",
        dump.to_str().unwrap(),
        "-o",
        restored.to_str().unwrap(),
    ]);
    assert!(!blocked.status.success());
    assert!(String::from_utf8_lossy(&blocked.stderr).contains("--safe"));
    assert!(!restored.exists(), "safe mode created a database file");

    let blocked = run(&["sql", source.to_str().unwrap(), "--write", "--safe"]);
    assert!(
        !blocked.status.success(),
        "safe mode opened the database for writing"
    );

    assert_eq!(
        before,
        fingerprint(&source),
        "safe mode modified the source database"
    );
}

#[test]
fn the_shell_commands_are_not_handed_to_sqlite_as_sql() {
    let sandbox = Sandbox::new("shell");
    let source = sandbox.path("shop.db");
    build_source(&source);

    let output = run_shell(
        &["sql", source.to_str().unwrap()],
        "tables;\ndesc table \"order\";\nhelp;\nquit;\n",
    );

    assert!(output.contains("author"), "tables; listed no tables");
    assert!(output.contains("big"), "tables; left out the views");

    assert!(
        output.contains("author_id"),
        "desc table did not list the columns"
    );
    assert!(
        output.contains("references author_id -> author.id"),
        "desc table did not list the foreign keys"
    );
    assert!(
        output.contains("idx_order_author"),
        "desc table did not list the indexes"
    );
    assert!(
        output.contains("GENERATED ALWAYS"),
        "desc table did not print the full CREATE statement"
    );

    assert!(
        output.contains("desc table NAME"),
        "help; listed no commands"
    );
    assert!(
        !output.contains("syntax error"),
        "a command was executed as SQL: {output}"
    );
}
