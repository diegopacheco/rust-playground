use std::fs;
use std::path::Path;

use rusqlite::Connection;

use crate::dumpfiles::DumpFiles;
use crate::error::DumpError;
use crate::progress::Progress;
use crate::report;
use crate::script;
use crate::workspace::sibling;

pub fn run(source: &Path, target: &Path, force: bool) -> Result<(), DumpError> {
    let files = DumpFiles::locate(source)?;

    if target.exists() {
        if !force {
            return Err(DumpError::TargetExists(target.to_path_buf()));
        }
        for suffix in ["", "-wal", "-shm"] {
            let stale = sibling(target, suffix);
            if stale.exists() {
                fs::remove_file(&stale).map_err(|e| DumpError::io(&stale, e))?;
            }
        }
    }

    let progress = Progress::new();
    progress.note(&format!("📁 source  {}", source.display()));
    progress.note(&format!("💾 target  {}", target.display()));

    let connection = Connection::open(target)?;
    let schema = script::load(&connection, &files.schema, "schema", &progress)?;
    let data = script::load(&connection, &files.data, "data", &progress)?;
    drop(connection);

    progress.note("");
    progress.note(&format!(
        "✅ restored  {}  {}, {}",
        report::size(target),
        report::count(schema.creates as usize, "object"),
        report::count(data.inserts as usize, "row")
    ));
    Ok(())
}
