//! CLI shape. Learner-facing wording comes from `strings_de` — including the
//! headings and flag descriptions clap would otherwise render in English.

use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

use crate::strings_de as de;

#[derive(Debug, Parser)]
#[command(
    name = "wb",
    version,
    about = de::KURZBESCHREIBUNG,
    override_usage = de::NUTZUNG_WB,
    help_template = de::HILFE_VORLAGE,
    subcommand_help_heading = de::UEBERSCHRIFT_BEFEHLE,
    next_help_heading = de::UEBERSCHRIFT_OPTIONEN,
    disable_help_subcommand = true,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Cli {
    /// Point at an unpacked Werkbank folder instead of searching upwards.
    #[arg(long, value_name = "PFAD", global = true, hide = true)]
    pub wurzel: Option<PathBuf>,

    #[arg(long, global = true, help = de::FLAG_ASCII)]
    pub ascii: bool,

    #[arg(short = 'h', long = "hilfe", alias = "help", global = true,
          action = ArgAction::Help, help = de::FLAG_HILFE)]
    pub hilfe_flag: Option<bool>,

    #[arg(short = 'v', long = "version", action = ArgAction::Version, help = de::FLAG_VERSION)]
    pub version_flag: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = de::CMD_STATUS, help_template = de::HILFE_VORLAGE_BEFEHL,
              next_help_heading = de::UEBERSCHRIFT_OPTIONEN, override_usage = de::NUTZUNG_STATUS)]
    Status {
        #[arg(long, help = de::FLAG_JSON)]
        json: bool,
    },

    #[command(about = de::CMD_CHECK, help_template = de::HILFE_VORLAGE_BEFEHL,
              next_help_heading = de::UEBERSCHRIFT_OPTIONEN, override_usage = de::NUTZUNG_CHECK)]
    Check {
        #[arg(value_name = "ID", help = de::ARG_UEBUNG_ID, help_heading = de::UEBERSCHRIFT_ANGABEN)]
        id: Option<String>,
        #[arg(long, help = de::FLAG_JSON)]
        json: bool,
    },

    #[command(about = de::CMD_ERFASSE, help_template = de::HILFE_VORLAGE_BEFEHL,
              next_help_heading = de::UEBERSCHRIFT_OPTIONEN, override_usage = de::NUTZUNG_ERFASSE)]
    Erfasse {
        #[arg(value_name = "NAME", help = de::ARG_ERFASSE_NAME, help_heading = de::UEBERSCHRIFT_ANGABEN)]
        name: Option<String>,
        #[arg(value_name = "ID", help = de::ARG_UEBUNG_ID, help_heading = de::UEBERSCHRIFT_ANGABEN)]
        id: Option<String>,
        /// Folder inside the exercise, only used by `ordnerliste`.
        #[arg(long, value_name = "PFAD", help = de::FLAG_ORDNER)]
        ordner: Option<String>,
    },

    #[command(about = de::CMD_BERICHT, help_template = de::HILFE_VORLAGE_BEFEHL,
              next_help_heading = de::UEBERSCHRIFT_OPTIONEN, override_usage = de::NUTZUNG_BERICHT)]
    Bericht {
        #[arg(long, value_name = "NAME", help = de::FLAG_ALIAS)]
        alias: Option<String>,
    },

    #[command(about = de::CMD_LOESUNG, help_template = de::HILFE_VORLAGE_BEFEHL,
              next_help_heading = de::UEBERSCHRIFT_OPTIONEN, override_usage = de::NUTZUNG_LOESUNG)]
    Loesung {
        #[arg(value_name = "ID", help = de::ARG_UEBUNG_ID, help_heading = de::UEBERSCHRIFT_ANGABEN)]
        id: String,
    },

    #[command(about = de::CMD_HILFE, help_template = de::HILFE_VORLAGE_BEFEHL,
              next_help_heading = de::UEBERSCHRIFT_OPTIONEN, override_usage = de::NUTZUNG_HILFE)]
    Hilfe,

    /// Hidden dedication (CLAUDE.md rule 10).
    #[command(hide = true)]
    DeoGratias,

    /// Developer/author tooling. Never shown to learners.
    #[command(hide = true, subcommand)]
    Intern(Intern),
}

#[derive(Debug, Subcommand)]
pub enum Intern {
    /// Validate every exercise.toml under a folder (runs in CI).
    Lint {
        #[arg(value_name = "PFAD")]
        pfad: Option<PathBuf>,
    },
    /// Turn accepted answers into `expect_hash` entries for exercise.toml.
    Hash {
        #[arg(long, value_name = "SALT")]
        salt: String,
        #[arg(value_name = "ANTWORT", required = true)]
        antworten: Vec<String>,
    },
}
