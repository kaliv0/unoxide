use std::fs;

use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use walkdir::WalkDir;

pub fn grep(
    pattern: &str,
    files: &[String],
    insensitive: bool,
    recursive: bool,
    count: bool,
    invert: bool,
) -> Result<()> {
    // build regex
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(insensitive)
        .build()
        .map_err(|_| anyhow!(r#"Invalid pattern "{}""#, pattern))?; // extract error to consts, add 'grep:' as a prefix?

    // get all file entries
    let entries = find_files(files, recursive);
    let files_count = entries.len();

    // let print = |fname: &str, val: &str| {
    //     if num_files > 1 {
    //         print!("{fname}:{val}");
    //     } else {
    //         print!("{val}");
    //     }
    // };

    Ok(())
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    // let mut results = vec![];
    // for path in paths {
    //     match path.as_str() {
    //         "-" => results.push(Ok(path.to_string())),
    //         _ => match fs::metadata(path) {
    //             Err(e) => results.push(Err(anyhow!("grep: {path}: {e}"))),
    //             Ok(metadata) => {
    //                 if metadata.is_file() {
    //                     results.push(Ok(path.to_string()));
    //                 } else if metadata.is_dir() {
    //                     if !recursive {
    //                         results.push(Err(anyhow!("grep: {path} is a directory")));
    //                     } else {
    //                         for entry in WalkDir::new(path)
    //                             .into_iter()
    //                             .flatten()
    //                             .filter(|val| val.file_type().is_file())
    //                         {
    //                             results.push(Ok(entry.path().display().to_string()));
    //                         }
    //                     }
    //                 }
    //             }
    //         },
    //     }
    // }
    // results

    paths
        .iter()
        .fold(Vec::new(), |mut results, path| -> Vec<Result<String>> {
            match path.as_str() {
                "-" => {
                    results.push(Ok(path.to_string()));
                }
                _ => match fs::metadata(path) {
                    Err(e) => {
                        results.push(Err(anyhow!("grep: {path}: {e}")));
                    }
                    Ok(metadata) => {
                        if metadata.is_file() {
                            results.push(Ok(path.to_string()));
                        } else if metadata.is_dir() {
                            if !recursive {
                                results.push(Err(anyhow!("grep: {path} is a directory")));
                            } else {
                                for entry in WalkDir::new(path)
                                    .into_iter()
                                    .flatten()
                                    .filter(|val| val.file_type().is_file())
                                {
                                    results.push(Ok(entry.path().display().to_string()));
                                }
                            }
                        }
                    }
                },
            }
            results
        })
}

fn log_data(files_count: usize, file_name: &str, val: &str) {
    if files_count > 1 {
        print!("{file_name}:{val}");
    } else {
        print!("{val}");
    }
}
