pub mod help_messages;
pub mod subcommands;

use crate::handlers::{cat::cat, echo::echo, head::head, uniq::uniq, wc::wc};
use anyhow::Result;
use clap::Parser;
use subcommands::Subcommands;

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
            Subcommands::Echo { text, omit_newline } => echo(&text, omit_newline),
            Subcommands::Cat {
                files,
                number_lines,
                number_nonblank_lines,
                squeeze_blank_lines,
            } => cat(
                &files,
                number_lines,
                number_nonblank_lines,
                squeeze_blank_lines,
            ),
            Subcommands::Head {
                files,
                lines,
                bytes,
                quiet,
                verbose,
            } => head(&files, lines, bytes, quiet, verbose),
            Subcommands::Wc {
                files,
                lines,
                words,
                bytes,
                chars,
            } => wc(&files, lines, words, bytes, chars),
            Subcommands::Uniq {
                in_file,
                out_file,
                show_count,
            } => uniq(&in_file, out_file.as_deref(), show_count),
            // _ => Ok(()), // throw error?
        }
    }
}
