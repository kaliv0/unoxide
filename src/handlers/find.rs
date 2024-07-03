use anyhow::Result;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use super::helpers::logging::display_file_error;
use crate::utils::entry_type::EntryType;

pub fn find(
    paths: &[String],
    names: &[Regex],
    entry_types: &[EntryType],
    min_depth: usize,
    max_depth: usize,
) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        entry_types.is_empty()
            || entry_types.iter().any(|entry_type| match entry_type {
                EntryType::Link => entry.file_type().is_symlink(),
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        names.is_empty()
            || names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in paths {
        let entries = WalkDir::new(path)
            .min_depth(min_depth)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    display_file_error("find", path, &From::from(e));
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
