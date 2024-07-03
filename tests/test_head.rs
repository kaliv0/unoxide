pub mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::{self, File};
use std::io::prelude::*;

use utils::helpers;

const PRG: &str = "unx";
const SUBCMD: &str = "head";
const EMPTY: &str = "./tests/resources/head/inputs/empty.txt";
const ONE: &str = "./tests/resources/head/inputs/one.txt";
const TWO: &str = "./tests/resources/head/inputs/two.txt";
const THREE: &str = "./tests/resources/head/inputs/three.txt";
const TWELVE: &str = "./tests/resources/head/inputs/twelve.txt";

// --------------------------------------------------
fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> Result<()> {
    let bad = random_string();
    let expected = format!(
        "invalid value '{bad}' for \
        '--bytes <BYTES>': invalid digit found in string"
    );

    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> Result<()> {
    let bad = random_string();
    let expected = format!(
        "error: invalid value '{bad}' for \
        '--lines <LINES>': invalid digit found in string"
    );
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> Result<()> {
    let msg = "the argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";

    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> Result<()> {
    let bad = helpers::generate_bad_file();
    let args = [EMPTY, &bad, ONE];
    let expected = format!("{SUBCMD}: {bad}: .* [(]os error 2[)]");
    helpers::skips_bad_entry(PRG, SUBCMD, &args, &expected)
}

// --------------------------------------------------
fn run(args: &[&str]) -> Result<()> {
    helpers::run(PRG, SUBCMD, args)
}

// --------------------------------------------------
fn run_stdin(args: &[&str], input_file: &str, expected_file: &str) -> Result<()> {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;

    let output = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .write_stdin(input)
        .args(args)
        .output()
        .expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run(&[EMPTY])
}

// --------------------------------------------------
#[test]
fn empty_n2() -> Result<()> {
    run(&[EMPTY, "-n", "2"])
}

// --------------------------------------------------
#[test]
fn empty_n4() -> Result<()> {
    run(&[EMPTY, "-n", "4"])
}

// --------------------------------------------------
#[test]
fn empty_c2() -> Result<()> {
    run(&[EMPTY, "-c", "2"])
}

// --------------------------------------------------
#[test]
fn empty_c4() -> Result<()> {
    run(&[EMPTY, "-c", "4"])
}

// --------------------------------------------------
#[test]
fn one() -> Result<()> {
    run(&[ONE])
}

#[test]
fn one_n2() -> Result<()> {
    run(&[ONE, "-n", "2"])
}

#[test]
fn one_n4() -> Result<()> {
    run(&[ONE, "-n", "4"])
}

#[test]
fn one_c2() -> Result<()> {
    run(&[ONE, "-c", "2"])
}

#[test]
fn one_c4() -> Result<()> {
    run(&[ONE, "-c", "4"])
}

#[test]
fn one_stdin() -> Result<()> {
    run_stdin(&[], ONE, "tests/resources/head/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() -> Result<()> {
    run_stdin(
        &["-n", "2"],
        ONE,
        "tests/resources/head/expected/one.txt.n2.out",
    )
}

#[test]
fn one_n4_stdin() -> Result<()> {
    run_stdin(
        &["-n", "4"],
        ONE,
        "tests/resources/head/expected/one.txt.n4.out",
    )
}

#[test]
fn one_c1_stdin() -> Result<()> {
    run_stdin(
        &["-c", "1"],
        ONE,
        "tests/resources/head/expected/one.txt.c1.out",
    )
}

#[test]
fn one_c2_stdin() -> Result<()> {
    run_stdin(
        &["-c", "2"],
        ONE,
        "tests/resources/head/expected/one.txt.c2.out",
    )
}

#[test]
fn one_c4_stdin() -> Result<()> {
    run_stdin(
        &["-c", "4"],
        ONE,
        "tests/resources/head/expected/one.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> Result<()> {
    run(&[TWO])
}

#[test]
fn two_n2() -> Result<()> {
    run(&[TWO, "-n", "2"])
}

#[test]
fn two_n4() -> Result<()> {
    run(&[TWO, "-n", "4"])
}

#[test]
fn two_c2() -> Result<()> {
    run(&[TWO, "-c", "2"])
}

#[test]
fn two_c4() -> Result<()> {
    run(&[TWO, "-c", "4"])
}

#[test]
fn two_stdin() -> Result<()> {
    run_stdin(&[], TWO, "tests/resources/head/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() -> Result<()> {
    run_stdin(
        &["-n", "2"],
        TWO,
        "tests/resources/head/expected/two.txt.n2.out",
    )
}

#[test]
fn two_n4_stdin() -> Result<()> {
    run_stdin(
        &["-n", "4"],
        TWO,
        "tests/resources/head/expected/two.txt.n4.out",
    )
}

#[test]
fn two_c2_stdin() -> Result<()> {
    run_stdin(
        &["-c", "2"],
        TWO,
        "tests/resources/head/expected/two.txt.c2.out",
    )
}

#[test]
fn two_c4_stdin() -> Result<()> {
    run_stdin(
        &["-c", "4"],
        TWO,
        "tests/resources/head/expected/two.txt.c4.out",
    )
}

#[test]
fn three_n2() -> Result<()> {
    run(&[THREE, "-n", "2"])
}

#[test]
fn three_c2() -> Result<()> {
    run(&[THREE, "-c", "2"])
}

#[test]
fn three_c4() -> Result<()> {
    run(&[THREE, "-c", "4"])
}

#[test]
fn three_n2_stdin() -> Result<()> {
    run_stdin(
        &["-n", "2"],
        THREE,
        "tests/resources/head/expected/three.txt.n2.out",
    )
}

#[test]
fn three_c2_stdin() -> Result<()> {
    run_stdin(
        &["-c", "2"],
        THREE,
        "tests/resources/head/expected/three.txt.c2.out",
    )
}

#[test]
fn three_c4_stdin() -> Result<()> {
    run_stdin(
        &["-c", "4"],
        THREE,
        "tests/resources/head/expected/three.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn twelve() -> Result<()> {
    run(&[TWELVE])
}

#[test]
fn twelve_n2() -> Result<()> {
    run(&[TWELVE, "-n", "2"])
}

#[test]
fn twelve_n4() -> Result<()> {
    run(&[TWELVE, "-n", "4"])
}

#[test]
fn twelve_c2() -> Result<()> {
    run(&[TWELVE, "-c", "2"])
}

#[test]
fn twelve_c4() -> Result<()> {
    run(&[TWELVE, "-c", "4"])
}

#[test]
fn twelve_stdin() -> Result<()> {
    run_stdin(&[], TWELVE, "tests/resources/head/expected/twelve.txt.out")
}

#[test]
fn twelve_n2_stdin() -> Result<()> {
    run_stdin(
        &["-n", "2"],
        TWELVE,
        "tests/resources/head/expected/twelve.txt.n2.out",
    )
}

#[test]
fn twelve_n4_stdin() -> Result<()> {
    run_stdin(
        &["-n", "4"],
        TWELVE,
        "tests/resources/head/expected/twelve.txt.n4.out",
    )
}

#[test]
fn twelve_c2_stdin() -> Result<()> {
    run_stdin(
        &["-c", "2"],
        TWELVE,
        "tests/resources/head/expected/twelve.txt.c2.out",
    )
}

#[test]
fn twelve_c4_stdin() -> Result<()> {
    run_stdin(
        &["-c", "4"],
        TWELVE,
        "tests/resources/head/expected/twelve.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_c1() -> Result<()> {
    run(&[EMPTY, TWO, THREE, TWELVE, "-c", "1"])
}

#[test]
fn multiple_files_c2() -> Result<()> {
    run(&[EMPTY, ONE, TWO, THREE, TWELVE, "-c", "2"])
}

#[test]
fn multiple_files_c4() -> Result<()> {
    run(&["-c", "4", EMPTY, ONE, TWO, THREE, TWELVE])
}
