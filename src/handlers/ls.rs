use anyhow::Result;
use chrono::{DateTime, Local};
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

use crate::utils::owner::Owner;

pub fn ls(paths: &[String], long: bool, show_hidden: bool) -> Result<()> {
    // modify as in grep
    let paths = find_files(paths, show_hidden)?;
    if long {
        println!("{}", format_output(&paths)?); // all paths concatenated in table
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

// -------------
fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    let mut results = vec![];
    for name in paths {
        match fs::metadata(name) {
            Err(e) => eprintln!("{name}: {e}"), // similar in grep
            Ok(meta) => {
                if meta.is_dir() {
                    for entry in fs::read_dir(name)? {
                        let entry = entry?;
                        let path = entry.path();
                        let is_hidden = path.file_name().map_or(false, |file_name| {
                            file_name.to_string_lossy().starts_with('.')
                        });
                        if !is_hidden || show_hidden {
                            results.push(entry.path());
                        }
                    }
                } else {
                    results.push(PathBuf::from(name));
                }
            }
        }
    }
    Ok(results)
    /*
    paths
        .iter()
        .fold(Vec::new(), |mut results, path| -> Vec<Result<PathBuf>> {
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
                            if 1==0 {
                                results.push(Err(anyhow!("grep: {path} is a directory")));
                            } else {
                                WalkDir::new(path)
                                    .into_iter()
                                    .flatten()
                                    .filter(|val| val.file_type().is_file())
                                    .for_each(|entry| {
                                        results.push(Ok(entry.path().display().to_string()));
                                    })
                            }
                        }
                    }
                },
            }
            results
        })
        */
}

// --------------------------------------------------
fn format_output(paths: &[PathBuf]) -> Result<String> {
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = path.metadata()?;

        let uid = metadata.uid();
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string());

        let gid = metadata.gid();
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| gid.to_string());

        let file_type = if path.is_dir() { "d" } else { "-" };
        let perms = format_mode(metadata.mode());
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

        table.add_row(
            Row::new()
                .with_cell(file_type) // 1
                .with_cell(perms) // 2
                .with_cell(metadata.nlink()) // 3
                .with_cell(user) // 4
                .with_cell(group) // 5
                .with_cell(metadata.len()) // 6
                .with_cell(modified.format("%b %d %y %H:%M")) // 7
                .with_cell(path.display()), // 8
        );
    }
    Ok(format!("{table}"))
}

// --------------------------------------------------
/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
fn format_mode(mode: u32) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}

// --------------------------------------------------
/// Given an octal number like 0o500 and an [`Owner`],
/// return a string like "r-x"
fn mk_triple(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}
