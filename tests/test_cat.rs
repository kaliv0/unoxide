use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "unox";
const SUBCMD: &str = "cat";
const EMPTY: &str = "tests/resources/cat/inputs/empty.txt";
const FOX: &str = "tests/resources/cat/inputs/fox.txt";
const SPIDERS: &str = "tests/resources/cat/inputs/spiders.txt";
const BUSTLE: &str = "tests/resources/cat/inputs/the-bustle.txt";

// --------------------------------------------------
#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
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

// --------------------------------------------------
#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

// --------------------------------------------------
fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> Result<()> {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .write_stdin(input)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle_stdin() -> Result<()> {
    run_stdin(
        BUSTLE,
        &["-"],
        "tests/resources/cat/expected/the-bustle.txt.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_n() -> Result<()> {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/resources/cat/expected/the-bustle.txt.n.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b() -> Result<()> {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/resources/cat/expected/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run(&[EMPTY], "tests/resources/cat/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n() -> Result<()> {
    run(
        &["-n", EMPTY],
        "tests/resources/cat/expected/empty.txt.n.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_b() -> Result<()> {
    run(
        &["-b", EMPTY],
        "tests/resources/cat/expected/empty.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn fox() -> Result<()> {
    run(&[FOX], "tests/resources/cat/expected/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> Result<()> {
    run(&["-n", FOX], "tests/resources/cat/expected/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> Result<()> {
    run(&["-b", FOX], "tests/resources/cat/expected/fox.txt.b.out")
}

// --------------------------------------------------
#[test]
fn spiders() -> Result<()> {
    run(&[SPIDERS], "tests/resources/cat/expected/spiders.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> Result<()> {
    run(
        &["--number", SPIDERS],
        "tests/resources/cat/expected/spiders.txt.n.out",
    )
}

// --------------------------------------------------
#[test]
fn spiders_b() -> Result<()> {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/resources/cat/expected/spiders.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle() -> Result<()> {
    run(&[BUSTLE], "tests/resources/cat/expected/the-bustle.txt.out")
}

// --------------------------------------------------
#[test]
fn bustle_n() -> Result<()> {
    run(
        &["-n", BUSTLE],
        "tests/resources/cat/expected/the-bustle.txt.n.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_b() -> Result<()> {
    run(
        &["-b", BUSTLE],
        "tests/resources/cat/expected/the-bustle.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn all() -> Result<()> {
    run(
        &[FOX, SPIDERS, BUSTLE],
        "tests/resources/cat/expected/all.out",
    )
}

// --------------------------------------------------
#[test]
fn all_n() -> Result<()> {
    run(
        &[FOX, SPIDERS, BUSTLE, "-n"],
        "tests/resources/cat/expected/all.n.out",
    )
}

// --------------------------------------------------
#[test]
fn all_b() -> Result<()> {
    run(
        &[FOX, SPIDERS, BUSTLE, "-b"],
        "tests/resources/cat/expected/all.b.out",
    )
}

//--------------------------------
//--------------------------------
//--------------------------------
#[test]
fn test_custom_vs_actual() -> Result<()> {
    let text = String::from("tests/resources/cat/inputs/spiders.txt");

    let expected = std::process::Command::new(SUBCMD)
        .arg(&text)
        .output()
        .unwrap();

    let actual = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .arg(&text)
        .output()
        .unwrap();

    let expected_stdout = String::from_utf8(expected.stdout).expect("invalid UTF-8");
    let actual_stdout = String::from_utf8(actual.stdout).expect("invalid UTF-8");

    // println!("{expected_stdout}");
    // println!("{actual_stdout}");

    assert!(actual.status.success());
    assert_eq!(expected_stdout.trim_end(), actual_stdout.trim_end());
    Ok(())
}
