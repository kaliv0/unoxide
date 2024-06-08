use anyhow::Result;
use clap::Parser;

use super::subcommands::Subcommands;
use crate::handlers::{cat::cat, echo::echo};

#[derive(Parser)]
#[clap(name = "unox")]
// #[clap(author = "")]
#[clap(version = "0.0.1")]
// #[clap(about = "")]
pub struct Cli {
    #[clap(subcommand)]
    subcommands: Subcommands,
}

impl Cli {
    pub fn run() -> Result<()> {
        // refactor return type -> use anyhow?
        let cli = Self::parse();

        match cli.subcommands {
            Subcommands::Echo { text, omit_newline } => echo(text, omit_newline),
            Subcommands::Cat {
                files,
                number_lines,
                number_nonblank_lines,
            } => cat(files, number_lines, number_nonblank_lines),
            // _ => Ok(()), // throw error?
        }
    }
}
