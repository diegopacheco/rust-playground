<img src="images/logo.svg" alt="sqlite-manager" width="420">

A Rust CLI that backs up, restores, verifies, queries and documents SQLite databases. It reads a `.db` file and writes a `schema.sql`, a `data.sql` and a `manifest.txt` that together rebuild the database exactly, restores that dump into a fresh database, tells you whether a dump is still sound, opens a SQL shell over a database, runs one-off SQL straight from the command line or a pipe, and prints a data dictionary of everything in it.

The source database is never written to, locked, or moved.

## How it Works

`dump` opens the database read only and immutable, so SQLite takes no locks and creates no `-wal` or `-shm` files next to it. When a hot write-ahead log is found, the database and its log are copied to a private scratch directory first, the log is merged there, and the dump is taken from that copy — so a live application keeps running and its files stay byte for byte identical.

Schema objects come out of `sqlite_master` in dependency order. Row data is streamed a row at a time and written as `INSERT` statements with explicit column lists, so generated columns are skipped and FTS shadow tables never leak in. Triggers are dropped at the top of `data.sql` and recreated at the bottom, so replaying the data cannot fire them a second time and corrupt the restore.

The database header travels with the schema. `user_version` and `application_id` are written into `schema.sql` when a database has set them, so a restore is stamped the same way the original was and a migration counter cannot silently reset to zero. A database that set neither carries neither, so ordinary dumps stay clean.

`dump` also writes a `manifest.txt` recording how many rows the source held in every table and how many objects its schema defined. Replaying a dump proves only that the SQL parses; the manifest is what proves nothing went missing between the two.

`import` and `check` replay a dump one statement at a time, using SQLite's own parser to decide where each statement ends. `check` replays into a throwaway database, then runs `integrity_check` and `foreign_key_check`, compares every table against the manifest, and exits non-zero if anything is wrong. A dump written before manifests existed still checks; it just says the counts were not verified.

## Architecture

![architecture](images/architecture.png)

## Features

- **`dump`** — writes `schema.sql`, `data.sql` and `manifest.txt` from a database, or from the only database it finds in a directory. Split files let you review the schema without wading through millions of inserts.
- **`import`** — rebuilds a database from a dump directory. Refuses to overwrite an existing file unless `--force` is passed, so a restore can never silently destroy a database.
- **`check`** — replays a dump into a scratch database, runs integrity and foreign key checks, and compares every table against `manifest.txt`. A backup you have never restored is not a backup, and a backup that restores one row short is worse than one that fails outright.
- **`sql`** — an interactive shell with SQL syntax highlighting, line numbers, tab completion over keywords, tables and columns, multi-line statements and box-drawn result tables. Read only unless `--write` is passed.
- **`sql-pipe`** — the same engine without the shell: one statement, or a whole `.sql` file on stdin, printed and gone. Reads a database file or a dump directory, which it rebuilds in a scratch database and throws away. Read only unless `--write` is passed, and a failing statement exits non-zero so a pipeline notices.
- **`tables;`, `desc table NAME;`, `help;`** — commands that read like SQL rather than dot commands, so the shell answers "what is in here" without leaving it. `sql-pipe` understands the same ones.
- **`dict`** — the data dictionary: every table with its columns, types, constraints, indexes, views, triggers and the full relation graph. Answers "what is in this database" in one screen.
- **`--safe`** — read only mode. It refuses `import` and `sql --write`, so no database file is ever opened for writing. It may be given anywhere on the line.
- **Never touches the source** — read only and immutable by default, private copy when a log is hot, and a guard that refuses to write output over the database being read.
- **Fine grained progress** — a bar per stage and a nested bar per table with live row counts, so a long dump tells you exactly which table it is on.
- **Faithful values** — blobs as hex, invalid UTF-8 as a cast blob, full float precision, doubled quotes in text and identifiers, `sqlite_sequence` restored so `AUTOINCREMENT` continues where it left off, and `user_version` and `application_id` carried across.

## Stack

- **Rust 2024 edition, rustc 1.98.0** — one static binary, no runtime to install alongside it.
- **rusqlite (bundled SQLite)** — SQLite is compiled into the binary, so there is no system SQLite version to match.
- **indicatif** — the multi-bar progress rendering; it hides itself when output is piped.
- **rustyline** — line editing for the `sql` shell, driving the highlighter and completer.
- **Bun + Vite + React** (in `sample/`) — a small notes app that writes to SQLite, used to exercise the CLI against a live, actively written database.

