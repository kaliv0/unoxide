pub mod utils;

use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;
use std::fs;

use utils::helpers::dies_recommends_usage;

const PRG: &str = "unox";
const SUBCMD: &str = "echo";

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    dies_recommends_usage(PRG, SUBCMD)
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/resources/echo/expected/hello1.txt")
}

// --------------------------------------------------
#[test]
fn hello2() -> Result<()> {
    run(
        &["Hello", "there"],
        "tests/resources/echo/expected/hello2.txt",
    )
}

// --------------------------------------------------
#[test]
fn hello1_no_newline() -> Result<()> {
    run(
        &["Hello  there", "-n"],
        "tests/resources/echo/expected/hello1.n.txt",
    )
}

// --------------------------------------------------
#[test]
fn hello2_no_newline() -> Result<()> {
    run(
        &["-n", "Hello", "there"],
        "tests/resources/echo/expected/hello2.n.txt",
    )
}
