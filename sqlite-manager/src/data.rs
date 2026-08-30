use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::db::{SEQUENCE, SchemaObject, Source, Table};
use crate::error::DumpError;
use crate::progress::Progress;
use crate::sql;

pub fn write(
    source: &Source,
    tables: &[Table],
    triggers: &[&SchemaObject],
    path: &Path,
    progress: &Progress,
) -> Result<u64, DumpError> {
    let file = File::create(path).map_err(|e| DumpError::io(path, e))?;
    let mut out = BufWriter::new(file);
    let stage = progress.stage("data", tables.len() as u64);
    let mut written = 0u64;

    write_line(&mut out, path, "PRAGMA foreign_keys=OFF;")?;
    write_line(&mut out, path, "BEGIN TRANSACTION;")?;

    for trigger in triggers {
        let statement = format!(
            "DROP TRIGGER IF EXISTS {};",
            sql::quote_ident(&trigger.name)
        );
        write_line(&mut out, path, &statement)?;
    }

    for table in tables {
        stage.set_message(table.name.clone());
        let bar = progress.task(&table.name, table.rows);

        if table.name == SEQUENCE {
            let statement = format!("DELETE FROM {};", sql::quote_ident(SEQUENCE));
            write_line(&mut out, path, &statement)?;
        }

        let prefix = insert_prefix(table);
        source.visit_rows(table, |row| {
            let mut line = String::with_capacity(prefix.len() + 32);
            line.push_str(&prefix);
            for index in 0..table.columns.len() {
                if index > 0 {
                    line.push(',');
                }
                line.push_str(&sql::literal(row.get_ref(index)?));
            }
            line.push_str(");");
            write_line(&mut out, path, &line)?;
            bar.inc(1);
            written += 1;
            Ok(())
        })?;

        bar.finish_and_clear();
        stage.inc(1);
    }

    for trigger in triggers {
        write_line(
            &mut out,
            path,
            &format!("{};", trigger.statement.trim_end()),
        )?;
    }

    write_line(&mut out, path, "COMMIT;")?;
    out.flush().map_err(|e| DumpError::io(path, e))?;
    stage.set_message("done");
    stage.finish();
    Ok(written)
}

fn insert_prefix(table: &Table) -> String {
    let columns: Vec<String> = table.columns.iter().map(|c| sql::quote_ident(c)).collect();
    format!(
        "INSERT INTO {}({}) VALUES(",
        sql::quote_ident(&table.name),
        columns.join(",")
    )
}

fn write_line(out: &mut impl Write, path: &Path, line: &str) -> Result<(), DumpError> {
    writeln!(out, "{line}").map_err(|e| DumpError::io(path, e))
}