Argument parsing is hand written rather than pulled from a crate; the surface is small enough that a dependency would cost more than it saves.

## Commands

```
sqlite-manager [--safe] <COMMAND> [ARGS]

dump   [PATH] [-o DIR]     read a database and write schema.sql, data.sql, manifest.txt
import <DIR> [-o FILE]     rebuild a database from a dump directory (--force to overwrite)
check  <DIR>               rebuild a dump in a scratch database, verify it against manifest.txt
sql    [PATH] [--write]    SQL shell with highlighting, line numbers, completion, tables
sql-pipe <PATH> [SQL]      run SQL once and print the result, no shell (--write before PATH)
                           PATH is a database file or a dump directory
dict   [PATH]              print the data dictionary
help                       print the help
version                    print the version

--safe                     read only, refuses import, sql --write and sql-pipe --write
```

`PATH` may be a database file or a directory to search; it defaults to the current directory. Running with no arguments prints the help.

### Safe mode

`--safe` is the guarantee that a session cannot write a database, whoever types the rest of the line:

```bash
$ sqlite-manager --safe import backup -o restored.db
❌ sqlite-manager: 🔒 --safe is on, import builds a database, so it cannot run

$ sqlite-manager --safe sql sample/dbs/store.db --write
❌ sqlite-manager: 🔒 --safe is on, sql --write opens the database for writing, so it cannot run
```

```bash
$ sqlite-manager --safe sql-pipe --write sample/dbs/store.db "delete from orders"
❌ sqlite-manager: 🔒 --safe is on, sql-pipe --write opens the database for writing, so it cannot run
```

`dump`, `check`, `dict` and a plain `sql-pipe` never write a database, so they run unchanged under `--safe`.

## sql-pipe

`sql` is for exploring, `sql-pipe` is for scripting. It opens the database, runs what it is given, prints the result and exits.

The statement is the rest of the line:

```bash
$ sqlite-manager sql-pipe sample/dbs/store.db "select id, name from customers limit 3"
┌────┬─────────────┐
│ id │ name        │
├────┼─────────────┤
│ 1  │ Ana Ribeiro │
│ 2  │ Lars Holm   │
│ 3  │ Mei Tanaka  │
└────┴─────────────┘
3 rows
```

Quoting is optional; every word after the database path is joined back together, so this is the same query:

```bash
sqlite-manager sql-pipe sample/dbs/store.db select id, name from customers limit 3
```

Nothing after the database path is read as an option, so `where id > -1` is SQL rather than a bad flag. `--write` and `--safe` go before the path. The one thing the shell does behind your back is glob `*`, so quote a `select *` unless the current directory is empty.

When the rest of the line is empty the statement is read from stdin, which is where the name comes from:

```bash
echo "select count(*) as products from products" | sqlite-manager sql-pipe sample/dbs/store.db
sqlite-manager sql-pipe sample/dbs/store.db < report.sql
```

A file may hold several statements, on one line or many; they are split with SQLite's own parser and each result is printed in turn. `tables;`, `desc table NAME;` and `help;` work here as well as in the shell. A statement that fails prints to stderr and exits non-zero, so `set -e` and `&&` behave, and the statements after it do not run.

The path may also be a dump directory. `schema.sql` and `data.sql` are rebuilt into a scratch database under the temp directory, the SQL runs there, and the scratch database is deleted on exit, so a backup answers questions without being imported first:

```bash
$ sqlite-manager sql-pipe ./backup "select count(*) from episodes"
```

Writes need `--write`, before the path:

```bash
$ sqlite-manager sql-pipe sample/dbs/store.db "delete from orders"
❌ sqlite-manager: delete from orders: attempt to write a readonly database

$ sqlite-manager sql-pipe --write sample/dbs/store.db "delete from orders where id = 6"
1 row changed
```

A dump directory refuses `--write`, because the write would land in the scratch database and be thrown away on exit. Use `import` to turn the dump into a database you can keep, then write to that.

## Sample databases

`sample/dbs/` holds three ready-made SQLite databases to try the commands against:

