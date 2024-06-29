use anyhow::Result;
use std::io::BufRead;

use super::helpers::{
    file_reader::open_file,
    logging::{display_file_error, display_file_header},
};

pub fn head(
    files: &[String],
    lines: u64,
    bytes: Option<u64>,
    quiet: bool,
    verbose: bool,
) -> Result<()> {
    for (file_num, filename) in files.iter().enumerate() {
        match open_file(filename) {
            Err(e) => display_file_error("head", filename, &e),
            Ok(file) => {
                handle_file(
                    file,
                    filename,
                    files.len(),
                    file_num,
                    bytes,
                    lines,
                    quiet,
                    verbose,
                )?;
            }
        }
    }
    Ok(())
}

fn handle_file(
    mut file: Box<dyn BufRead>,
    filename: &str,
    files_count: usize,
    file_num: usize,
    bytes: Option<u64>,
    lines: u64,
    quiet: bool,
    verbose: bool,
) -> Result<()> {
    display_file_header(filename, quiet, verbose, files_count, file_num);

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
