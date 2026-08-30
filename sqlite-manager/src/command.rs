use rusqlite::Connection;

use crate::error::DumpError;
use crate::highlight;
use crate::query;

pub fn is_command(line: &str) -> bool {
    let head = line
        .trim()
        .trim_end_matches(';')
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_lowercase();
    head.starts_with('.')
        || matches!(
            head.as_str(),
            "tables" | "desc" | "describe" | "help" | "quit" | "exit"
        )
}

pub fn handle(connection: &Connection, line: &str) -> Option<Result<(), DumpError>> {
    if !is_command(line) {
        return None;
    }
    let body = line.trim().trim_end_matches(';').trim();
    let (command, rest) = match body.split_once(char::is_whitespace) {
        Some((head, tail)) => (head.to_lowercase(), tail.trim()),
        None => (body.to_lowercase(), ""),
    };
    if command == "desc" || command == "describe" {
        let name = match rest.split_once(char::is_whitespace) {
            Some((head, tail)) if head.eq_ignore_ascii_case("table") => tail.trim(),
            _ if rest.eq_ignore_ascii_case("table") => "",
            _ => rest,
        };
        return Some(describe_table(connection, unquote(name)));
    }
    let argument = rest.split_whitespace().next();

    let outcome = match command.as_str() {
        ".quit" | ".exit" | "quit" | "exit" => std::process::exit(0),
        ".help" | "help" => {
            println!("{HELP}");
            Ok(())
        }
        ".tables" | "tables" => show(connection, TABLES),
        ".schema" => match argument {
            Some(name) => {
                let statement = format!(
                    "SELECT sql FROM sqlite_master WHERE tbl_name = '{}'",
                    name.replace('\'', "''")
                );
                show(connection, &statement)
            }
            None => show(connection, "SELECT type, name, sql FROM sqlite_master"),
        },
        other => Err(DumpError::Usage(format!(
            "unknown command: {other}, try help;"
        ))),
    };
    Some(outcome)
}

fn show(connection: &Connection, statement: &str) -> Result<(), DumpError> {
    let outcome = query::run(connection, statement)
        .map_err(|failure| DumpError::Query(statement.to_string(), failure))?;
    query::print(&outcome);
    Ok(())
}

fn unquote(name: &str) -> String {
    let name = name.trim();
    let unwrapped = match name.chars().next() {
        Some('"') => name.strip_prefix('"').and_then(|n| n.strip_suffix('"')),
        Some('\'') => name.strip_prefix('\'').and_then(|n| n.strip_suffix('\'')),
        Some('`') => name.strip_prefix('`').and_then(|n| n.strip_suffix('`')),
        Some('[') => name.strip_prefix('[').and_then(|n| n.strip_suffix(']')),
        _ => None,
    };
    match unwrapped {
        Some(inner) => inner.replace("\"\"", "\""),
        None => name.to_string(),
    }
}

fn describe_table(connection: &Connection, name: String) -> Result<(), DumpError> {
    if name.is_empty() {
        return Err(DumpError::Usage(
            "desc needs a table name, try: desc table NAME;".to_string(),
        ));
    }
    let name = name.as_str();
    let rows = crate::describe::rows(connection, name)?;
    crate::describe::table(connection, name, rows)?;
    for statement in statements(connection, name)? {
        println!("\n{}", highlight::paint(statement.trim()));
    }
    println!();
    Ok(())
}

fn statements(connection: &Connection, table: &str) -> Result<Vec<String>, rusqlite::Error> {
    let mut prepared = connection.prepare(
        "SELECT sql FROM sqlite_master WHERE tbl_name = ?1 AND sql IS NOT NULL \
         ORDER BY CASE type WHEN 'table' THEN 1 WHEN 'index' THEN 2 ELSE 3 END, name",
    )?;
    let found = prepared.query_map([table], |row| row.get::<_, String>(0))?;
    found.collect()
}

const TABLES: &str = "SELECT name, type FROM pragma_table_list \
                      WHERE schema = 'main' AND type IN ('table', 'view', 'virtual') \
                      AND name NOT LIKE 'sqlite\\_%' ESCAPE '\\' \
                      ORDER BY type, name";

const HELP: &str = "  📋 tables;               list every table and view
  🔬 desc table NAME;      describe a table: columns, indexes, keys and SQL
  🧱 .schema [TABLE]       show the raw SQL that defines the schema
  ❓ help;                 this list
  👋 quit;                 leave the shell

  Each command also works dot-prefixed and without the semicolon,
  so tables; .tables and tables are the same thing.

  Statements run when they are complete, so a statement can span
  several lines. Tab completes keywords, tables and columns.
  Ctrl-C clears the current statement, Ctrl-D leaves.";
