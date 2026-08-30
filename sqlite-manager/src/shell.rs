use std::borrow::Cow;
use std::cell::RefCell;
use std::path::Path;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags};
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Editor, Helper};

use crate::error::DumpError;
use crate::highlight::{self, BOLD, DIM, RED, RESET};
use crate::script;
use crate::table::{Grid, cell};

pub fn run(database: &Path, writable: bool) -> Result<(), DumpError> {
    let path = std::fs::canonicalize(database).map_err(|e| DumpError::io(database, e))?;
    let connection = open(&path, writable)?;

    let mode = if writable { "read write" } else { "read only" };
    println!(
        "{BOLD}sqlite-manager sql{RESET}  {}  ({mode})",
        path.display()
    );
    println!("{DIM}.help lists the shell commands, .quit leaves{RESET}\n");

    let helper = SqlHelper {
        vocabulary: RefCell::new(Vec::new()),
    };
    let mut editor: Editor<SqlHelper, DefaultHistory> = Editor::new()?;
    editor.set_helper(Some(helper));
    refresh(&editor, &connection);

    let history = std::env::temp_dir().join("sqlite-manager-history");
    let _ = editor.load_history(&history);

    let mut pending = String::new();
    let mut number = 1usize;

    loop {
        let marker = if pending.is_empty() { "sql>" } else { "...>" };
        let prompt = format!("{DIM}{number:>4}{RESET} {BOLD}{marker}{RESET} ");
        match editor.readline(&prompt) {
            Ok(line) => {
                if pending.is_empty() && handle_command(line.trim(), &connection)? {
                    let _ = editor.add_history_entry(line.trim());
                    number += 1;
                    continue;
                }
                if pending.is_empty() && line.trim().is_empty() {
                    continue;
                }
                pending.push_str(&line);
                pending.push('\n');
                number += 1;

                if !script::is_complete(&pending) {
                    continue;
                }
                let _ = editor.add_history_entry(pending.trim_end());
                execute(&connection, pending.trim());
                refresh(&editor, &connection);
                pending.clear();
            }
            Err(ReadlineError::Interrupted) => {
                pending.clear();
                println!("{DIM}cancelled{RESET}");
            }
            Err(ReadlineError::Eof) => break,
            Err(failure) => return Err(DumpError::Shell(failure)),
        }
    }

    let _ = editor.save_history(&history);
    Ok(())
}

fn open(path: &Path, writable: bool) -> Result<Connection, DumpError> {
    if writable {
        return Ok(Connection::open(path)?);
    }
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI;
    let uri = format!("file:{}?mode=ro", path.display());
    match Connection::open_with_flags(&uri, flags) {
        Ok(connection) => Ok(connection),
        Err(_) => Ok(Connection::open_with_flags(
            format!("{uri}&immutable=1"),
            flags,
        )?),
    }
}

fn handle_command(line: &str, connection: &Connection) -> Result<bool, DumpError> {
    if !line.starts_with('.') {
        return Ok(false);
    }
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap_or_default();
    let argument = parts.next();

    match command {
        ".quit" | ".exit" => std::process::exit(0),
        ".help" => println!("{}", HELP),
        ".tables" => execute(connection, TABLES),
        ".schema" => match argument {
            Some(name) => {
                let query = format!(
                    "SELECT sql FROM sqlite_master WHERE tbl_name = '{}'",
                    name.replace('\'', "''")
                );
                execute(connection, &query);
            }
            None => execute(connection, "SELECT type, name, sql FROM sqlite_master"),
        },
        other => println!("unknown shell command: {other}, try .help"),
    }
    Ok(true)
}

fn execute(connection: &Connection, statement: &str) {
    match query(connection, statement) {
        Ok(Outcome::Rows(grid)) => {
            if grid.len() == 0 {
                println!("{DIM}no rows{RESET}\n");
                return;
            }
            print!("{}", grid.render());
            println!("{DIM}{}{RESET}\n", crate::report::count(grid.len(), "row"));
        }
        Ok(Outcome::Changed(count)) => {
            println!(
                "{DIM}{}{RESET}\n",
                crate::report::count(count, "row changed")
            );
        }
        Err(failure) => println!("{RED}error{RESET} {failure}\n"),
    }
}

