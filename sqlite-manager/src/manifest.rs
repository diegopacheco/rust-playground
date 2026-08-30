use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use rusqlite::Connection;

use crate::db::Table;
use crate::error::DumpError;
use crate::report;
use crate::sql;

pub const FILE: &str = "manifest.txt";

pub struct Manifest {
    pub objects: u64,
    pub tables: BTreeMap<String, u64>,
}

impl Manifest {
    pub fn of(objects: usize, tables: &[Table]) -> Self {
        Self {
            objects: objects as u64,
            tables: tables
                .iter()
                .map(|table| (table.name.clone(), table.rows))
                .collect(),
        }
    }

    pub fn write(&self, path: &Path) -> Result<(), DumpError> {
        let file = File::create(path).map_err(|e| DumpError::io(path, e))?;
        let mut out = BufWriter::new(file);
        writeln!(out, "objects {}", self.objects).map_err(|e| DumpError::io(path, e))?;
        for (name, rows) in &self.tables {
            writeln!(out, "table {rows} {}", escape(name)).map_err(|e| DumpError::io(path, e))?;
        }
        out.flush().map_err(|e| DumpError::io(path, e))
    }

    pub fn read(path: &Path) -> Result<Self, DumpError> {
        let file = File::open(path).map_err(|e| DumpError::io(path, e))?;
        let mut manifest = Self {
            objects: 0,
            tables: BTreeMap::new(),
        };
        for (index, line) in BufReader::new(file).lines().enumerate() {
            let line = line.map_err(|e| DumpError::io(path, e))?;
            if line.trim().is_empty() {
                continue;
            }
            if manifest.take(&line).is_none() {
                return Err(DumpError::Statement {
                    path: path.display().to_string(),
                    line: index + 1,
                    message: format!("cannot read the manifest entry {line:?}"),
                });
            }
        }
        Ok(manifest)
    }

    pub fn disagreements(&self, connection: &Connection, rebuilt: u64) -> Vec<String> {
        let mut problems = Vec::new();
        if self.objects != rebuilt {
            problems.push(format!(
                "the manifest lists {} but the dump rebuilt {}",
                report::count(self.objects as usize, "object"),
                report::count(rebuilt as usize, "object")
            ));
        }
        for (name, expected) in &self.tables {
            let query = format!("SELECT count(*) FROM {}", sql::quote_ident(name));
            match connection.query_row(&query, [], |row| row.get::<_, i64>(0)) {
                Ok(found) if found.max(0) as u64 == *expected => {}
                Ok(found) => problems.push(format!(
                    "{name} rebuilt {} but the manifest lists {}",
                    report::count(found.max(0) as usize, "row"),
                    report::count(*expected as usize, "row")
                )),
                Err(_) => problems.push(format!("{name} is missing from the dump")),
            }
        }
        problems
    }

    fn take(&mut self, line: &str) -> Option<()> {
        let (keyword, rest) = line.split_once(' ')?;
        match keyword {
            "objects" => self.objects = rest.trim().parse().ok()?,
            "table" => {
                let (rows, name) = rest.split_once(' ')?;
                self.tables.insert(unescape(name), rows.parse().ok()?);
            }
            _ => return None,
        }
        Some(())
    }
}

fn escape(name: &str) -> String {
    name.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn unescape(name: &str) -> String {
    let mut text = String::with_capacity(name.len());
    let mut characters = name.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            text.push(character);
            continue;
        }
        match characters.next() {
            Some('n') => text.push('\n'),
            Some('r') => text.push('\r'),
            Some(other) => text.push(other),
            None => text.push('\\'),
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_with_a_newline_survives_the_round_trip() {
        let awkward = "od\nd\\ ball";
        let escaped = escape(awkward);
        assert!(
            !escaped.contains('\n'),
            "a raw newline would split the line"
        );
        assert_eq!(unescape(&escaped), awkward);
    }

    #[test]
    fn entries_are_read_back_as_they_were_written() {
        let mut manifest = Manifest {
            objects: 3,
            tables: BTreeMap::new(),
        };
        manifest.take("objects 10").unwrap();
        manifest.take("table 17534 episodes").unwrap();
        manifest.take("table 4 od\\nd").unwrap();
        assert_eq!(manifest.objects, 10);
        assert_eq!(manifest.tables["episodes"], 17534);
        assert_eq!(manifest.tables["od\nd"], 4);
    }

    #[test]
    fn a_damaged_manifest_line_is_refused() {
        let mut manifest = Manifest {
            objects: 0,
            tables: BTreeMap::new(),
        };
        assert!(manifest.take("frobnicate 1").is_none());
        assert!(manifest.take("table nine episodes").is_none());
        assert!(manifest.take("objects").is_none());
    }
}
