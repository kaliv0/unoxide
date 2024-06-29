use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

use super::helpers::logging::{display_file_error, display_file_header};

#[derive(Debug, PartialEq)]
pub enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

pub fn tail(
    files: &[String],
    lines: String,
    bytes: Option<String>,
    quiet: bool,
    verbose: bool,
) -> Result<()> {
    let (lines_count, bytes_count) = parse_takeval(lines, bytes)?;
    for (file_num, filename) in files.iter().enumerate() {
        match File::open(filename) {
            Err(e) => display_file_error("tail", filename, &From::from(e)),
            Ok(file) => {
                handle_file(
                    file,
                    filename,
                    file_num,
                    files.len(),
                    quiet,
                    verbose,
                    &bytes_count,
                    &lines_count,
                )?;
            }
        }
    }
    Ok(())
}

fn handle_file(
    file: File,
    filename: &str,
    file_num: usize,
    files_count: usize,
    quiet: bool,
    verbose: bool,
    bytes_count: &Option<TakeValue>,
    lines_count: &TakeValue,
) -> Result<()> {
    display_file_header(filename, quiet, verbose, files_count, file_num);
    let (total_lines, total_bytes) = count_lines_bytes(filename)?;
    let file = BufReader::new(file);
    if let Some(bytes_count) = bytes_count {
        print_bytes(file, bytes_count, total_bytes)?;
    } else {
        print_lines(file, lines_count, total_lines)?;
    }
    Ok(())
}

// -------------
fn parse_takeval(lines: String, bytes: Option<String>) -> Result<(TakeValue, Option<TakeValue>)> {
    let lines_count = parse_num(lines).map_err(|e| anyhow!("tail: illegal line count -- {e}"))?;
    let bytes_count = bytes
        .map(parse_num)
        .transpose()
        .map_err(|e| anyhow!("tail: illegal byte count -- {e}"))?;
    Ok((lines_count, bytes_count))
}

fn parse_num(val: String) -> Result<TakeValue> {
    let pattern = Regex::new(r"^([+-])?(\d+)$")?;
    match pattern.captures(&val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let signed_num = format!("{sign}{}", caps.get(2).unwrap().as_str());

            if let Ok(num) = signed_num.parse() {
                if sign == "+" && num == 0 {
                    Ok(TakeValue::PlusZero)
                } else {
                    Ok(TakeValue::TakeNum(num))
                }
            } else {
                bail!(val)
            }
        }
        _ => bail!(val),
    }
}

fn count_lines_bytes(filename: &str) -> Result<(i64, i64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut lines_count = 0;
    let mut bytes_count = 0;
    let mut buf = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        lines_count += 1;
        bytes_count += bytes_read as i64;
        buf.clear();
    }
    Ok((lines_count, bytes_count))
}

fn print_bytes<T: Read + Seek>(
    mut file: T,
    bytes_count: &TakeValue,
    total_bytes: i64,
) -> Result<()> {
    if let Some(start) = get_start_index(bytes_count, total_bytes) {
        file.seek(SeekFrom::Start(start))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if !buffer.is_empty() {
            print!("{}", String::from_utf8_lossy(&buffer));
        }
    }

    Ok(())
}

fn print_lines(mut file: impl BufRead, lines_count: &TakeValue, total_lines: i64) -> Result<()> {
    if let Some(start) = get_start_index(lines_count, total_lines) {
        let mut line_num = 0;
        let mut buf = Vec::new();
        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    }

    Ok(())
}

fn get_start_index(take_val: &TakeValue, total: i64) -> Option<u64> {
    match take_val {
        TakeValue::PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }
        TakeValue::TakeNum(num) => {
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                let start = if num < &0 { total + num } else { num - 1 };
                Some(if start < 0 { 0 } else { start as u64 })
            }
        }
    }
}
