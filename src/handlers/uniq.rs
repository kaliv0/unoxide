use crate::utils::{display_error, file_reader};
use anyhow::{Ok, Result};
use std::{
    fs::File,
    io::{self, BufRead, Write},
};

pub fn uniq(in_file: &str, out_file: Option<&str>, show_count: bool) -> Result<()> {
    let mut file = file_reader::open(in_file)
        .map_err(|e| display_error("uniq", in_file, &e))
        .unwrap();

    let mut output_file: Box<dyn Write> = match out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != previous.trim_end() {
            write!(output_file, "{}", format_data(show_count, count, &previous))?;
            previous = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }
    /*      counter = 1
            curr_line = file.readline()
            while next_line := file.readline():
                # count duplicates until different line is reached
                if self._compare_lines(curr_line, next_line):
                    counter += 1
                    if self.show_all_repeated:
                        self._handle_message(counter, curr_line)
                        curr_line = next_line
                    continue
                # previous line is different compared to next_one
                # but 'uniq' only if not last in series of duplicates
                self._handle_message(counter, curr_line)
                counter = 1
                curr_line = next_line
            # handle last line in file
            self._handle_message(counter, curr_line)
    */

    write!(output_file, "{}", format_data(show_count, count, &previous))?;
    Ok(())
}

//----------------------
fn format_data(flag: bool, count: u64, text: &str) -> String {
    if count > 0 {
        // move outside function?
        if flag {
            format!("{count:>4} {text}")
        } else {
            format!("{text}")
        }
    } else {
        "".to_string()
    }
}
