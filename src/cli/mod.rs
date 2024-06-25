pub mod help_messages;
pub mod subcommands;

use anyhow::Result;
use clap::{builder::PossibleValue, Parser, ValueEnum};
use std::ops::Range;

use crate::handlers::{cat::cat, cut::cut, echo::echo, find::find, head::head, uniq::uniq, wc::wc};
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
            Subcommands::Cut {
                files,
                delimiter,
                args_extract,
            } => cut(&files, &delimiter, &args_extract), // do ew need to move and not borrow extract?
                                                         // _ => Ok(()), // throw error?
        }
    }
}

//--------------
//move to util file?
// uniq
pub struct UniqFlags {
    pub show_count: bool,
    pub show_unique: bool,
    pub show_repeated: bool,
    pub ignore_case: bool,
}

//--------------
// find
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

//--------------
// cut
#[derive(clap::Args, Debug, Clone)]
#[group(required = true, multiple = false)]
pub struct ArgsExtract {
    /// Selected fields
    #[arg(short, long, value_name = "FIELDS")]
    pub fields: Option<String>,

    /// Selected bytes
    #[arg(short, long, value_name = "BYTES")]
    pub bytes: Option<String>,

    /// Selected chars
    #[arg(short, long, value_name = "CHARS")]
    pub chars: Option<String>,
}

pub type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}
