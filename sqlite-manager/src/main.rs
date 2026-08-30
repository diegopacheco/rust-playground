mod check;
mod cli;
mod command;
mod data;
mod db;
mod describe;
mod dictionary;
mod dump;
mod dumpfiles;
mod error;
mod highlight;
mod import;
mod manifest;
mod pipe;
mod progress;
mod query;
mod report;
mod schema;
mod script;
mod shell;
mod sql;
mod table;
mod workspace;

use cli::Command;
use error::DumpError;

fn main() {
    if let Err(failure) = run() {
        eprintln!("❌ sqlite-manager: {failure}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), DumpError> {
    match cli::parse(std::env::args().skip(1))? {
        Command::Help => {
            print!("{}", cli::USAGE);
            Ok(())
        }
        Command::Version => {
            println!(
                "🏷️  {} {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            );
            Ok(())
        }
        Command::Dump { database, output } => dump::run(&database, &output),
        Command::Import {
            source,
            target,
            force,
        } => import::run(&source, &target, force),
        Command::Check { source } => check::run(&source),
        Command::Sql {
            database,
            write,
            safe,
        } => shell::run(&database, write, safe),
        Command::Pipe {
            database,
            statement,
            write,
        } => pipe::run(&database, statement.as_deref(), write),
        Command::Dictionary { database } => dictionary::run(&database),
    }
}
