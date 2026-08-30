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

fn run_stdin(arguments: &[&str], input: &str) -> Output {
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
    child.wait_with_output().unwrap()
}

fn run_shell(arguments: &[&str], input: &str) -> String {
    String::from_utf8_lossy(&run_stdin(arguments, input).stdout).to_string()
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

    let blocked = run(&[
        "--safe",
        "sql-pipe",
        "--write",
        source.to_str().unwrap(),
        "DELETE FROM author",
    ]);
    assert!(
        !blocked.status.success(),
        "safe mode let sql-pipe open the database for writing"
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

#[test]
fn the_header_pragmas_travel_with_the_dump() {
    let sandbox = Sandbox::new("header");
    let source = sandbox.path("stamped.db");
    let connection = Connection::open(&source).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE t(a INTEGER);
             INSERT INTO t(a) VALUES (1);
             PRAGMA user_version=7;
             PRAGMA application_id=999;",
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

    for pragma in ["user_version", "application_id"] {
        let query = format!("SELECT cast({pragma} AS TEXT) FROM pragma_{pragma}");
        assert_eq!(
            scalar(&source, &query),
            scalar(&restored, &query),
            "{pragma} was lost, so the restore is not the same database"
        );
    }
}

#[test]
fn a_database_that_never_set_the_pragmas_keeps_a_clean_schema() {
    let sandbox = Sandbox::new("unstamped");
    let source = sandbox.path("plain.db");
    let connection = Connection::open(&source).unwrap();
    connection
        .execute_batch("CREATE TABLE t(a INTEGER);")
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

    let schema = std::fs::read_to_string(dump.join("schema.sql")).unwrap();
    assert!(
        !schema.contains("user_version") && !schema.contains("application_id"),
        "a database that set neither pragma should not carry them: {schema}"
    );
}

#[test]
fn check_catches_a_dump_that_quietly_lost_a_row() {
    let sandbox = Sandbox::new("shortfall");
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
    let kept: Vec<&str> = sound.lines().collect();
    let victim = kept
        .iter()
        .position(|line| line.starts_with("INSERT INTO \"author\""))
        .expect("the dump should insert an author");
    let short: Vec<&str> = kept
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != victim)
        .map(|(_, line)| *line)
        .collect();
    std::fs::write(&data, short.join("\n")).unwrap();

    let output = run(&["check", dump.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "a dump missing a row was called sound"
    );
    let complaint = String::from_utf8_lossy(&output.stderr);
    assert!(
        complaint.contains("author") && complaint.contains("manifest"),
        "the failure should name the short table: {complaint}"
    );
}

#[test]
fn a_dump_taken_before_manifests_still_checks() {
    let sandbox = Sandbox::new("legacy");
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
    std::fs::remove_file(dump.join("manifest.txt")).unwrap();

    let output = run(&["check", dump.to_str().unwrap()]);
    assert!(
        output.status.success(),
        "an older dump should still check: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("not verified"),
        "the missing manifest should be called out"
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
        .success(),
        "an older dump should still import"
    );
}

#[test]
fn sql_pipe_reads_the_statement_from_the_line_or_from_stdin() {
    let sandbox = Sandbox::new("pipe-input");
    let source = sandbox.path("shop.db");
    build_source(&source);
    let path = source.to_str().unwrap();

    let from_line = run(&["sql-pipe", path, "SELECT name FROM author ORDER BY id"]);
    assert!(from_line.status.success());
    let from_line = String::from_utf8_lossy(&from_line.stdout).to_string();

    let from_stdin = run_stdin(
        &["sql-pipe", path],
        "SELECT name FROM author ORDER BY id;\n",
    );
    assert!(from_stdin.status.success());
    let from_stdin = String::from_utf8_lossy(&from_stdin.stdout).to_string();

    assert!(from_line.contains("Zoë ünïcode"), "{from_line}");
    assert_eq!(
        from_line, from_stdin,
        "the two ways of handing sql-pipe a statement must answer the same thing"
    );

    let words = run(&["sql-pipe", path, "SELECT", "name", "FROM", "author"]);
    assert!(
        String::from_utf8_lossy(&words.stdout).contains("Ana"),
        "an unquoted statement is the rest of the line, joined back together"
    );
}

#[test]
fn sql_pipe_treats_everything_after_the_database_as_sql() {
    let sandbox = Sandbox::new("pipe-flags");
    let source = sandbox.path("shop.db");
    build_source(&source);

    let output = run(&[
        "sql-pipe",
        source.to_str().unwrap(),
        "SELECT id FROM author WHERE id > -1 ORDER BY id",
    ]);
    assert!(
        output.status.success(),
        "a negative number in the SQL was read as an option: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run(&["sql-pipe", "--bogus", source.to_str().unwrap(), "SELECT 1"]);
    assert!(
        !output.status.success(),
        "an unknown option before the database should still fail"
    );
}

#[test]
fn sql_pipe_runs_every_statement_it_is_given() {
    let sandbox = Sandbox::new("pipe-many");
    let source = sandbox.path("shop.db");
    build_source(&source);

    let output = run_stdin(
        &["sql-pipe", source.to_str().unwrap()],
        "SELECT count(*) AS authors FROM author;\n\nSELECT count(*) AS orders FROM \"order\";\n",
    );
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("authors"), "{text}");
    assert!(text.contains("orders"), "{text}");
}

#[test]
fn sql_pipe_is_read_only_until_write_is_asked_for() {
    let sandbox = Sandbox::new("pipe-write");
    let source = sandbox.path("shop.db");
    build_source(&source);
    let path = source.to_str().unwrap();
    let before = fingerprint(&source);

    let refused = run(&["sql-pipe", path, "INSERT INTO author(name) VALUES ('Kim')"]);
    assert!(!refused.status.success());
    assert_eq!(
        before,
        fingerprint(&source),
        "sql-pipe wrote to the database without --write"
    );

    let allowed = run(&[
        "sql-pipe",
        "--write",
        path,
        "INSERT INTO author(name) VALUES ('Kim')",
    ]);
    assert!(
        allowed.status.success(),
        "{}",
        String::from_utf8_lossy(&allowed.stderr)
    );
    assert_eq!(
        scalar(&source, "SELECT CAST(count(*) AS TEXT) FROM author"),
        "4"
    );
}

#[test]
fn sql_pipe_stops_and_fails_when_a_statement_is_wrong() {
    let sandbox = Sandbox::new("pipe-error");
    let source = sandbox.path("shop.db");
    build_source(&source);
    let path = source.to_str().unwrap();

    let broken = run(&["sql-pipe", path, "SELECT nope FROM author"]);
    assert!(
        !broken.status.success(),
        "a failing query must exit non-zero so a pipeline notices"
    );
    assert!(String::from_utf8_lossy(&broken.stderr).contains("no such column"));

    let unterminated = run_stdin(&["sql-pipe", path], "SELECT 'oops");
    assert!(!unterminated.status.success());
    assert!(
        String::from_utf8_lossy(&unterminated.stderr).contains("unterminated"),
        "a half written statement must be reported, not quietly dropped"
    );

    let nothing = run_stdin(&["sql-pipe", path], "");
    assert!(!nothing.status.success());
    assert!(String::from_utf8_lossy(&nothing.stderr).contains("needs a statement"));
}

#[test]
fn sql_pipe_answers_the_shell_commands_too() {
    let sandbox = Sandbox::new("pipe-commands");
    let source = sandbox.path("shop.db");
    build_source(&source);
    let path = source.to_str().unwrap();

    let listed = run(&["sql-pipe", path, "tables;"]);
    assert!(
        listed.status.success(),
        "sql-pipe handed tables; to SQLite as SQL: {}",
        String::from_utf8_lossy(&listed.stderr)
    );
    let text = String::from_utf8_lossy(&listed.stdout);
    assert!(text.contains("author"), "{text}");

    let described = run(&["sql-pipe", path, "desc table \"order\";"]);
    assert!(described.status.success());
    assert!(
        String::from_utf8_lossy(&described.stdout).contains("author_id"),
        "desc table did not list the columns"
    );

    let missing = run(&["sql-pipe", path, "desc table nope;"]);
    assert!(
        !missing.status.success(),
        "a command that failed must exit non-zero so a pipeline notices"
    );
}

#[test]
fn several_statements_on_one_line_each_run() {
    let sandbox = Sandbox::new("pipe-one-line");
    let source = sandbox.path("shop.db");
    build_source(&source);

    let output = run(&[
        "sql-pipe",
        source.to_str().unwrap(),
        "SELECT count(*) AS authors FROM author; SELECT 'a;b' AS quoted;",
    ]);
    assert!(
        output.status.success(),
        "statements are split by SQL, not by line: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("authors"), "{text}");
    assert!(
        text.contains("a;b"),
        "a semicolon inside a string is not a statement boundary: {text}"
    );
}

#[test]
fn a_failing_statement_is_reported_once() {
    let sandbox = Sandbox::new("pipe-message");
    let source = sandbox.path("shop.db");
    build_source(&source);

    let broken = run(&["sql-pipe", source.to_str().unwrap(), "SELECT nope;"]);
    let message = String::from_utf8_lossy(&broken.stderr);
    assert_eq!(
        message.matches("SELECT nope").count(),
        1,
        "the statement is printed twice: {message}"
    );
}
