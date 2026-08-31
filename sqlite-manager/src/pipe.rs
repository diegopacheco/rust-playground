use std::io::{IsTerminal, Read};
use std::path::Path;

use rusqlite::Connection;

use crate::cli::Source;
use crate::command;
use crate::dumpfiles::DumpFiles;
use crate::error::DumpError;
use crate::progress::Progress;
use crate::query;
use crate::script;
use crate::workspace::Workspace;

pub fn run(source: &Source, statement: Option<&str>, writable: bool) -> Result<(), DumpError> {
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

    let session = match source {
        Source::Database(path) => Session {
            connection: query::open(path, writable)?,
            _scratch: None,
        },
        Source::Dump(path) => rebuild(path)?,
    };

    for statement in &statements {
        if let Some(outcome) = command::handle(&session.connection, statement) {
            outcome?;
            continue;
        }
        let outcome = query::run(&session.connection, statement)
            .map_err(|failure| DumpError::Query(statement.clone(), failure))?;
        query::print(&outcome);
    }
    Ok(())
}

struct Session {
    connection: Connection,
    _scratch: Option<Workspace>,
}

fn rebuild(directory: &Path) -> Result<Session, DumpError> {
    let files = DumpFiles::locate(directory)?;
    let scratch = Workspace::create()?;

    let progress = Progress::new();
    progress.note(&format!("📥 loading  {}", directory.display()));

    let connection = Connection::open(scratch.database())?;
    script::load(&connection, &files.schema, "schema", &progress)?;
    script::load(&connection, &files.data, "data", &progress)?;

    Ok(Session {
        connection,
        _scratch: Some(scratch),
    })
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
