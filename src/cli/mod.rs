pub mod help_messages;
pub mod subcommands;

use anyhow::Result;
use clap::{builder::PossibleValue, Parser, ValueEnum};

use crate::handlers::{cat::cat, echo::echo, find::find, head::head, uniq::uniq, wc::wc};
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
                show_unique,
                show_repeated,
                ignore_case,
            } => uniq(
                &in_file,
                out_file.as_deref(),
                &UniqFlags {
                    show_count,
                    show_unique,
                    show_repeated,
                    ignore_case,
                },
            ),
            Subcommands::Find {
                paths,
                names,
                entry_types,
                min_depth,
                max_depth,
            } => find(&paths, &names, &entry_types, min_depth, max_depth),
            // _ => Ok(()), // throw error?
        }
    }
}

//--------------
//move to util file?
pub struct UniqFlags {
    pub show_count: bool,
    pub show_unique: bool,
    pub show_repeated: bool,
    pub ignore_case: bool,
}

//--------------
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}
