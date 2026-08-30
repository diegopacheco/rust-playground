use std::collections::HashSet;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags, Row};

use crate::error::DumpError;
use crate::sql;
use crate::workspace::{Workspace, sibling};

pub struct SchemaObject {
    pub kind: String,
    pub name: String,
    pub statement: String,
}

pub struct Column {
    pub name: String,
    pub kind: String,
    pub required: bool,
    pub default: Option<String>,
    pub key: bool,
}

pub struct Relation {
    pub table: String,
    pub column: String,
    pub target_table: String,
    pub target_column: String,
    pub on_delete: String,
}

pub struct Index {
    pub name: String,
    pub unique: bool,
    pub columns: Vec<String>,
}

pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: u64,
}

pub struct Header {
    pub user_version: i64,
    pub application_id: i64,
}

struct Entry {
    schema: String,
    name: String,
    kind: String,
}

pub struct Source {
    connection: Connection,
    path: PathBuf,
    workspace: Option<Workspace>,
}

impl Source {
    pub fn open(path: &Path) -> Result<Self, DumpError> {
        if !path.is_file() {
            return Err(DumpError::MissingDatabase(path.to_path_buf()));
        }
        let path = std::fs::canonicalize(path).map_err(|e| DumpError::io(path, e))?;

        let source = if has_pending_log(&path) {
            Self::from_copy(path)?
        } else {
            Self::from_original(path)?
        };
        source
            .connection
            .query_row("SELECT count(*) FROM sqlite_master", [], |row| {
                row.get::<_, i64>(0)
            })?;
        Ok(source)
    }

    fn from_original(path: PathBuf) -> Result<Self, DumpError> {
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI;
        let connection = Connection::open_with_flags(read_only_uri(&path), flags)?;
        Ok(Self {
            connection,
            path,
            workspace: None,
        })
    }

