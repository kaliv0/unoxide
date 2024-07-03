use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "unox";
const SUBCMD: &str = "cat";
const EMPTY: &str = "./tests/resources/cat/inputs/empty.txt";
const FOX: &str = "./tests/resources/cat/inputs/fox.txt";
const SPIDERS: &str = "./tests/resources/cat/inputs/spiders.txt";
const BUSTLE: &str = "./tests/resources/cat/inputs/the-bustle.txt";

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
fn run(args: &[&str]) -> Result<()> {
    let expected = std::process::Command::new(SUBCMD)
        .args(args)
        .output()
        .unwrap();
    let actual = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .output()
        .unwrap();

    let expected_stdout = String::from_utf8(expected.stdout).expect("invalid UTF-8");
    let actual_stdout = String::from_utf8(actual.stdout).expect("invalid UTF-8");
    assert!(actual.status.success());
    assert_eq!(expected_stdout.trim_end(), actual_stdout.trim_end());
    Ok(())
}

// --------------------------------------------------
fn run_stdin(input_file: &str, args: &[&str]) -> Result<()> {
    let expected = std::process::Command::new(SUBCMD)
        .arg(&input_file)
        .args(args)
        .output()
        .unwrap();

    let actual = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .arg(&input_file)
        .args(args)
        .output()
        .unwrap();

    let expected_stdout = String::from_utf8(expected.stdout).expect("invalid UTF-8");
    let actual_stdout = String::from_utf8(actual.stdout).expect("invalid UTF-8");
    assert!(actual.status.success());
    assert_eq!(expected_stdout.trim_end(), actual_stdout.trim_end());
    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle_stdin() -> Result<()> {
    run_stdin(BUSTLE, &["-"])
}

// --------------------------------------------------
#[test]
fn bustle_stdin_n() -> Result<()> {
    run_stdin(BUSTLE, &["-n", "-"])
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b() -> Result<()> {
    run_stdin(BUSTLE, &["-b", "-"])
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run(&[EMPTY])
}

// --------------------------------------------------
#[test]
fn empty_n() -> Result<()> {
    run(&["-n", EMPTY])
}

// --------------------------------------------------
#[test]
fn empty_b() -> Result<()> {
    run(&["-b", EMPTY])
}

// --------------------------------------------------
#[test]
fn fox() -> Result<()> {
    run(&[FOX])
}

// --------------------------------------------------
#[test]
fn fox_n() -> Result<()> {
    run(&["-n", FOX])
}

// --------------------------------------------------
#[test]
fn fox_b() -> Result<()> {
    run(&["-b", FOX])
}

// --------------------------------------------------
#[test]
fn spiders() -> Result<()> {
    run(&[SPIDERS])
}

// --------------------------------------------------
#[test]
fn spiders_n() -> Result<()> {
    run(&["--number", SPIDERS])
}

// --------------------------------------------------
#[test]
fn spiders_b() -> Result<()> {
    run(&["--number-nonblank", SPIDERS])
}

// --------------------------------------------------
#[test]
fn bustle() -> Result<()> {
    run(&[BUSTLE])
}

// --------------------------------------------------
#[test]
fn bustle_n() -> Result<()> {
    run(&["-n", BUSTLE])
}

// --------------------------------------------------
#[test]
fn bustle_b() -> Result<()> {
    run(&["-b", BUSTLE])
}

// --------------------------------------------------
#[test]
fn all() -> Result<()> {
    run(&[FOX, SPIDERS, BUSTLE])
}

// --------------------------------------------------
// #[test]
// fn all_n() -> Result<()> {
//     run(
//         &[FOX, SPIDERS, BUSTLE, "-n"],
//         "tests/resources/cat/expected/all.n.out",
//     )
// }

// --------------------------------------------------
// #[test]
// fn all_b() -> Result<()> {
//     run(
//         &[FOX, SPIDERS, BUSTLE, "-b"],
//         "tests/resources/cat/expected/all.b.out",
//     )
// }
