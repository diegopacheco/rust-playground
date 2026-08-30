use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::db::{Header, SchemaObject};
use crate::error::DumpError;
use crate::progress::Progress;

pub fn write(
    objects: &[SchemaObject],
    header: &Header,
    path: &Path,
    progress: &Progress,
) -> Result<(), DumpError> {
    let file = File::create(path).map_err(|e| DumpError::io(path, e))?;
    let mut out = BufWriter::new(file);
    let bar = progress.stage("schema", objects.len() as u64);

    write_line(&mut out, path, "PRAGMA foreign_keys=OFF;")?;
    write_line(&mut out, path, "BEGIN TRANSACTION;")?;

    if header.application_id != 0 {
        let statement = format!("PRAGMA application_id={};", header.application_id);
        write_line(&mut out, path, &statement)?;
    }
    if header.user_version != 0 {
        let statement = format!("PRAGMA user_version={};", header.user_version);
        write_line(&mut out, path, &statement)?;
    }

    for object in objects {
        bar.set_message(format!("{} {}", object.kind, object.name));
        write_line(&mut out, path, &format!("{};", object.statement.trim_end()))?;
        bar.inc(1);
    }

    write_line(&mut out, path, "COMMIT;")?;
    out.flush().map_err(|e| DumpError::io(path, e))?;
    bar.set_message("done");
    bar.finish();
    Ok(())
}

fn write_line(out: &mut impl Write, path: &Path, line: &str) -> Result<(), DumpError> {
    writeln!(out, "{line}").map_err(|e| DumpError::io(path, e))
}
