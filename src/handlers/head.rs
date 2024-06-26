use anyhow::Result;
use std::io::BufRead;

use super::helpers::{error_handler::display_error, file_reader::open_file};

pub fn head(
    files: &[String],
    lines: u64,
    bytes: Option<u64>,
    quiet: bool,
    verbose: bool,
) -> Result<()> {
    let num_files = files.len();
    for (file_num, filename) in files.iter().enumerate() {
        match open_file(filename) {
            Err(e) => display_error("head", filename, &e),
            Ok(file) => {
                handle_file(
                    file, filename, num_files, file_num, bytes, lines, quiet, verbose,
                )?;
            }
        }
    }
    Ok(())
}

fn handle_file(
    mut file: Box<dyn BufRead>,
    filename: &String,
    num_files: usize,
    file_num: usize,
    bytes: Option<u64>,
    lines: u64,
    quiet: bool,
    verbose: bool,
) -> Result<()> {
    if verbose || (num_files > 1 && !quiet) {
        println!(
            "{}==> {} <==",
            if file_num > 0 { "\n" } else { "" },
            filename
        );
    }
    if let Some(num_bytes) = bytes {
        let mut buffer = vec![0; num_bytes as usize];
        let bytes_read = file.read(&mut buffer)?;
        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    } else {
        let mut line = String::new();
        for _ in 0..lines {
            let bytes = file.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }
            print!("{line}");
            line.clear();
        }
    }
    Ok(())
}
