use std::io::{IsTerminal, Read};
use std::path::Path;

use crate::command;
use crate::error::DumpError;
use crate::query;
use crate::script;

pub fn run(database: &Path, statement: Option<&str>, writable: bool) -> Result<(), DumpError> {
    let text = match statement {
        Some(given) => given.to_string(),
        None => read_stdin()?,
    };
    let statements = script::split(&text)?;
    if statements.is_empty() {
        return Err(DumpError::Usage(
            "sql-pipe needs a statement, as arguments or on stdin".to_string(),
        ));
    }

    let connection = query::open(database, writable)?;
    for statement in &statements {
        if let Some(outcome) = command::handle(&connection, statement) {
            outcome?;
            continue;
        }
        let outcome = query::run(&connection, statement)
            .map_err(|failure| DumpError::Query(statement.clone(), failure))?;
        query::print(&outcome);
    }
    Ok(())
}

fn read_stdin() -> Result<String, DumpError> {
    let mut input = std::io::stdin();
    if input.is_terminal() {
        return Err(DumpError::Usage(
            "sql-pipe needs a statement, as arguments or on stdin".to_string(),
        ));
    }
    let mut text = String::new();
    input
        .read_to_string(&mut text)
        .map_err(|e| DumpError::io(Path::new("stdin"), e))?;
    Ok(text)
}
