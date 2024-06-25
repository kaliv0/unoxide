use anyhow::{anyhow, bail, Result};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use regex::Regex;
use std::{
    io::{self, BufRead},
    num::NonZeroUsize,
    ops::Range,
};

use crate::{
    cli::{ArgsExtract, Extract, PositionList},
    utils::{display_error, file_reader},
};

pub fn cut(files: &[String], delimiter: &str, args_extract: &ArgsExtract) -> Result<()> {
    let delimiter = parse_delimiter(delimiter)?;
    let extract = parse_extract(args_extract.to_owned())?; //other ways to handle this?

    for filename in files {
        match file_reader::open(filename) {
            Err(e) => display_error("cut", filename, &e),
            Ok(file) => handle_file(file, delimiter, &extract)?,
        }
    }
    Ok(())
}

//--------------
// helpers
fn parse_delimiter(delimiter: &str) -> Result<u8> {
    let delim_bytes = delimiter.as_bytes();
    if delim_bytes.len() != 1 {
        bail!("cut: delimiter must be a single byte");
    }
    let delimiter: u8 = *delim_bytes.first().unwrap();
    Ok(delimiter)
}

fn parse_extract(extract: ArgsExtract) -> Result<Extract> {
    let extract = if let Some(fields) = extract.fields.map(parse_positions).transpose()? {
        Extract::Fields(fields)
    } else if let Some(bytes) = extract.bytes.map(parse_positions).transpose()? {
        Extract::Bytes(bytes)
    } else if let Some(chars) = extract.chars.map(parse_positions).transpose()? {
        Extract::Chars(chars)
    } else {
        unreachable!("Must have --fields, --bytes, or --chars");
    };
    Ok(extract)
}

//--------------
fn handle_file(file: Box<dyn BufRead>, delimiter: u8, extract: &Extract) -> Result<()> {
    match &extract {
        Extract::Fields(field_positions) => {
            let mut reader = ReaderBuilder::new()
                .delimiter(delimiter)
                .has_headers(false)
                .from_reader(file);

            let mut writer = WriterBuilder::new()
                .delimiter(delimiter)
                .from_writer(io::stdout());

            for record in reader.records() {
                writer.write_record(extract_fields(&record?, field_positions))?;
            }
        }
        Extract::Bytes(byte_positions) => {
            for line in file.lines() {
                println!("{}", extract_bytes(&line?, byte_positions));
            }
        }
        Extract::Chars(char_positions) => {
            for line in file.lines() {
                println!("{}", extract_chars(&line?, char_positions));
            }
        }
    }
    Ok(())
}

//--------------
fn parse_positions(range: String) -> Result<PositionList> {
    let range_regex = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    //refactor and simplify
    range
        .split(',')
        .map(|val| {
            parse_index(val).map(|n| n..n + 1).or_else(|e| {
                range_regex.captures(val).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 > n2 {
                        bail!(
                            // "First number in range ({}) must be lower than second number ({})",
                            // n1 + 1,
                            // n2 + 1
                            "cut: invalid decreasing range"
                        );
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<PositionList>>()
        .map_err(From::from) // do we need this?
}

fn parse_index(input: &str) -> Result<usize> {
    let value_error = || anyhow!(r#"cut: invalid extract value: `{input}`"#);
    // TODO: refactor?
    input
        .starts_with('+')
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| value_error())
        })

    // if input.starts_with("+") {
    //     Err(value_error())
    // } else {
    //     input
    //         .parse::<NonZeroUsize>()
    //         .map(|n| usize::from(n) - 1)
    //         .map_err(|_| value_error())
    // }
}

// -------------------
fn extract_fields<'a>(record: &'a StringRecord, field_positions: &[Range<usize>]) -> Vec<&'a str> {
    field_positions
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}

fn extract_bytes(line: &str, byte_positions: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<u8> = byte_positions
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_chars(line: &str, char_positions: &[Range<usize>]) -> String {
    let chars: Vec<char> = line.chars().collect();
    char_positions
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}
