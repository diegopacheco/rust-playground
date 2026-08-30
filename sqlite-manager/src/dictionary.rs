use std::path::Path;

use crate::db::Source;
use crate::error::DumpError;
use crate::report;

pub fn run(database: &Path) -> Result<(), DumpError> {
    let source = Source::open(database)?;
    let objects = source.schema_objects()?;
    let tables: Vec<_> = source
        .tables()?
        .into_iter()
        .filter(|table| !table.name.starts_with("sqlite_"))
        .collect();

    println!("database  {}", source.path().display());
    println!(
        "{}, {}, {}, {}\n",
        report::count(tables.len(), "table"),
        report::count(kind_count(&objects, "view"), "view"),
        report::count(kind_count(&objects, "index"), "index"),
        report::count(kind_count(&objects, "trigger"), "trigger")
    );

    let mut relations = Vec::new();

    for table in &tables {
        println!(
            "TABLE {}  ({})",
            table.name,
            report::count(table.rows as usize, "row")
        );

        let columns = source.columns(&table.name)?;
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

        let indexes = source.indexes(&table.name)?;
        for index in &indexes {
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

        for relation in source.relations(&table.name)? {
            println!(
                "  references {} -> {}.{}",
                relation.column, relation.target_table, relation.target_column
            );
            relations.push(relation);
        }
        println!();
    }

    for object in objects.iter().filter(|o| o.kind == "view") {
        println!("VIEW {}\n  {}\n", object.name, object.statement.trim());
    }
    for object in objects.iter().filter(|o| o.kind == "trigger") {
        println!("TRIGGER {}\n  {}\n", object.name, object.statement.trim());
    }

    if relations.is_empty() {
        println!("RELATIONS\n  none");
        return Ok(());
    }
    println!("RELATIONS");
    for relation in &relations {
        let cascade = if relation.on_delete == "NO ACTION" {
            String::new()
        } else {
            format!("  on delete {}", relation.on_delete.to_lowercase())
        };
        println!(
            "  {}.{} -> {}.{}{cascade}",
            relation.table, relation.column, relation.target_table, relation.target_column
        );
    }
    Ok(())
}

fn kind_count(objects: &[crate::db::SchemaObject], kind: &str) -> usize {
    objects.iter().filter(|object| object.kind == kind).count()
}
