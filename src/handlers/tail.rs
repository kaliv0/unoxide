use anyhow::{anyhow, bail, Result};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    thread,
    time::{self, Duration},
};

use super::helpers::logging::{display_file_error, display_file_header};

static PATTERN: OnceCell<Regex> = OnceCell::new();
const SLEEP_INTERVAL: Duration = time::Duration::from_secs(1);

pub fn tail(
    files: &[String],
    lines: String,
    bytes: Option<String>,
    quiet: bool,
    verbose: bool,
    follow: bool,
) -> Result<()> {
    let (lines_count, bytes_count) = parse_lines_bytes(lines, bytes)?;
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
                    follow,
                    lines_count,
                    bytes_count,
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
    follow: bool,
    lines_count: i64,
    bytes_count: Option<i64>,
) -> Result<()> {
    display_file_header(filename, quiet, verbose, files_count, file_num);
    let (total_lines, total_bytes) = count_lines_bytes(filename)?;

    let file = BufReader::new(file);
    if follow {
        follow_file(file)?;
    } else if let Some(bytes_count) = bytes_count {
        print_bytes(file, bytes_count, total_bytes)?;
    } else {
        print_lines(file, lines_count, total_lines)?;
    }
    Ok(())
}

// -------------
fn parse_lines_bytes(lines: String, bytes: Option<String>) -> Result<(i64, Option<i64>)> {
    let lines_count =
        parse_num(lines).map_err(|e| anyhow!("tail: invalid number of lines: `{e}`"))?;
    let bytes_count = bytes
        .map(parse_num)
        .transpose()
        .map_err(|e| anyhow!("tail: invalid number of bytes: `{e}`"))?;
    Ok((lines_count, bytes_count))
}

fn parse_num(val: String) -> Result<i64> {
    let pattern = PATTERN.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());

    match pattern.captures(&val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let signed_num = format!("{}{}", sign, caps.get(2).unwrap().as_str());

            if let Ok(num) = signed_num.parse::<i64>() {
                if sign == "+" && num == 0 {
                    Ok(1) // effectively similar to "cat"
                } else {
                    Ok(num)
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

fn print_bytes<T>(mut file: T, bytes_count: i64, total_bytes: i64) -> Result<()>
where
    T: Read + Seek,
{
    if let Some(start) = get_start_index(bytes_count, total_bytes) {
        file.seek(SeekFrom::Start(start))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
        }
    }
    Ok(())
}

fn print_lines<T>(mut file: T, lines_count: i64, total_lines: i64) -> Result<()>
where
    T: BufRead,
{
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

fn get_start_index(value: i64, total: i64) -> Option<u64> {
    if value == 0 || total == 0 || value > total {
        None
    } else {
        let start = if value < 0 { total + value } else { value - 1 };
        Some(if start < 0 { 0 } else { start as u64 })
    }
}

fn follow_file<T>(mut file: T) -> Result<()>
where
    T: Read + Seek + BufRead,
{
    let mut buf = Vec::new();
    file.seek(SeekFrom::End(0))?;
    loop {
        file.read_until(b'\n', &mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
            buf.clear();
        } else {
            file.seek(SeekFrom::End(0))?;
            thread::sleep(SLEEP_INTERVAL);
        }
    }
}
