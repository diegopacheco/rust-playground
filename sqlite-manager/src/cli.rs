use std::path::{Path, PathBuf};

use crate::error::DumpError;

const EXTENSIONS: [&str; 4] = ["db", "sqlite", "sqlite3", "db3"];

pub const USAGE: &str = "\
🗃️  sqlite-manager - dump, restore and verify SQLite databases

USAGE:
    sqlite-manager [--safe] <COMMAND> [ARGS]

COMMANDS:
    📦 dump   [PATH] [-o DIR]   read a database and write schema.sql, data.sql
                               and manifest.txt
                               PATH is a database file or a directory to search,
                               and defaults to the current directory
                               -o defaults to ./dump

    📥 import <DIR> [-o FILE]   rebuild a database from a dump directory
                               -o defaults to ./restored.db
                               --force overwrites an existing target

    🔎 check  <DIR>             rebuild a dump in a scratch database and report
                               whether it is sound, comparing every table
                               against the row counts in manifest.txt

    🐚 sql    [PATH] [--write]  open a SQL shell with syntax highlighting,
                               line numbers, tab completion and table output
                               read only unless --write is passed

    🚰 sql-pipe <PATH> [SQL]    run SQL once and print the result, no shell
                               SQL is the rest of the line, or stdin when the
                               rest of the line is empty
                               read only unless --write is given before PATH

    📖 dict   [PATH]            print the data dictionary: tables, columns,
                               indexes, views, triggers and relations

    ❓ help                     print this help
    🏷️  version                  print the version

OPTIONS:
    🔒 --safe                   read only mode, may be given anywhere on the
                               line. Refuses import, sql --write and
                               sql-pipe --write, so no database file is ever
                               opened for writing.

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
        safe: bool,
    },
    Pipe {
        database: PathBuf,
        statement: Option<String>,
        write: bool,
    },
    Dictionary {
        database: PathBuf,
    },
    Help,
    Version,
}

pub fn parse<I: IntoIterator<Item = String>>(arguments: I) -> Result<Command, DumpError> {
    let arguments: Vec<String> = arguments.into_iter().collect();
    if let Some(position) = arguments.iter().position(|argument| argument == "sql-pipe") {
        return pipe(&arguments[..position], &arguments[position + 1..]);
    }

    let mut safe = false;
    let mut given: Vec<String> = Vec::new();
    for argument in arguments {
        if argument == "--safe" {
            safe = true;
        } else {
            given.push(argument);
        }
    }

    let mut arguments = given.into_iter();
    let Some(name) = arguments.next() else {
        return Ok(Command::Help);
    };

    match name.as_str() {
        "help" | "-h" | "--help" => Ok(Command::Help),
        "version" | "-V" | "--version" => Ok(Command::Version),
        "dump" => dump(arguments),
        "import" if safe => Err(DumpError::Blocked(
            "import builds a database, so it cannot run".to_string(),
        )),
        "import" => import(arguments),
        "check" => check(arguments),
        "sql" => sql(arguments, safe),
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

fn sql<I: Iterator<Item = String>>(arguments: I, safe: bool) -> Result<Command, DumpError> {
    let options = Options::read(arguments, "sql")?;
    if safe && options.write {
        return Err(DumpError::Blocked(
            "sql --write opens the database for writing, so it cannot run".to_string(),
        ));
    }
    Ok(Command::Sql {
        database: resolve(options.value.as_deref())?,
        write: options.write,
        safe,
    })
}

fn pipe(head: &[String], tail: &[String]) -> Result<Command, DumpError> {
    let mut safe = head.iter().any(|argument| argument == "--safe");
    if let Some(unexpected) = head.iter().find(|argument| *argument != "--safe") {
        return Err(DumpError::Usage(format!(
            "unexpected argument before sql-pipe: {unexpected}"
        )));
    }

    let mut write = false;
    let mut rest = tail.iter();
    let mut database = None;

    for argument in rest.by_ref() {
        match argument.as_str() {
            "--safe" => safe = true,
            "--write" => write = true,
            flag if flag.starts_with('-') => {
                return Err(DumpError::Usage(format!("unknown option: {flag}")));
            }
            value => {
                database = Some(PathBuf::from(value));
                break;
            }
        }
    }

    if safe && write {
        return Err(DumpError::Blocked(
            "sql-pipe --write opens the database for writing, so it cannot run".to_string(),
        ));
    }
    let database =
        database.ok_or_else(|| DumpError::Usage("sql-pipe needs a database".to_string()))?;
    if !database.is_file() {
        return Err(DumpError::MissingDatabase(database));
    }

    let words: Vec<&str> = rest.map(String::as_str).collect();
    Ok(Command::Pipe {
        database,
        statement: (!words.is_empty()).then(|| words.join(" ")),
        write,
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
