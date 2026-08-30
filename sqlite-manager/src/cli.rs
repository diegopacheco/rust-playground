use std::path::{Path, PathBuf};

use crate::error::DumpError;

const EXTENSIONS: [&str; 4] = ["db", "sqlite", "sqlite3", "db3"];

pub const USAGE: &str = "\
sqlite-manager - dump, restore and verify SQLite databases

USAGE:
    sqlite-manager <COMMAND> [ARGS]

COMMANDS:
    dump   [PATH] [-o DIR]     read a database and write schema.sql and data.sql
                               PATH is a database file or a directory to search,
                               and defaults to the current directory
                               -o defaults to ./dump

    import <DIR> [-o FILE]     rebuild a database from a dump directory
                               -o defaults to ./restored.db
                               --force overwrites an existing target

    check  <DIR>               rebuild a dump in a scratch database and report
                               whether it is sound

    sql    [PATH] [--write]    open a SQL shell with syntax highlighting,
                               line numbers, tab completion and table output
                               read only unless --write is passed

    dict   [PATH]              print the data dictionary: tables, columns,
                               indexes, views, triggers and relations

    help                       print this help
    version                    print the version

Dumping never writes to, locks, or moves the source database.
Importing never writes to the dump it reads.
";

pub enum Command {
    Dump {
        database: PathBuf,
        output: PathBuf,
    },
    Import {
        source: PathBuf,
        target: PathBuf,
        force: bool,
    },
    Check {
        source: PathBuf,
    },
    Sql {
        database: PathBuf,
        write: bool,
    },
    Dictionary {
        database: PathBuf,
    },
    Help,
    Version,
}

pub fn parse<I: IntoIterator<Item = String>>(arguments: I) -> Result<Command, DumpError> {
    let mut arguments = arguments.into_iter().peekable();
    let Some(name) = arguments.next() else {
        return Ok(Command::Help);
    };

    match name.as_str() {
        "help" | "-h" | "--help" => Ok(Command::Help),
        "version" | "-V" | "--version" => Ok(Command::Version),
        "dump" => dump(arguments),
        "import" => import(arguments),
        "check" => check(arguments),
        "sql" => sql(arguments),
        "dictionary" | "dict" => dictionary(arguments),
        other => Err(DumpError::Usage(format!(
            "unknown command: {other}, try: sqlite-manager help"
        ))),
    }
}

fn dump<I: Iterator<Item = String>>(arguments: I) -> Result<Command, DumpError> {
    let options = Options::read(arguments, "dump")?;
    Ok(Command::Dump {
        database: resolve(options.value.as_deref())?,
        output: options.output.unwrap_or_else(|| PathBuf::from("dump")),
    })
}

fn import<I: Iterator<Item = String>>(arguments: I) -> Result<Command, DumpError> {
    let options = Options::read(arguments, "import")?;
    let source = options
        .value
        .ok_or_else(|| DumpError::Usage("import needs a dump directory".to_string()))?;
    Ok(Command::Import {
        source,
        target: options
            .output
            .unwrap_or_else(|| PathBuf::from("restored.db")),
        force: options.force,
    })
}

fn check<I: Iterator<Item = String>>(arguments: I) -> Result<Command, DumpError> {
    let options = Options::read(arguments, "check")?;
    let source = options
        .value
        .ok_or_else(|| DumpError::Usage("check needs a dump directory".to_string()))?;
    Ok(Command::Check { source })
}

fn sql<I: Iterator<Item = String>>(arguments: I) -> Result<Command, DumpError> {
    let options = Options::read(arguments, "sql")?;
    Ok(Command::Sql {
        database: resolve(options.value.as_deref())?,
        write: options.write,
    })
}

fn dictionary<I: Iterator<Item = String>>(arguments: I) -> Result<Command, DumpError> {
    let options = Options::read(arguments, "dictionary")?;
    Ok(Command::Dictionary {
        database: resolve(options.value.as_deref())?,
    })
}

struct Options {
    value: Option<PathBuf>,
    output: Option<PathBuf>,
    force: bool,
    write: bool,
}

impl Options {
    fn read<I: Iterator<Item = String>>(arguments: I, command: &str) -> Result<Self, DumpError> {
        let mut arguments = arguments;
        let mut options = Options {
            value: None,
            output: None,
            force: false,
            write: false,
        };

        while let Some(argument) = arguments.next() {
            match argument.as_str() {
                "-o" | "--out" => {
                    let value = arguments
                        .next()
                        .ok_or_else(|| DumpError::Usage(format!("{command} --out needs a path")))?;
                    options.output = Some(PathBuf::from(value));
                }
                "--force" => options.force = true,
                "--write" => options.write = true,
                flag if flag.starts_with('-') => {
                    return Err(DumpError::Usage(format!("unknown option: {flag}")));
                }
                value if options.value.is_none() => options.value = Some(PathBuf::from(value)),
                value => {
                    return Err(DumpError::Usage(format!("unexpected argument: {value}")));
                }
            }
        }
        Ok(options)
    }
}

fn resolve(source: Option<&Path>) -> Result<PathBuf, DumpError> {
    let target = source.unwrap_or_else(|| Path::new("."));
    if target.is_file() {
        return Ok(target.to_path_buf());
    }
    if target.is_dir() {
        return discover(target);
    }
    Err(DumpError::MissingDatabase(target.to_path_buf()))
}

fn discover(directory: &Path) -> Result<PathBuf, DumpError> {
    let entries = std::fs::read_dir(directory).map_err(|e| DumpError::io(directory, e))?;
    let mut found: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let path = entry.map_err(|e| DumpError::io(directory, e))?.path();
        if path.is_file() && has_database_extension(&path) {
            found.push(path);
        }
    }
    found.sort();

    match found.len() {
        0 => Err(DumpError::NoDatabase(directory.to_path_buf())),
        1 => Ok(found.remove(0)),
        _ => Err(DumpError::AmbiguousDatabase(found)),
    }
}

fn has_database_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| EXTENSIONS.contains(&extension.to_lowercase().as_str()))
}
