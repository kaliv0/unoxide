use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

pub fn dies_recommends_usage(command: &str, subcommand: &str) -> Result<()> {
    Command::cargo_bin(command)?
        .arg(subcommand)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

pub fn skips_bad_entry(
    command: &str,
    subcommand: &str,
    args: &[&str],
    expected: &str,
) -> Result<()> {
    Command::cargo_bin(command)?
        .arg(subcommand)
        .args(args)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

pub fn run(command: &str, subcommand: &str, args: &[&str]) -> Result<()> {
    let expected = std::process::Command::new(subcommand)
        .args(args)
        .output()
        .unwrap();
    let actual = Command::cargo_bin(command)?
        .arg(subcommand)
        .args(args)
        .output()
        .unwrap();

    let expected_stdout = String::from_utf8(expected.stdout).expect("invalid UTF-8");
    let actual_stdout = String::from_utf8(actual.stdout).expect("invalid UTF-8");
    assert!(actual.status.success());
    assert_eq!(expected_stdout.trim_end(), actual_stdout.trim_end());
    Ok(())
}

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
