use std::path::Path;

use rusqlite::Connection;

use crate::dumpfiles::DumpFiles;
use crate::error::DumpError;
use crate::progress::Progress;
use crate::report;
use crate::script;
use crate::workspace::Workspace;

pub fn run(source: &Path) -> Result<(), DumpError> {
    let files = DumpFiles::locate(source)?;
    let workspace = Workspace::create()?;

    let progress = Progress::new();
    progress.note(&format!("🔎 checking  {}", source.display()));

    let connection = Connection::open(workspace.database())?;
    let schema = script::load(&connection, &files.schema, "schema", &progress)?;
    let data = script::load(&connection, &files.data, "data", &progress)?;

    let mut problems = Vec::new();
    let integrity: String = connection.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
    if integrity != "ok" {
        problems.push(format!("integrity check said {integrity}"));
    }

    let orphans = foreign_key_orphans(&connection)?;
    if orphans > 0 {
        problems.push(report::count(orphans, "broken reference"));
    }

    progress.note("");
    progress.note(&format!(
        "🧱 schema.sql  {:>9}  {}",
        report::size(&files.schema),
        report::count(schema.creates as usize, "object")
    ));
    progress.note(&format!(
        "🧾 data.sql    {:>9}  {}",
        report::size(&files.data),
        report::count(data.inserts as usize, "row")
    ));

    if problems.is_empty() {
        progress.note("");
        progress.note("✅ the dump rebuilds cleanly and is sound");
        return Ok(());
    }
    Err(DumpError::Unsound(problems))
}

fn foreign_key_orphans(connection: &Connection) -> Result<usize, DumpError> {
    let mut statement = connection.prepare("PRAGMA foreign_key_check")?;
    let mut rows = statement.query([])?;
    let mut orphans = 0;
    while rows.next()?.is_some() {
        orphans += 1;
    }
    Ok(orphans)
}
