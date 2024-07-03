pub mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

use utils::helpers;

const PRG: &str = "unox";
const SUBCMD: &str = "cut";
const CSV: &str = "./tests/resources/cut/inputs/movies1.csv";
const TSV: &str = "./tests/resources/cut/inputs/movies1.tsv";

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
fn skips_bad_file() -> Result<()> {
    let bad = helpers::generate_bad_file();
    let args = ["-f", "1", CSV, &bad, TSV];
    let expected = format!("{SUBCMD}: {bad}: .* [(]os error 2[)]");
    helpers::skips_bad_entry(PRG, SUBCMD, &args, &expected)
}

// --------------------------------------------------
fn dies(args: &[&str], expected: &str) -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_not_enough_args() -> Result<()> {
    dies(
        &[CSV],
        "the following required arguments were not provided:\n  \
        <--fields <FIELDS>|--bytes <BYTES>|--chars <CHARS>>",
    )
}

// --------------------------------------------------
#[test]
fn dies_bad_digit_field() -> Result<()> {
    let bad = random_string();
    dies(
        &[CSV, "-f", &bad],
        &format!(r#"cut: invalid extract value: `{}`"#, &bad),
    )
}

// --------------------------------------------------
#[test]
fn dies_bad_digit_bytes() -> Result<()> {
    let bad = random_string();
    dies(
        &[CSV, "-b", &bad],
        &format!(r#"cut: invalid extract value: `{}`"#, &bad),
    )
}

// --------------------------------------------------
#[test]
fn dies_bad_digit_chars() -> Result<()> {
    let bad = random_string();
    dies(
        &[CSV, "-c", &bad],
        &format!(r#"cut: invalid extract value: `{}`"#, &bad),
    )
}

// --------------------------------------------------
#[test]
fn dies_empty_delimiter() -> Result<()> {
    dies(
        &[CSV, "-f", "1", "-d", ""],
        "cut: delimiter must be a single byte",
    )
}

// --------------------------------------------------
#[test]
fn dies_bad_delimiter() -> Result<()> {
    dies(
        &[CSV, "-f", "1", "-d", ",,"],
        "cut: delimiter must be a single byte",
    )
}

// --------------------------------------------------
#[test]
fn dies_chars_bytes_fields() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args([CSV, "-c", "1", "-f", "1", "-b", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_fields() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args([CSV, "-f", "1", "-b", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_chars_fields() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args([CSV, "-c", "1", "-f", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_chars_bytes() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args([CSV, "-c", "1", "-b", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str]) -> Result<()> {
    helpers::run(PRG, SUBCMD, args)
}

// --------------------------------------------------
#[test]
fn tsv_f1() -> Result<()> {
    run(&[TSV, "-f", "1"])
}

// --------------------------------------------------
#[test]
fn tsv_f2() -> Result<()> {
    run(&[TSV, "-f", "2"])
}

// --------------------------------------------------
#[test]
fn tsv_f3() -> Result<()> {
    run(&[TSV, "-f", "3"])
}

// --------------------------------------------------
#[test]
fn tsv_f1_2() -> Result<()> {
    run(&[TSV, "-f", "1-2"])
}

// --------------------------------------------------
#[test]
fn tsv_f2_3() -> Result<()> {
    run(&[TSV, "-f", "2-3"])
}

// --------------------------------------------------
#[test]
fn tsv_f1_3() -> Result<()> {
    run(&[TSV, "-f", "1-3"])
}

// --------------------------------------------------
#[test]
fn csv_f1() -> Result<()> {
    run(&[CSV, "-f", "1", "-d", ","])
}

// --------------------------------------------------
#[test]
fn csv_f2() -> Result<()> {
    run(&[CSV, "-f", "2", "-d", ","])
}

// --------------------------------------------------
#[test]
fn csv_f3() -> Result<()> {
    run(&[CSV, "-f", "3", "-d", ","])
}

// --------------------------------------------------
#[test]
fn csv_f1_2() -> Result<()> {
    run(&[CSV, "-f", "1-2", "-d", ","])
}

// --------------------------------------------------
#[test]
fn csv_f2_3() -> Result<()> {
    run(&[CSV, "-f", "2-3", "-d", ","])
}

// --------------------------------------------------
#[test]
fn csv_f1_3() -> Result<()> {
    run(&[CSV, "-f", "1-3", "-d", ","])
}

// --------------------------------------------------
#[test]
fn tsv_b1() -> Result<()> {
    run(&[TSV, "-b", "1"])
}

// --------------------------------------------------
#[test]
fn tsv_b2() -> Result<()> {
    run(&[TSV, "-b", "2"])
}

// --------------------------------------------------
#[test]
fn tsv_b1_2() -> Result<()> {
    run(&[TSV, "-b", "1-2"])
}

// --------------------------------------------------
#[test]
fn tsv_b2_3() -> Result<()> {
    run(&[TSV, "-b", "2-3"])
}

// --------------------------------------------------
#[test]
fn tsv_c1() -> Result<()> {
    run(&[TSV, "-c", "1"])
}

// --------------------------------------------------
#[test]
fn tsv_c2() -> Result<()> {
    run(&[TSV, "-c", "2"])
}

// --------------------------------------------------
#[test]
fn tsv_c7() -> Result<()> {
    run(&[TSV, "-c", "7"])
}

// --------------------------------------------------
#[test]
fn tsv_c1_2() -> Result<()> {
    run(&[TSV, "-c", "1-2"])
}

// --------------------------------------------------
#[test]
fn tsv_c2_3() -> Result<()> {
    run(&[TSV, "-c", "2-3"])
}

// --------------------------------------------------
#[test]
fn tsv_c1_7() -> Result<()> {
    run(&[TSV, "-c", "1-7"])
}
