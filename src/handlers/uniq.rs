use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead, Write},
};

use super::helpers::{file_reader::open_file, logging::display_file_error};
use crate::utils::uniq_flags::UniqFlags;

pub fn uniq(in_file: &str, out_file: Option<&str>, flags: &UniqFlags) -> Result<()> {
    match open_file(in_file) {
        Err(e) => display_file_error("uniq", in_file, &e),
        Ok(file) => {
            handle_file(file, out_file, flags)?;
        }
    }
    Ok(())
}

fn handle_file(
    mut file: Box<dyn BufRead>,
    out_file: Option<&str>,
    flags: &UniqFlags,
) -> Result<()> {
    let mut output_file = get_output_file(out_file)?;

    let mut curr_line = String::new();
    let mut prev_line = String::new();
    let mut counter: u64 = 0;
    loop {
        let bytes = file.read_line(&mut curr_line)?;
        if bytes == 0 {
            break;
        }
        if !compare_lines(&prev_line, &curr_line, flags.ignore_case) {
            log_data(&mut output_file, counter, &prev_line, flags)?;
            prev_line = curr_line.clone();
            counter = 0;
        }
        counter += 1;
        curr_line.clear();
    }
    log_data(&mut output_file, counter, &prev_line, flags)?;
    Ok(())
}

//----------------------
fn get_output_file(out_file: Option<&str>) -> Result<Box<dyn Write>> {
    let output_file: Box<dyn Write> = match out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    Ok(output_file)
}

fn compare_lines(prev_line: &str, curr_line: &str, ignore_case: bool) -> bool {
    if ignore_case {
        prev_line.trim_end().to_uppercase() == curr_line.trim_end().to_uppercase()
    } else {
        prev_line.trim_end() == curr_line.trim_end()
    }
}

fn log_data(
    mut output_file: impl io::Write,
    counter: u64,
    text: &str,
    flags: &UniqFlags,
) -> Result<()> {
    if (flags.show_unique && counter == 1)
        || (flags.show_repeated && counter > 1)
        || (!flags.show_unique && !flags.show_repeated)
    {
        write!(
            output_file,
            "{}",
            format_data(flags.show_count, counter, text)
        )?;
    }
    Ok(())
}

fn format_data(flag: bool, counter: u64, text: &str) -> String {
    if counter == 0 {
        "".to_string()
    } else if flag {
        format!("{counter:>4} {text}")
    } else {
        text.to_string()
    }
}
