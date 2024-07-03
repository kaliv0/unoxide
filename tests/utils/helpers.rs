use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

pub fn generate_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

pub fn dies_recommends_usage(command: &str, subcommand: &str) -> Result<()> {
    Command::cargo_bin(command)?
        .arg(subcommand)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}
