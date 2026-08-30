use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::db::SchemaObject;
use crate::error::DumpError;
use crate::progress::Progress;

pub fn write(objects: &[SchemaObject], path: &Path, progress: &Progress) -> Result<(), DumpError> {
    let file = File::create(path).map_err(|e| DumpError::io(path, e))?;
    let mut out = BufWriter::new(file);
    let bar = progress.stage("schema", objects.len() as u64);

    write_line(&mut out, path, "PRAGMA foreign_keys=OFF;")?;
    write_line(&mut out, path, "BEGIN TRANSACTION;")?;

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