    fn from_copy(path: PathBuf) -> Result<Self, DumpError> {
        let workspace = Workspace::create()?;
        for suffix in ["", "-wal", "-shm"] {
            let from = sibling(&path, suffix);
            if from.is_file() {
                let to = sibling(&workspace.database(), suffix);
                std::fs::copy(&from, &to).map_err(|e| DumpError::io(&from, e))?;
            }
        }
        let connection = Connection::open(workspace.database())?;
        connection.query_row("PRAGMA journal_mode=DELETE", [], |row| {
            row.get::<_, String>(0)
        })?;
        Ok(Self {
            connection,
            path,
            workspace: Some(workspace),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn merged_log(&self) -> bool {
        self.workspace.is_some()
    }

    pub fn header(&self) -> Result<Header, DumpError> {
        Ok(Header {
            user_version: self
                .connection
                .query_row("PRAGMA user_version", [], |row| row.get(0))?,
            application_id: self
                .connection
                .query_row("PRAGMA application_id", [], |row| row.get(0))?,
        })
    }

    pub fn schema_objects(&self) -> Result<Vec<SchemaObject>, DumpError> {
        let hidden = self.shadow_tables()?;
        let mut statement = self.connection.prepare(
            "SELECT type, name, tbl_name, sql FROM sqlite_master \
             WHERE sql IS NOT NULL AND name NOT LIKE 'sqlite\\_%' ESCAPE '\\' \
             ORDER BY CASE type \
                 WHEN 'table' THEN 1 WHEN 'view' THEN 2 WHEN 'index' THEN 3 ELSE 4 \
             END, name",
        )?;
        let mut rows = statement.query([])?;
        let mut objects = Vec::new();

        while let Some(row) = rows.next()? {
            let kind: String = row.get(0)?;
            let name: String = row.get(1)?;
            let owner: String = row.get(2)?;
            let statement: String = row.get(3)?;
            if hidden.contains(&name) || hidden.contains(&owner) {
                continue;
            }
            objects.push(SchemaObject {
                kind,
                name,
                statement,
            });
        }
        Ok(objects)
    }

    pub fn tables(&self) -> Result<Vec<Table>, DumpError> {
        let entries = self.table_list()?;
        let mut names: Vec<String> = entries
            .iter()
            .filter(|entry| entry.schema == "main")
            .filter(|entry| entry.kind == "table" || entry.kind == "virtual")
            .filter(|entry| !entry.name.starts_with("sqlite_"))
            .map(|entry| entry.name.clone())
            .collect();
        names.sort();

        if entries.iter().any(|entry| entry.name == SEQUENCE) {
            names.push(SEQUENCE.to_string());
        }
        names.iter().map(|name| self.describe(name)).collect()
    }

    pub fn visit_rows<F>(&self, table: &Table, mut visit: F) -> Result<(), DumpError>
    where
        F: FnMut(&Row<'_>) -> Result<(), DumpError>,
    {
        let columns: Vec<String> = table.columns.iter().map(|c| sql::quote_ident(c)).collect();
        let query = format!(
            "SELECT {} FROM {}",
            columns.join(", "),
            sql::quote_ident(&table.name)
        );
        let mut statement = self.connection.prepare(&query)?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            visit(row)?;
        }
        Ok(())
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    fn describe(&self, name: &str) -> Result<Table, DumpError> {
        let mut statement = self
            .connection
            .prepare(&format!("PRAGMA table_info({})", sql::quote_ident(name)))?;
        let mut rows = statement.query([])?;
        let mut columns = Vec::new();
        while let Some(row) = rows.next()? {
            columns.push(row.get::<_, String>(1)?);
        }

        let count: i64 = self.connection.query_row(
            &format!("SELECT count(*) FROM {}", sql::quote_ident(name)),
            [],
            |row| row.get(0),
        )?;

        Ok(Table {
            name: name.to_string(),
            columns,
            rows: count.max(0) as u64,
        })
    }

    fn table_list(&self) -> Result<Vec<Entry>, DumpError> {
        let mut statement = self.connection.prepare("PRAGMA table_list")?;
        let mut rows = statement.query([])?;
        let mut entries = Vec::new();
        while let Some(row) = rows.next()? {
            entries.push(Entry {
                schema: row.get(0)?,
                name: row.get(1)?,
                kind: row.get(2)?,
            });
        }
        Ok(entries)
    }

    fn shadow_tables(&self) -> Result<HashSet<String>, DumpError> {
        Ok(self
            .table_list()?
            .into_iter()
            .filter(|entry| entry.kind == "shadow")
            .map(|entry| entry.name)
            .collect())
    }
}

pub fn columns(connection: &Connection, table: &str) -> Result<Vec<Column>, DumpError> {
    let mut statement =
        connection.prepare(&format!("PRAGMA table_info({})", sql::quote_ident(table)))?;
    let mut rows = statement.query([])?;
    let mut columns = Vec::new();
    while let Some(row) = rows.next()? {
        columns.push(Column {
            name: row.get(1)?,
            kind: row.get(2)?,
            required: row.get::<_, i64>(3)? != 0,
            default: row.get(4)?,
            key: row.get::<_, i64>(5)? != 0,
        });
    }
    Ok(columns)
}

pub fn relations(connection: &Connection, table: &str) -> Result<Vec<Relation>, DumpError> {
    let mut statement = connection.prepare(&format!(
        "PRAGMA foreign_key_list({})",
        sql::quote_ident(table)
    ))?;
    let mut rows = statement.query([])?;
    let mut relations = Vec::new();
    while let Some(row) = rows.next()? {
        let target_table: String = row.get(2)?;
        let column: String = row.get(3)?;
        let target_column: Option<String> = row.get(4)?;
        relations.push(Relation {
            table: table.to_string(),
            target_column: target_column.unwrap_or_else(|| "rowid".to_string()),
            column,
            target_table,
            on_delete: row.get(6)?,
        });
    }
    Ok(relations)
}

pub fn indexes(connection: &Connection, table: &str) -> Result<Vec<Index>, DumpError> {
    let mut statement =
        connection.prepare(&format!("PRAGMA index_list({})", sql::quote_ident(table)))?;
    let mut rows = statement.query([])?;
    let mut listed = Vec::new();
    while let Some(row) = rows.next()? {
        listed.push((row.get::<_, String>(1)?, row.get::<_, i64>(2)? != 0));
    }

    let mut indexes = Vec::new();
    for (name, unique) in listed {
        let mut statement =
            connection.prepare(&format!("PRAGMA index_info({})", sql::quote_ident(&name)))?;
        let mut rows = statement.query([])?;
        let mut columns = Vec::new();
        while let Some(row) = rows.next()? {
            if let Some(column) = row.get::<_, Option<String>>(2)? {
                columns.push(column);
            }
        }
        indexes.push(Index {
            name,
            unique,
            columns,
        });
    }
    Ok(indexes)
}

pub const SEQUENCE: &str = "sqlite_sequence";

fn read_only_uri(path: &Path) -> String {
    let mut uri = String::from("file:");
    for character in path.to_string_lossy().chars() {
        match character {
            '%' => uri.push_str("%25"),
            '?' => uri.push_str("%3f"),
            '#' => uri.push_str("%23"),
            other => uri.push(other),
        }
    }
    uri.push_str("?mode=ro&immutable=1");
    uri
}

fn has_pending_log(path: &Path) -> bool {
    std::fs::metadata(sibling(path, "-wal")).is_ok_and(|meta| meta.len() > 0)
}
