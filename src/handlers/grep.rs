use anyhow::{anyhow, Result};
use regex::{Regex, RegexBuilder};
use std::{fs, io::BufRead, mem};
use walkdir::WalkDir;

use super::helpers::{
    file_reader::open_file,
    logging::{display_error, display_file_error},
};

pub fn grep(
    pattern: &str,
    files: &[String],
    ignore_case: bool,
    recursive: bool,
    count: bool,
    invert: bool,
) -> Result<()> {
    let pattern = build_regex_pattern(pattern, ignore_case)?;
    let files = find_files(files, recursive)?;
    let files_count = files.len();

    for filename in files {
        match open_file(&filename) {
            Err(e) => display_file_error("grep", &filename, &e),
            Ok(file) => match find_lines(file, &pattern, invert) {
                Err(e) => display_error("grep", &e),
                Ok(matches) => {
                    if count {
                        log_data(files_count, &filename, &format!("{}\n", matches.len()));
                    } else {
                        matches
                            .iter()
                            .for_each(|line| log_data(files_count, &filename, line))
                    }
                }
            },
        }
    }
    Ok(())
}

//------------------
fn build_regex_pattern(pattern: &str, ignore_case: bool) -> Result<Regex> {
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(ignore_case)
        .build()
        .map_err(|_| anyhow!(r#"grep: invalid pattern `{pattern}`"#))?;
    Ok(pattern)
}

fn log_data(files_count: usize, file_name: &str, value: &str) {
    if files_count > 1 {
        print!("{file_name}:{value}");
    } else {
        print!("{value}");
    }
}

//------------------
fn find_files(paths: &[String], recursive: bool) -> Result<Vec<String>> {
    let files = paths
        .iter()
        .fold(Vec::new(), |mut results, path| -> Vec<String> {
            match path.as_str() {
                "-" => {
                    results.push(path.to_string());
                }
                _ => match fs::metadata(path) {
                    Err(e) => {
                        display_error("grep", &anyhow!("{path}: {e}"));
                    }
                    Ok(metadata) => {
                        if metadata.is_file() {
                            results.push(path.to_string());
                        } else if metadata.is_dir() {
                            if !recursive {
                                display_error("grep", &anyhow!("{path} is a directory"));
                            } else {
                                WalkDir::new(path)
                                    .into_iter()
                                    .flatten()
                                    .filter(|val| val.file_type().is_file())
                                    .for_each(|entry| {
                                        results.push(entry.path().display().to_string());
                                    })
                            }
                        }
                    }
                },
            }
            results
        });
    Ok(files)
}

fn find_lines(mut file: Box<dyn BufRead>, pattern: &Regex, invert: bool) -> Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&line) ^ invert {
            matches.push(mem::take(&mut line));
        }
        line.clear();
    }
    Ok(matches)
}
