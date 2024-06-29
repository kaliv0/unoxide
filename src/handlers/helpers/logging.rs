use anyhow::Error;

pub fn display_error(subcommand: &str, error: &Error) {
    eprintln!("{subcommand}: {error}");
}

pub fn display_file_error(subcommand: &str, filename: &str, error: &Error) {
    eprintln!("{subcommand}: {filename}: {error}");
}

pub fn display_file_header(
    filename: &str,
    quiet: bool,
    verbose: bool,
    files_count: usize,
    file_num: usize,
) {
    if verbose || (files_count > 1 && !quiet) {
        println!(
            "{}==> {} <==",
            if file_num > 0 { "\n" } else { "" },
            filename
        );
    }
}
