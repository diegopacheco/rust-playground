use std::borrow::Cow;
use std::cell::RefCell;
use std::path::Path;

use rusqlite::Connection;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Editor, Helper};

use crate::command;
use crate::error::DumpError;
use crate::highlight::{self, BOLD, DIM, RED, RESET};
use crate::query;
use crate::script;

pub fn run(database: &Path, writable: bool, safe: bool) -> Result<(), DumpError> {
    let path = std::fs::canonicalize(database).map_err(|e| DumpError::io(database, e))?;
    let connection = query::open(&path, writable)?;

    let mode = match (writable, safe) {
        (true, _) => "✏️  read write",
        (false, true) => "🔒 safe, read only",
        (false, false) => "👀 read only",
    };
    println!(
        "🐚 {BOLD}sqlite-manager sql{RESET}  {}  ({mode})",
        path.display()
    );
    println!("{DIM}help; lists the shell commands, quit; leaves{RESET}\n");

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
                if pending.is_empty()
                    && let Some(outcome) = command::handle(&connection, line.trim())
                {
                    if let Err(failure) = outcome {
                        println!("{RED}❌ error{RESET} {failure}\n");
                    }
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
                query::report(&connection, pending.trim());
                refresh(&editor, &connection);
                pending.clear();
            }
            Err(ReadlineError::Interrupted) => {
                pending.clear();
                println!("{DIM}🚫 cancelled{RESET}");
            }
            Err(ReadlineError::Eof) => break,
            Err(failure) => return Err(DumpError::Shell(failure)),
        }
    }

    let _ = editor.save_history(&history);
    Ok(())
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
        if input.is_empty() || command::is_command(input) || script::is_complete(context.input()) {
            return Ok(ValidationResult::Valid(None));
        }
        Ok(ValidationResult::Incomplete)
    }
}
