use anyhow::Error;

pub fn display_file_error(subcommand: &str, filename: &str, error: &Error) {
    eprintln!("{subcommand}: {filename}: {error}");
}

pub fn display_error(subcommand: &str, error: &Error) {
    eprintln!("{subcommand}: {error}");
}
