use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::{borrow::Cow, fs, path::Path};

const PRG: &str = "unox";
const SUBCMD: &str = "find";

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
fn skips_bad_dir() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error [23][)]", &bad);
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_name() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["--name", "*.csv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: invalid value '*.csv'"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_type() -> Result<()> {
    let expected = "error: invalid value 'x' for '--type [<TYPE>...]'";
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["--type", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
fn format_file_name(expected_file: &str) -> Cow<str> {
    // Equivalent to: Cow::Borrowed(expected_file)
    expected_file.into()
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let file = format_file_name(expected_file);
    let contents = fs::read_to_string(file.as_ref())?;
    let mut expected: Vec<&str> = contents.split('\n').filter(|s| !s.is_empty()).collect();
    expected.sort();

    let cmd = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .assert()
        .success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let mut lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    lines.sort();

    assert_eq!(lines, expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn path1() -> Result<()> {
    run(
        &["./tests/resources/find/inputs"],
        "./tests/resources/find/expected/path1.txt",
    )
}

// --------------------------------------------------
#[test]
fn path_a() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/a"],
        "./tests/resources/find/expected/path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn path_a_b() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/a/b"],
        "./tests/resources/find/expected/path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn path_d() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/d"],
        "./tests/resources/find/expected/path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn path_a_b_d() -> Result<()> {
    run(
        &[
            "./tests/resources/find/inputs/a/b",
            "./tests/resources/find/inputs/d",
        ],
        "./tests/resources/find/expected/path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f() -> Result<()> {
    run(
        &["./tests/resources/find/inputs", "-t", "f"],
        "./tests/resources/find/expected/type_f.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/a", "-t", "f"],
        "./tests/resources/find/expected/type_f_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/a/b", "--type", "f"],
        "./tests/resources/find/expected/type_f_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_d() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/d", "--type", "f"],
        "./tests/resources/find/expected/type_f_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b_d() -> Result<()> {
    run(
        &[
            "./tests/resources/find/inputs/a/b",
            "./tests/resources/find/inputs/d",
            "--type",
            "f",
        ],
        "./tests/resources/find/expected/type_f_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d() -> Result<()> {
    run(
        &["./tests/resources/find/inputs", "-t", "d"],
        "./tests/resources/find/expected/type_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/a", "-t", "d"],
        "./tests/resources/find/expected/type_d_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/a/b", "--type", "d"],
        "./tests/resources/find/expected/type_d_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_d() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/d", "--type", "d"],
        "./tests/resources/find/expected/type_d_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b_d() -> Result<()> {
    run(
        &[
            "./tests/resources/find/inputs/a/b",
            "./tests/resources/find/inputs/d",
            "--type",
            "d",
        ],
        "./tests/resources/find/expected/type_d_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_l() -> Result<()> {
    run(
        &["./tests/resources/find/inputs", "-t", "l"],
        "./tests/resources/find/expected/type_l.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_l() -> Result<()> {
    run(
        &["./tests/resources/find/inputs", "-t", "l", "f"],
        "./tests/resources/find/expected/type_f_l.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv() -> Result<()> {
    run(
        &["./tests/resources/find/inputs", "-n", ".*[.]csv"],
        "./tests/resources/find/expected/name_csv.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv_mp3() -> Result<()> {
    run(
        &[
            "./tests/resources/find/inputs",
            "-n",
            ".*[.]csv",
            "-n",
            ".*[.]mp3",
        ],
        "./tests/resources/find/expected/name_csv_mp3.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_txt_path_a_d() -> Result<()> {
    run(
        &[
            "./tests/resources/find/inputs/a",
            "./tests/resources/find/inputs/d",
            "--name",
            ".*.txt",
        ],
        "./tests/resources/find/expected/name_txt_path_a_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_a() -> Result<()> {
    run(
        &["./tests/resources/find/inputs", "-n", "a"],
        "./tests/resources/find/expected/name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_name_a() -> Result<()> {
    run(
        &["./tests/resources/find/inputs", "-t", "f", "-n", "a"],
        "./tests/resources/find/expected/type_f_name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_name_a() -> Result<()> {
    run(
        &[
            "./tests/resources/find/inputs",
            "--type",
            "d",
            "--name",
            "a",
        ],
        "./tests/resources/find/expected/type_d_name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn path_g() -> Result<()> {
    run(
        &["./tests/resources/find/inputs/g.csv"],
        "./tests/resources/find/expected/path_g.txt",
    )
}

// --------------------------------------------------
#[test]
fn unreadable_dir() -> Result<()> {
    let dirname = "./tests/resources/find/inputs/cant-touch-this";
    if !Path::new(dirname).exists() {
        fs::create_dir(dirname)?;
    }

    std::process::Command::new("chmod")
        .args(["000", dirname])
        .status()
        .expect("failed");

    let cmd = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .arg("./tests/resources/find/inputs")
        .assert()
        .success();
    fs::remove_dir(dirname)?;

    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();

    assert_eq!(lines.len(), 17);

    let stderr = String::from_utf8(out.stderr.clone())?;
    assert!(stderr.contains("cant-touch-this: Permission denied"));
    Ok(())
}
