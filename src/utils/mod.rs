pub mod file_reader;

use anyhow::Error;

pub fn display_error(subcommand: &str, filename: &str, error: &Error) {
    eprintln!("{subcommand}: {filename}: {error}");
}
