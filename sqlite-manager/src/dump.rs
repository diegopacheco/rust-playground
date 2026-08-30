use std::fs;
use std::path::{Path, PathBuf};

use crate::data;
use crate::db::{SchemaObject, Source};
use crate::error::DumpError;
use crate::progress::Progress;
use crate::report;
use crate::schema;

pub fn run(database: &Path, output: &Path) -> Result<(), DumpError> {
    let source = Source::open(database)?;
    fs::create_dir_all(output).map_err(|e| DumpError::io(output, e))?;

    let schema_path = output.join("schema.sql");
    let data_path = output.join("data.sql");
    guard_source(&source, &[&schema_path, &data_path])?;

    let progress = Progress::new();
    progress.note(&format!("📦 source  {}", source.path().display()));
    progress.note(&format!("📁 target  {}", output.display()));
    if source.merged_log() {
        progress.note(
            "🧊 note    a write-ahead log was merged from a private copy, \
             the source was left alone",
        );
    }

    let objects = source.schema_objects()?;
    schema::write(&objects, &schema_path, &progress)?;

    let triggers: Vec<&SchemaObject> = objects
        .iter()
        .filter(|object| object.kind == "trigger")
        .collect();
    let tables = source.tables()?;
    let rows = data::write(&source, &tables, &triggers, &data_path, &progress)?;

    progress.note("");
    progress.note(&format!(
        "🧱 schema.sql  {:>9}  {}",
        report::size(&schema_path),
        report::count(objects.len(), "object")
    ));
    progress.note(&format!(
        "🧾 data.sql    {:>9}  {}, {}",
        report::size(&data_path),
        report::count(tables.len(), "table"),
        report::count(rows as usize, "row")
    ));
    Ok(())
}

fn guard_source(source: &Source, targets: &[&PathBuf]) -> Result<(), DumpError> {
    for target in targets {
        if fs::canonicalize(target).is_ok_and(|path| path == source.path()) {
            return Err(DumpError::OutputCollision(source.path().to_path_buf()));
        }
    }
    Ok(())
}
