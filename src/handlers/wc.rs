use crate::utils::{display_error, file_reader};
use anyhow::Result;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct FileData {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

//----------------------
pub fn wc(
    files: &[String],
    mut lines: bool,
    mut words: bool,
    mut bytes: bool,
    chars: bool,
) -> Result<()> {
    adjust_flags(&mut lines, &mut words, &mut bytes, chars);

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in files {
        match file_reader::open(filename) {
            Err(e) => display_error("wc", filename, &e),
            Ok(file) => {
                handle_file(
                    file,
                    lines,
                    words,
                    bytes,
                    chars,
                    filename,
                    &mut total_lines,
                    &mut total_words,
                    &mut total_bytes,
                    &mut total_chars,
                );
            }
        }
    }
    if files.len() > 1 {
        display_totals(
            lines,
            words,
            bytes,
            chars,
            total_lines,
            total_words,
            total_bytes,
            total_chars,
        );
    }
    Ok(())
}

//----------------------
fn adjust_flags(lines: &mut bool, words: &mut bool, bytes: &mut bool, chars: bool) {
    if [*lines, *words, *bytes, chars]
        .iter()
        .all(|val| val == &false)
    {
        *lines = true;
        *words = true;
        *bytes = true;
    }
}

fn handle_file(
    file: Box<dyn BufRead>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
    filename: &String,
    total_lines: &mut usize,
    total_words: &mut usize,
    total_bytes: &mut usize,
    total_chars: &mut usize,
) {
    if let Ok(data) = count(file) {
        println!(
            "{}{}{}{}{}",
            format_field(data.num_lines, lines),
            format_field(data.num_words, words),
            format_field(data.num_bytes, bytes),
            format_field(data.num_chars, chars),
            format_filename(filename)
        );
        *total_lines += data.num_lines;
        *total_words += data.num_words;
        *total_bytes += data.num_bytes;
        *total_chars += data.num_chars;
    }
}

fn count(mut file: Box<dyn BufRead>) -> Result<FileData> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += line.split_ascii_whitespace().count();
        num_bytes += line_bytes;
        num_chars += line.chars().count();

        line.clear();
    }
    Ok(FileData {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn display_totals(
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
    total_lines: usize,
    total_words: usize,
    total_bytes: usize,
    total_chars: usize,
) {
    println!(
        "{}{}{}{} total",
        format_field(total_lines, lines),
        format_field(total_words, words),
        format_field(total_bytes, bytes),
        format_field(total_chars, chars)
    );
}

fn format_field(value: usize, flag: bool) -> String {
    if flag {
        format!("{value:>8}")
    } else {
        "".to_string()
    }
}

fn format_filename(filename: &str) -> String {
    if filename == "-" {
        "".to_string()
    } else {
        format!(" {filename}")
    }
}
