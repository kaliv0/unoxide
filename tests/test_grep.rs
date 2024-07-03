pub mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;

use utils::helpers;

const PRG: &str = "unox";
const SUBCMD: &str = "grep";
const BUSTLE: &str = "./tests/resources/grep/inputs/bustle.txt";
const EMPTY: &str = "./tests/resources/grep/inputs/empty.txt";
const FOX: &str = "./tests/resources/grep/inputs/fox.txt";
const NOBODY: &str = "./tests/resources/grep/inputs/nobody.txt";
const INPUTS_DIR: &str = "./tests/resources/grep/inputs";

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    helpers::dies_recommends_usage(PRG, SUBCMD)
}

// --------------------------------------------------
#[test]
fn dies_bad_pattern() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["*foo", FOX])
        .assert()
        .failure()
        .stderr(predicate::str::contains(r#"grep: invalid pattern `*foo`"#));
    Ok(())
}

// --------------------------------------------------
#[test]
fn warns_bad_file() -> Result<()> {
    let bad = helpers::generate_bad_file();
    let args = ["foo", &bad];
    let expected = format!("{SUBCMD}: {bad}: .* [(]os error 2[)]");
    helpers::skips_bad_entry(PRG, SUBCMD, &args, &expected)
}

// --------------------------------------------------
fn run(args: &[&str]) -> Result<()> {
    helpers::run(PRG, SUBCMD, args)
}

// --------------------------------------------------
#[test]
fn empty_file() -> Result<()> {
    run(&["foo", EMPTY])
}

// --------------------------------------------------
#[test]
fn empty_regex() -> Result<()> {
    run(&["", FOX])
}

// --------------------------------------------------
#[test]
fn bustle_capitalized() -> Result<()> {
    run(&["The", BUSTLE])
}

// --------------------------------------------------
#[test]
fn bustle_lowercase() -> Result<()> {
    run(&["the", BUSTLE])
}

// --------------------------------------------------
#[test]
fn bustle_ignore_case() -> Result<()> {
    run(&["--ignore-case", "the", BUSTLE])
}

// --------------------------------------------------
#[test]
fn nobody() -> Result<()> {
    run(&["nobody", NOBODY])
}

// --------------------------------------------------
#[test]
fn nobody_ignore_case() -> Result<()> {
    run(&["-i", "nobody", NOBODY])
}

// --------------------------------------------------
#[test]
fn multiple_files() -> Result<()> {
    run(&["The", BUSTLE, EMPTY, FOX])
}

// --------------------------------------------------
#[test]
fn multiple_files_ignore_case() -> Result<()> {
    run(&["-i", "the", BUSTLE, EMPTY, FOX])
}

// --------------------------------------------------
#[test]
fn recursive() -> Result<()> {
    run(&["--recursive", "dog", INPUTS_DIR])
}

// --------------------------------------------------
#[test]
fn recursive_ignore_case() -> Result<()> {
    run(&["-ri", "then", INPUTS_DIR])
}

// --------------------------------------------------
#[test]
fn sensitive_count_capital() -> Result<()> {
    run(&["--count", "The", BUSTLE])
}

// --------------------------------------------------
#[test]
fn sensitive_count_lower() -> Result<()> {
    run(&["--count", "the", BUSTLE])
}

// --------------------------------------------------
#[test]
fn insensitive_count() -> Result<()> {
    run(&["-ci", "the", BUSTLE])
}

// --------------------------------------------------
#[test]
fn nobody_count() -> Result<()> {
    run(&["-c", "nobody", NOBODY])
}

// --------------------------------------------------
#[test]
fn nobody_count_ignore_case() -> Result<()> {
    run(&["-ci", "nobody", NOBODY])
}

// --------------------------------------------------
#[test]
fn sensitive_count_multiple() -> Result<()> {
    run(&["-c", "The", BUSTLE, EMPTY, FOX, NOBODY])
}

// --------------------------------------------------
#[test]
fn insensitive_count_multiple() -> Result<()> {
    run(&["-ic", "the", BUSTLE, EMPTY, FOX, NOBODY])
}

// --------------------------------------------------
#[test]
fn warns_dir_not_recursive() -> Result<()> {
    let stdout = "The quick brown fox jumps over the lazy dog.";
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["fox", INPUTS_DIR, FOX])
        .assert()
        .stderr(predicate::str::contains(
            "./tests/resources/grep/inputs is a directory",
        ))
        .stdout(predicate::str::contains(stdout));
    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin() -> Result<()> {
    let input = fs::read_to_string(BUSTLE)?;
    let expected =
        fs::read_to_string("./tests/resources/grep/expected/bustle.txt.the.capitalized")?;

    let output = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .arg("The")
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin_insensitive_count() -> Result<()> {
    let files = &[BUSTLE, EMPTY, FOX, NOBODY];

    let mut input = String::new();
    for file in files {
        input += &fs::read_to_string(file)?;
    }

    let expected_file = "./tests/resources/grep/expected/the.recursive.insensitive.count.stdin";
    let expected = fs::read_to_string(expected_file)?;

    let output = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["-ci", "the", "-"])
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}
