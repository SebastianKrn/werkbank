//! Werkbank runner `wb`.
//!
//! A single portable binary: no server, no accounts, no database, no network,
//! no LLM (CLAUDE.md rule 1). It reads exercises, runs declarative checks and
//! answers in German.

mod capture;
mod checks;
mod cli;
mod clock;
mod commands;
mod content;
mod error;
mod exercise;
mod progress;
mod report;
mod strings_de;
mod workspace;

use clap::Parser;

use cli::{Cli, Command};
use error::Result;
use strings_de::{self as de, Symbols};
use workspace::Workspace;

/// Exit codes (SPEC §2): 0 = everything passed, 1 = something is still open,
/// 2 = usage or configuration problem.
fn main() {
    let code = match Cli::try_parse() {
        Ok(args) => match dispatch(args) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("{err}");
                2
            }
        },
        Err(err) => clap_error(err),
    };
    std::process::exit(code);
}

/// clap's own messages are English; only `--help`/`--version` are let through.
/// Everything else becomes a German sentence plus our German help.
fn clap_error(err: clap::Error) -> i32 {
    use clap::error::ErrorKind;
    match err.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayVersion
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
            let _ = err.print();
            0
        }
        _ => {
            eprintln!("{}", de::unbekannter_befehl());
            eprintln!();
            eprintln!("{}", de::hilfe());
            2
        }
    }
}

fn dispatch(args: Cli) -> Result<i32> {
    let symbols = Symbols::set(args.ascii);
    let root = args.wurzel.as_deref();

    match args.command {
        // `wb` alone is the same as `wb hilfe` — the learner typed something,
        // they should get something useful back.
        None | Some(Command::Hilfe) => commands::hilfe(),
        Some(Command::DeoGratias) => commands::deo_gratias(),
        Some(Command::Intern(intern)) => commands::intern(&intern, root),
        Some(Command::Status { json }) => {
            let workspace = Workspace::open(root)?;
            commands::status(&workspace, json, symbols)
        }
        Some(Command::Check { id, json }) => {
            let workspace = Workspace::open(root)?;
            commands::check(&workspace, id.as_deref(), json, symbols)
        }
        Some(Command::Erfasse { name, id, ordner }) => {
            let workspace = Workspace::open(root)?;
            commands::erfasse(
                &workspace,
                name.as_deref(),
                id.as_deref(),
                ordner.as_deref(),
            )
        }
        Some(Command::Bericht { alias }) => {
            let workspace = Workspace::open(root)?;
            commands::bericht(&workspace, alias.as_deref())
        }
        Some(Command::Loesung { id }) => {
            let workspace = Workspace::open(root)?;
            commands::loesung(&workspace, &id)
        }
    }
}
