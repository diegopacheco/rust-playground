use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rusqlite::Connection;

use crate::error::DumpError;
use crate::progress::Progress;

pub struct Outcome {
    pub statements: u64,
    pub creates: u64,
    pub inserts: u64,
}

pub fn load(
    connection: &Connection,
    path: &Path,
    label: &str,
    progress: &Progress,
) -> Result<Outcome, DumpError> {
    let total = std::fs::metadata(path)
        .map_err(|e| DumpError::io(path, e))?
        .len();
    let file = File::open(path).map_err(|e| DumpError::io(path, e))?;
    let bar = progress.bytes(label, total);

    let mut pending = String::new();
    let mut opened_at = 0usize;
    let mut outcome = Outcome {
        statements: 0,
        creates: 0,
        inserts: 0,
    };

    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|e| DumpError::io(path, e))?;
        bar.inc(line.len() as u64 + 1);

        if pending.is_empty() {
            opened_at = index + 1;
            if line.trim().is_empty() {
                continue;
            }
        }
        pending.push_str(&line);
        pending.push('\n');

        if !is_complete(&pending) {
            continue;
        }
        connection
            .execute_batch(&pending)
            .map_err(|failure| DumpError::Statement {
                path: path.display().to_string(),
                line: opened_at,
                message: failure.to_string(),
            })?;
        match leading_word(&pending).as_str() {
            "insert" => outcome.inserts += 1,
            "create" => outcome.creates += 1,
            _ => {}
        }
        outcome.statements += 1;
        pending.clear();
    }

    if !pending.trim().is_empty() {
        return Err(DumpError::Statement {
            path: path.display().to_string(),
            line: opened_at,
            message: "unterminated statement at end of file".to_string(),
        });
    }

    bar.set_message("done");
    bar.finish();
    Ok(outcome)
}

fn leading_word(statement: &str) -> String {
    statement
        .trim_start()
        .split(|c: char| !c.is_ascii_alphabetic())
        .next()
        .unwrap_or_default()
        .to_lowercase()
}

pub fn split(text: &str) -> Result<Vec<String>, DumpError> {
    let mut statements = Vec::new();
    let mut pending = String::new();

    for character in text.chars() {
        if pending.is_empty() && character.is_whitespace() {
            continue;
        }
        pending.push(character);

        if character != ';' || !is_complete(&pending) {
            continue;
        }
        statements.push(pending.trim().to_string());
        pending.clear();
    }

    let trailing = pending.trim();
    if trailing.is_empty() {
        return Ok(statements);
    }
    if is_complete(&format!("{trailing};")) {
        statements.push(trailing.to_string());
        return Ok(statements);
    }
    Err(DumpError::Usage(format!(
        "unterminated SQL statement: {trailing}"
    )))
}

pub fn is_complete(text: &str) -> bool {
    let Ok(probe) = CString::new(text) else {
        return false;
    };
    unsafe { rusqlite::ffi::sqlite3_complete(probe.as_ptr()) != 0 }
}
