use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DumpError {
    Usage(String),
    Blocked(String),
    MissingDatabase(PathBuf),
    NoDatabase(PathBuf),
    AmbiguousDatabase(Vec<PathBuf>),
    MissingDump(PathBuf),
    OutputCollision(PathBuf),
    TargetExists(PathBuf),
    Statement {
        path: String,
        line: usize,
        message: String,
    },
    Unsound(Vec<String>),
    Query(String, rusqlite::Error),
    Io(String, std::io::Error),
    Sqlite(rusqlite::Error),
    Shell(rustyline::error::ReadlineError),
}

impl DumpError {
    pub fn io(path: &Path, source: std::io::Error) -> Self {
        DumpError::Io(path.display().to_string(), source)
    }
}

impl fmt::Display for DumpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DumpError::Usage(message) => write!(formatter, "{message}"),
            DumpError::Blocked(message) => write!(formatter, "🔒 --safe is on, {message}"),
            DumpError::MissingDatabase(path) => {
                write!(formatter, "no such database file: {}", path.display())
            }
            DumpError::NoDatabase(path) => {
                write!(formatter, "no SQLite database found in {}", path.display())
            }
            DumpError::AmbiguousDatabase(paths) => {
                let names: Vec<String> = paths.iter().map(|p| p.display().to_string()).collect();
                write!(
                    formatter,
                    "several databases found, pass one explicitly: {}",
                    names.join(", ")
                )
            }
            DumpError::MissingDump(path) => write!(
                formatter,
                "{} is not a dump directory, it needs schema.sql and data.sql",
                path.display()
            ),
            DumpError::OutputCollision(path) => write!(
                formatter,
                "refusing to write over the source database: {}",
                path.display()
            ),
            DumpError::TargetExists(path) => write!(
                formatter,
                "{} already exists, pass --force to overwrite it",
                path.display()
            ),
            DumpError::Statement {
                path,
                line,
                message,
            } => {
                write!(formatter, "{path}:{line}: {message}")
            }
            DumpError::Unsound(problems) => {
                write!(formatter, "the dump is not sound: {}", problems.join("; "))
            }
            DumpError::Query(statement, source) => {
                let message = source.to_string();
                match message.contains(statement.trim()) {
                    true => write!(formatter, "{message}"),
                    false => write!(formatter, "{statement}: {message}"),
                }
            }
            DumpError::Io(context, source) => write!(formatter, "{context}: {source}"),
            DumpError::Sqlite(source) => write!(formatter, "sqlite: {source}"),
            DumpError::Shell(source) => write!(formatter, "shell: {source}"),
        }
    }
}

impl From<rusqlite::Error> for DumpError {
    fn from(source: rusqlite::Error) -> Self {
        DumpError::Sqlite(source)
    }
}

impl From<rustyline::error::ReadlineError> for DumpError {
    fn from(source: rustyline::error::ReadlineError) -> Self {
        DumpError::Shell(source)
    }
}
