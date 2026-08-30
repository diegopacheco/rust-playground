use std::path::Path;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags};

use crate::error::DumpError;
use crate::highlight::{DIM, RED, RESET};
use crate::table::{Grid, cell};

pub enum Outcome {
    Rows(Grid),
    Changed(usize),
}

pub fn open(path: &Path, writable: bool) -> Result<Connection, DumpError> {
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

pub fn run(connection: &Connection, statement: &str) -> Result<Outcome, rusqlite::Error> {
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

pub fn print(outcome: &Outcome) {
    match outcome {
        Outcome::Rows(grid) => {
            if grid.len() == 0 {
                println!("{DIM}🫙 no rows{RESET}\n");
                return;
            }
            print!("{}", grid.render());
            println!("{DIM}{}{RESET}\n", crate::report::count(grid.len(), "row"));
        }
        Outcome::Changed(count) => {
            println!(
                "{DIM}{}{RESET}\n",
                crate::report::count(*count, "row changed")
            );
        }
    }
}

pub fn report(connection: &Connection, statement: &str) {
    match run(connection, statement) {
        Ok(outcome) => print(&outcome),
        Err(failure) => println!("{RED}❌ error{RESET} {failure}\n"),
    }
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
