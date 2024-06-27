use anyhow::Result;
use clap::Parser;

use super::subcommands::Subcommands;
use crate::handlers::{
    cat::cat, cut::cut, echo::echo, find::find, grep::grep, head::head, uniq::uniq, wc::wc,
};
use crate::utils::uniq_flags::UniqFlags;

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
                output_delimiter,
                extract,
            } => cut(&files, &delimiter, output_delimiter.as_deref(), &extract),
            // do we need to move and not borrow extract?
            Subcommands::Grep {
                pattern,
                files,
                ignore_case,
                recursive,
                count,
                invert,
            } => grep(&pattern, &files, ignore_case, recursive, count, invert),
            // _ => Ok(()), // throw error?
        }
    }
}
