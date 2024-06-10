use anyhow::Result;
use clap::Parser;

use super::subcommands::Subcommands;
use crate::handlers::{cat::cat, echo::echo, head::head};

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
        let cli = Self::parse();

        match cli.subcommands {
            Subcommands::Echo { text, omit_newline } => echo(text, omit_newline),
            Subcommands::Cat {
                files,
                number_lines,
                number_nonblank_lines,
                squeeze_blank_lines,
            } => cat(
                files,
                number_lines,
                number_nonblank_lines,
                squeeze_blank_lines,
            ),
            Subcommands::Head {
                files,
                lines,
                bytes,
            } => head(files, lines, bytes),
            // _ => Ok(()), // throw error?
        }
    }
}
