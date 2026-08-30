use rusqlite::Connection;

use crate::db::{self, Relation};
use crate::error::DumpError;
use crate::report;
use crate::sql;

pub fn table(connection: &Connection, name: &str, rows: u64) -> Result<Vec<Relation>, DumpError> {
    println!(
        "🧱 {} {}  ({})",
        kind(connection, name),
        name,
        report::count(rows as usize, "row")
    );

    let columns = db::columns(connection, name)?;
    let width = columns.iter().map(|c| c.name.len()).max().unwrap_or(0);
    let kind_width = columns.iter().map(|c| c.kind.len()).max().unwrap_or(0);

    for column in &columns {
        let mut marks = Vec::new();
        if column.key {
            marks.push("primary key".to_string());
        }
        if column.required {
            marks.push("not null".to_string());
        }
        if let Some(value) = &column.default {
            marks.push(format!("default {value}"));
        }
        let kind = if column.kind.is_empty() {
            "-"
        } else {
            &column.kind
        };
        let line = format!(
            "  {:<width$}  {:<kind_width$}  {}",
            column.name,
            kind,
            marks.join(", ")
        );
        println!("{}", line.trim_end());
    }

    for index in db::indexes(connection, name)? {
        let columns = index.columns.join(", ");
        if index.name.starts_with("sqlite_autoindex_") {
            println!("  unique ({columns})");
            continue;
        }
        let label = if index.unique {
            "unique index"
        } else {
            "index"
        };
        println!("  {label} {} ({columns})", index.name);
    }

    let relations = db::relations(connection, name)?;
    for relation in &relations {
        println!(
            "  references {} -> {}.{}",
            relation.column, relation.target_table, relation.target_column
        );
    }
    Ok(relations)
}

fn kind(connection: &Connection, name: &str) -> String {
    connection
        .query_row(
            "SELECT upper(type) FROM sqlite_master WHERE name = ?1",
            [name],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "TABLE".to_string())
}

pub fn rows(connection: &Connection, name: &str) -> Result<u64, DumpError> {
    let count: i64 = connection.query_row(
        &format!("SELECT count(*) FROM {}", sql::quote_ident(name)),
        [],
        |row| row.get(0),
    )?;
    Ok(count.max(0) as u64)
}
