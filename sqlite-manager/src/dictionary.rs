use std::path::Path;

use crate::db::Source;
use crate::describe;
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

    println!("📖 database  {}", source.path().display());
    println!(
        "{}, {}, {}, {}\n",
        report::count(tables.len(), "table"),
        report::count(kind_count(&objects, "view"), "view"),
        report::count(kind_count(&objects, "index"), "index"),
        report::count(kind_count(&objects, "trigger"), "trigger")
    );

    let mut relations = Vec::new();

    for table in &tables {
        relations.extend(describe::table(
            source.connection(),
            &table.name,
            table.rows,
        )?);
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
