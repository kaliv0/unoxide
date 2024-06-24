use clap::{ArgAction, Subcommand};
use regex::Regex;

use super::help_messages;
use crate::cli::EntryType;

#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(about = help_messages::ECHO)]
    Echo {
        #[arg(required(true))]
        text: Vec<String>,

        #[arg(short('n'))]
        omit_newline: bool,
    },

    #[clap(about = help_messages::CAT)]
    Cat {
        #[arg(value_name = "FILE", default_value = "-")]
        files: Vec<String>,

        /// number all output lines
        #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
        number_lines: bool,

        /// show_nonempty_line_numbers
        #[arg(short('b'), long("number-nonblank"))]
        number_nonblank_lines: bool,

        /// suppress repeated empty output lines
        #[arg(short('s'), long("squeeze-blank"))]
        squeeze_blank_lines: bool,
    },

    #[clap(about = help_messages::HEAD)]
    Head {
        #[arg(value_name = "FILE", default_value = "-")]
        files: Vec<String>,

        /// print the first NUM lines instead of the first 10;
        #[arg(
            short('n'),
            long,
            value_name = "LINES",
            value_parser = clap::value_parser!(u64).range(1..),
            default_value = "10"
        )]
        lines: u64,

        /// print the first NUM bytes of each file
        #[arg(
            short('c'),
            long,
            value_name = "BYTES", 
            value_parser = clap::value_parser!(u64).range(1..),
            conflicts_with("lines")
        )]
        bytes: Option<u64>,

        /// never print headers giving file names
        #[arg(short, long("quiet"), visible_alias = "silent")]
        quiet: bool,

        /// always print headers giving file names
        #[arg(short, long, conflicts_with("quiet"))]
        verbose: bool,
    },

    #[clap(about = help_messages::WC)]
    Wc {
        #[arg(value_name = "FILE", default_value = "-")]
        files: Vec<String>,

        /// print the newline counts
        #[arg(short, long)]
        lines: bool,

        /// print the word counts
        #[arg(short, long)]
        words: bool,

        /// print the byte counts
        #[arg(short('c'), long)]
        bytes: bool,

        /// print the character counts
        #[arg(short('m'), long, conflicts_with("bytes"))]
        chars: bool,
    },

    #[clap(about=help_messages::UNIQ)]
    Uniq {
        #[arg(value_name = "IN_FILE", default_value = "-")]
        in_file: String,

        #[arg(value_name = "OUT_FILE")]
        out_file: Option<String>,

        /// prefix lines by the number of occurrences
        #[arg(short('c'), long("count"))]
        show_count: bool,

        /// only print unique lines
        #[arg(short('u'), long("unique"))]
        show_unique: bool,

        /// only print duplicate lines, one for each group
        #[arg(short('d'), long("repeated"))]
        show_repeated: bool,

        /// ignore differences in case when comparing
        #[arg(short, long)]
        ignore_case: bool,
    },
    #[clap()]
    Find {
        #[arg(value_name = "PATH", default_value = ".")]
        paths: Vec<String>,

        #[arg(
            short('n'),
            long("name"),
            value_name = "NAME",
            value_parser(Regex::new),
            action(ArgAction::Append),
            num_args(0..)
        )]
        names: Vec<Regex>,

        #[arg(
            short('t'),
            long("type"),
            value_name = "TYPE",
            value_parser(clap::value_parser!(EntryType)),
            action(ArgAction::Append),
            num_args(0..)
        )]
        entry_types: Vec<EntryType>,

        #[arg(
            long,
            value_name = "LEVELS",
            value_parser = clap::value_parser!(usize), //remove?
            default_value = "0"
        )]
        min_depth: usize,

        #[arg(
            long,
            value_name = "LEVELS",
            value_parser = clap::value_parser!(usize), //remove?
            default_value_t = usize::MAX
        )]
        max_depth: usize,
        /*
        - size?
        - delete?
         */
    },
}