enum Outcome {
    Rows(Grid),
    Changed(usize),
}

fn query(connection: &Connection, statement: &str) -> Result<Outcome, rusqlite::Error> {
    let mut prepared = connection.prepare(statement)?;
    let headers: Vec<String> = prepared
        .column_names()
        .iter()
        .map(|n| n.to_string())
        .collect();

    if headers.is_empty() {
        let changed = prepared.execute([])?;
        return Ok(Outcome::Changed(changed));
    }

    let mut grid = Grid::new(headers.clone());
    let mut rows = prepared.query([])?;
    while let Some(row) = rows.next()? {
        let mut line = Vec::with_capacity(headers.len());
        for index in 0..headers.len() {
            line.push(cell(&show(row.get_ref(index)?)));
        }
        grid.push(line);
    }
    Ok(Outcome::Rows(grid))
}

fn show(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => "NULL".to_string(),
        ValueRef::Integer(number) => number.to_string(),
        ValueRef::Real(number) => format!("{number:?}"),
        ValueRef::Text(bytes) => String::from_utf8_lossy(bytes).to_string(),
        ValueRef::Blob(bytes) => format!("<{} bytes>", bytes.len()),
    }
}

fn refresh(editor: &Editor<SqlHelper, DefaultHistory>, connection: &Connection) {
    let Some(helper) = editor.helper() else {
        return;
    };
    let mut words: Vec<String> = highlight::KEYWORDS.iter().map(|k| k.to_string()).collect();

    if let Ok(mut statement) = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type IN ('table', 'view') \
         UNION SELECT name FROM pragma_table_list",
    ) && let Ok(names) = statement.query_map([], |row| row.get::<_, String>(0))
    {
        for name in names.flatten() {
            if let Ok(mut columns) = connection.prepare(&format!(
                "PRAGMA table_info({})",
                crate::sql::quote_ident(&name)
            )) && let Ok(found) = columns.query_map([], |row| row.get::<_, String>(1))
            {
                words.extend(found.flatten());
            }
            words.push(name);
        }
    }
    words.sort();
    words.dedup();
    *helper.vocabulary.borrow_mut() = words;
}

const TABLES: &str = "SELECT name, type FROM pragma_table_list \
                      WHERE schema = 'main' AND type IN ('table', 'view', 'virtual') \
                      AND name NOT LIKE 'sqlite\\_%' ESCAPE '\\' \
                      ORDER BY type, name";

const HELP: &str = "  .tables            list tables and views
  .schema [TABLE]    show the SQL that defines the schema
  .help              this list
  .quit              leave the shell

  Statements run when they are complete, so a statement can span
  several lines. Tab completes keywords, tables and columns.
  Ctrl-C clears the current statement, Ctrl-D leaves.";

struct SqlHelper {
    vocabulary: RefCell<Vec<String>>,
}

impl Helper for SqlHelper {}

impl Completer for SqlHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        position: usize,
        _context: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line[..position]
            .rfind(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .map(|index| index + 1)
            .unwrap_or(0);
        let prefix = &line[start..position];
        if prefix.is_empty() {
            return Ok((start, Vec::new()));
        }

        let lowered = prefix.to_lowercase();
        let matches = self
            .vocabulary
            .borrow()
            .iter()
            .filter(|word| word.to_lowercase().starts_with(&lowered))
            .map(|word| Pair {
                display: word.clone(),
                replacement: word.clone(),
            })
            .collect();
        Ok((start, matches))
    }
}

impl Hinter for SqlHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _position: usize, _context: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for SqlHelper {
    fn highlight<'l>(&self, line: &'l str, _position: usize) -> Cow<'l, str> {
        Cow::Owned(highlight::paint(line))
    }

    fn highlight_char(&self, _line: &str, _position: usize, _kind: CmdKind) -> bool {
        true
    }
}

impl Validator for SqlHelper {
    fn validate(&self, context: &mut ValidationContext<'_>) -> rustyline::Result<ValidationResult> {
        let input = context.input().trim();
        if input.is_empty() || input.starts_with('.') || script::is_complete(context.input()) {
            return Ok(ValidationResult::Valid(None));
        }
        Ok(ValidationResult::Incomplete)
    }
}