| Database | Contents |
| --- | --- |
| `store.db` | customers, products, orders and order items, with two indexes and an `open_orders` view |
| `library.db` | authors, books, members and loans, with a partial index and an `open_loans` view |
| `metrics.db` | four hosts and 3,000 metric samples, the one large enough for the progress bars to show |

```bash
sqlite-manager dict sample/dbs/store.db
sqlite-manager --safe sql sample/dbs/library.db
sqlite-manager dump sample/dbs/metrics.db -o backup
sqlite-manager sql-pipe sample/dbs/metrics.db "select metric, count(*) from samples group by metric"
```

Each one is built from the `.sql` file beside it, so `sample/dbs/build.sh` recreates all three from scratch.

### Notes app API

The app in `sample/` is a plain REST service on `http://localhost:7777`.

| Method | Path | Body | Returns |
| --- | --- | --- | --- |
| `GET` | `/api/authors` | | authors |
| `GET` | `/api/notes` | | notes with author and tags |
| `GET` | `/api/stats` | | counts of authors, notes, tags |
| `POST` | `/api/notes` | `{authorId, title, body, tags}` | the new note list, `201` |
| `POST` | `/api/seed` | `{rows}` | `{inserted}` |
| `PATCH` | `/api/notes/:id` | | the note list, pin toggled |
| `DELETE` | `/api/notes/:id` | | the note list |

## Key data structures and design decisions

- **`Source`** wraps the read-only connection and owns an optional `Workspace`. Field order matters: the connection is declared before the workspace so it is dropped first, and the scratch directory is only removed once SQLite has let go of it.
- **`Workspace`** is a temp directory that deletes itself on `Drop`. Both the hot-log copy and `check`'s scratch database use it, so no cleanup path can be forgotten.
- **The manifest is written from the source, not from what was dumped.** Counting the rows the dump wrote would only confirm the dump agrees with itself; counting the source is what turns `check` into a real comparison.
- **A missing `manifest.txt` is not an error.** `check` reports that counts were not verified and carries on, so dumps taken by an older build keep working.
- **Statement splitting uses `sqlite3_complete`**, not a `;` scan. A `CREATE TRIGGER ... BEGIN ... END;` body and a multi-line string literal both contain semicolons that do not end the statement.
- **Triggers are guarded in `data.sql`**, not reordered in `schema.sql`. `schema.sql` stays a complete, readable description of the schema, and the data file stays safe to replay on its own.
- **`PRAGMA table_list` classifies tables** so FTS shadow tables are excluded from both the schema and the data, while the virtual table itself is dumped and rebuilt normally.
- **Column lists are explicit** in every `INSERT`. Generated columns cannot be inserted into, and an explicit list keeps a dump valid if the schema later gains a column.
- **`sql-pipe` stops parsing options at the database path.** SQL is full of tokens that look like flags — `-1`, `--` comments — and no escape hatch is needed if the parser simply knows the argument list is over.
- **Errors carry a file and line**, so a bad dump reports `data.sql:4: near "INSRT": syntax error` rather than a bare failure.

## How to run

```bash
./build.sh          # fmt check, clippy with warnings denied, release build
./test.sh           # 17 unit tests and 17 end-to-end tests
./install.sh        # builds and installs to /usr/local/bin (BIN_DIR= to override)
./uninstall.sh      # removes it again
./run.sh dump ./my.db -o backup
```

The end-to-end tests drive the real binary: they assert the source file is byte identical after a dump, that a hot write-ahead log is included without the live files changing, that a trigger does not fire twice during a restore, and that `check` rejects syntax errors, truncated files and broken references.

### What it looks like

```
$ sqlite-manager dump sample/sample.db -o backup
📦 source  /path/to/sample/sample.db
📁 target  backup
🧊 note    a write-ahead log was merged from a private copy, the source was left alone

🧱 schema.sql     1018 B  7 objects
🧾 data.sql     720.8 KB  5 tables, 5,018 rows

$ sqlite-manager check backup
🔎 checking  backup

🧱 schema.sql     1018 B  7 objects
🧾 data.sql     720.8 KB  5,018 rows

✅ the dump rebuilds cleanly and is sound

$ sqlite-manager import backup -o restored.db
📁 source  backup
💾 target  restored.db

✅ restored  528.0 KB  7 objects, 5,018 rows
```

