use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn open_file(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn open_file_map_err(filename: &str, subcommand: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| anyhow!("{subcommand}: {filename}: {e}"))?,
        ))),
    }
}
