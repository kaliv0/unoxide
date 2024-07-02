use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

use super::helpers::logging::display_error;
use crate::utils::owner::Owner;

pub fn ls(paths: &[String], long: bool, show_hidden: bool) -> Result<()> {
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
    for path in paths {
        match fs::metadata(path) {
            Err(e) => display_error("ls", &anyhow!("{path}: {e}")),
            Ok(metadata) => {
                if metadata.is_dir() {
                    for entry in fs::read_dir(path)? {
                        let path = entry?.path();
                        let is_hidden = path.file_name().map_or(false, |file_name| {
                            file_name.to_string_lossy().starts_with('.')
                        });
                        if !is_hidden || show_hidden {
                            results.push(path);
                        }
                    }
                } else {
                    results.push(PathBuf::from(path));
                }
            }
        }
    }
    Ok(results)
}

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
        let permissions = format_mode(metadata.mode());
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

        table.add_row(
            Row::new()
                .with_cell(file_type) // 1
                .with_cell(permissions) // 2
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

fn format_mode(mode: u32) -> String {
    format!(
        "{}{}{}",
        apply_masks(mode, Owner::User),
        apply_masks(mode, Owner::Group),
        apply_masks(mode, Owner::Other),
    )
}

fn apply_masks(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}