While a dump runs, a bar tracks the stage and a nested bar tracks the table:

```
⠉ data     [━━━━━━━━━╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌]     1/4     posts
   posts                    [━━━━━━━━━━━━━━━━━━━━━━━━╌╌╌╌╌]   101,957/120,000   rows
```

`dict` prints the schema and the relation graph:

```
$ sqlite-manager dict sample/sample.db
📖 database  /path/to/sample/sample.db
4 tables, 1 view, 2 indexes, 0 triggers

🧱 TABLE notes  (5,002 rows)
  id          INTEGER  primary key
  author_id   INTEGER  not null
  title       TEXT     not null
  body        TEXT     not null, default ''
  pinned      INTEGER  not null, default 0
  created_at  TEXT     not null, default datetime('now')
  index idx_notes_created (created_at)
  index idx_notes_author (author_id)
  references author_id -> authors.id

RELATIONS
  note_tags.tag_id -> tags.id  on delete cascade
  note_tags.note_id -> notes.id  on delete cascade
  notes.author_id -> authors.id  on delete cascade
```

`sql` opens the shell. Keywords colour as you type, the prompt counts lines, Tab completes, and results come back as tables. Alongside SQL it takes three commands, written with a semicolon like everything else:

| Command | Does |
| --- | --- |
| `tables;` | list every table and view |
| `desc table NAME;` | describe one table: columns, indexes, foreign keys and the full `CREATE` statement |
| `help;` | list the commands |

Each also works dot-prefixed and without the semicolon, so `tables;`, `.tables` and `tables` are the same thing. `desc` takes a view as happily as a table, and quoted names (`desc table "order";`) are unwrapped before the lookup.

```
   1 sql> desc table orders;
🧱 TABLE orders  (6 rows)
  id           INTEGER  primary key
  customer_id  INTEGER  not null
  placed_at    TEXT     not null
  status       TEXT     not null
  index idx_orders_status (status)
  index idx_orders_customer (customer_id)
  references customer_id -> customers.id

CREATE TABLE orders (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  customer_id INTEGER NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
  placed_at   TEXT NOT NULL,
  status      TEXT NOT NULL
)

CREATE INDEX idx_orders_customer ON orders(customer_id)
```

Ordinary queries come back as tables:

```
🐚 sqlite-manager sql sample/sample.db  (👀 read only)
help; lists the shell commands, quit; leaves

   1 sql> SELECT a.name, count(*) AS notes
   2 ...> FROM notes n JOIN authors a ON a.id = n.author_id
   3 ...> GROUP BY a.name ORDER BY notes DESC;
┌───────┬───────┐
│ name  │ notes │
├───────┼───────┤
│ Ada   │ 1668  │
│ Grace │ 1667  │
│ Alan  │ 1667  │
└───────┴───────┘
3 rows
```

## The notes app

`sample/` holds a small React app served by Vite, with a Bun API that writes to `sample/sample.db` in WAL mode. It exists so the CLI can be exercised against a database that a real process is holding open and writing to.

```bash
cd sample
./start.sh     # installs if needed, starts the API on :7777 and the UI on :5173
./stop.sh      # stops both
```

Then, while it is still running:

```bash
sqlite-manager dict sample/sample.db
sqlite-manager dump sample/sample.db -o backup
sqlite-manager check backup
```

### Screens

The app opens empty, with the author picker, the note form and the seed button. The header reads the counts straight out of SQLite, so it shows `0 notes` against the freshly created `sample/sample.db`.

![empty](printscreens/01-empty.png)

After two notes are added, each row shows its author, timestamp and tags, all resolved through the `note_tags` join table. The first note has been pinned, which is the `PATCH` route flipping a column and the list re-sorting on `pinned DESC`. This is the state the `dict` and `dump` output above was taken from.

![notes](printscreens/02-notes.png)

`seed 5,000 rows` inserts five thousand notes in one transaction, which pushes most of the data into the write-ahead log rather than the main database file. That is the interesting case: `sample.db` itself stays tiny while `sample.db-wal` grows, and dumping it is what exercises the private-copy path.

![seeded](printscreens/03-seeded.png)
