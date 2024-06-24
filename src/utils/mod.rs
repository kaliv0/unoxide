use anyhow::Error;

pub mod file_reader;

pub fn display_error(subcommand: &str, filename: &str, error: &Error) {
    eprintln!("{subcommand}: {filename}: {error}");
}
