use clap::{ArgAction, Subcommand};
use regex::Regex;

use crate::constants::help_messages;
use crate::utils::{entry_type::EntryType, extract::ArgsExtract};

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

        /// show nonempty_line_numbers
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

        /// print the first NUM lines instead of the first 10
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
            value_parser = clap::value_parser!(usize),
            default_value = "0"
        )]
        min_depth: usize,

        #[arg(
            long,
            value_name = "LEVELS",
            value_parser = clap::value_parser!(usize),
            default_value_t = usize::MAX
        )]
        max_depth: usize,
    },

    #[clap(about = help_messages::CUT)]
    Cut {
        #[arg(default_value = "-")]
        files: Vec<String>,

        /// use DELIM instead of TAB for field delimiter
        #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
        delimiter: String,

        /// use STRING as the output delimiter; the default is to use the input delimiter
        #[arg(long, value_name = "DELIMITER")]
        output_delimiter: Option<String>,

        #[command(flatten)]
        extract: ArgsExtract,
        /*
        - enable partial ranges (3-, -5) -> use std::ops::RangeTo/RangeFrom
         */
    },

    #[clap(about = help_messages::GREP)]
    Grep {
        #[arg()]
        pattern: String,

        #[arg(value_name = "FILE", default_value = "-")]
        files: Vec<String>,

        /// ignore case distinctions in patterns and data
        #[arg(short, long("ignore-case"))]
        ignore_case: bool,

        /// handle directories recursively
        #[arg(short, long)]
        recursive: bool,

        /// print found matches count
        #[arg(short, long)]
        count: bool,

        /// select non-matching lines
        #[arg(short('v'), long("invert-match"))]
        invert: bool,
    },

    #[clap(about = help_messages::COMM)]
    Comm {
        #[arg()]
        file_1: String,

        #[arg()]
        file_2: String,

        /// suppress column 1 (lines unique to FILE1)
        #[arg(short('1'), action(ArgAction::SetFalse))]
        show_col_1: bool,

        /// suppress column 2 (lines unique to FILE2)
        #[arg(short('2'), action(ArgAction::SetFalse))]
        show_col_2: bool,

        /// suppress column 3 (lines that appear in both files)
        #[arg(short('3'), action(ArgAction::SetFalse))]
        show_col_3: bool,

        /// ignore case distinctions in comparison of lines
        #[arg(short, long("ignore-case"))]
        ignore_case: bool,

        /// separate columns with given delimiter
        #[arg(short, long("output-delimiter"), default_value = "\t")]
        delimiter: String,
    },

    #[clap(about = help_messages::TAIL)]
    Tail {
        #[arg(required = true)]
        files: Vec<String>,

        /// print the first NUM lines instead of the last 10
        #[arg(short('n'), long, value_name = "LINES", default_value = "10")]
        lines: String,

        /// print the last NUM bytes of each file
        #[arg(short('c'), long, value_name = "BYTES", conflicts_with("lines"))]
        bytes: Option<String>,

        /// never print headers giving file names
        #[arg(short, long("quiet"), visible_alias = "silent")]
        quiet: bool,

        /// always print headers giving file names
        #[arg(short, long, conflicts_with("quiet"))]
        verbose: bool,

        /// output appended data as the file grows
        #[arg(short, long)]
        follow: bool,
    },
}
