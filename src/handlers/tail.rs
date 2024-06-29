use anyhow::{anyhow, bail, Result};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

use crate::utils::enums::TakeValue::{self, PlusZero, TakeNum};

static NUM_RE: OnceCell<Regex> = OnceCell::new();

pub fn tail(files: &[String], lines: String, bytes: Option<String>, quiet: bool) -> Result<()> {
    // parse lines/bytes to
    let lines_val = parse_num(lines).map_err(|e| anyhow!("tail: illegal line count -- {e}"))?;
    let bytes_val = bytes
        .map(parse_num)
        .transpose()
        .map_err(|e| anyhow!("tail: illegal byte count -- {e}"))?;

    let num_files = files.len();
    for (file_num, filename) in files.iter().enumerate() {
        match File::open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                if !quiet && num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" },);
                }

                let (total_lines, total_bytes) = count_lines_bytes(filename)?;
                let file = BufReader::new(file);
                if let Some(num_bytes) = &bytes_val {
                    print_bytes(file, num_bytes, total_bytes)?;
                } else {
                    print_lines(file, &lines_val, total_lines)?;
                }
            }
        }
    }

    Ok(())
}

// -------------
fn parse_num(val: String) -> Result<TakeValue> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());

    match num_re.captures(&val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let signed_num = format!("{sign}{}", caps.get(2).unwrap().as_str());

            if let Ok(num) = signed_num.parse() {
                if sign == "+" && num == 0 {
                    Ok(PlusZero)
                } else {
                    Ok(TakeNum(num))
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
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += bytes_read as i64;
        buf.clear();
    }
    Ok((num_lines, num_bytes))
}

fn print_bytes<T: Read + Seek>(mut file: T, num_bytes: &TakeValue, total_bytes: i64) -> Result<()> {
    if let Some(start) = get_start_index(num_bytes, total_bytes) {
        file.seek(SeekFrom::Start(start))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if !buffer.is_empty() {
            print!("{}", String::from_utf8_lossy(&buffer));
        }
    }

    Ok(())
}

fn print_lines(mut file: impl BufRead, num_lines: &TakeValue, total_lines: i64) -> Result<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
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
        PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }
        TakeNum(num) => {
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                let start = if num < &0 { total + num } else { num - 1 };
                Some(if start < 0 { 0 } else { start as u64 })
            }
        }
    }
}
