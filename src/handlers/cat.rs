use anyhow::Result;
use std::io::BufRead;

use super::helpers::{error_handler::display_file_error, file_reader::open_file};

pub fn cat(
    files: &[String],
    number_lines: bool,
    number_nonblank_lines: bool,
    squeeze_blank_lines: bool,
) -> Result<()> {
    for filename in files {
        match open_file(filename) {
            Err(e) => display_file_error("cat", filename, &e),
            Ok(file) => {
                handle_file(
                    file,
                    squeeze_blank_lines,
                    number_lines,
                    number_nonblank_lines,
                )?;
            }
        }
    }
    Ok(())
}

fn handle_file(
    file: Box<dyn BufRead>,
    squeeze_blank_lines: bool,
    number_lines: bool,
    number_nonblank_lines: bool,
) -> Result<()> {
    let mut line_num = 0;
    let mut prev_line = String::new();
    for line_result in file.lines() {
        let line = line_result?;
        if squeeze_blank_lines && line.is_empty() && prev_line.is_empty() {
            prev_line = line;
            continue;
        }

        if number_lines || (number_nonblank_lines && !line.is_empty()) {
            line_num += 1;
            println!("{:6}\t{}", line_num, line); // move vars inside braces (for constistency with other files)?
        } else {
            println!("{}", line);
        }
        prev_line = line;
    }
    Ok(())
}
