pub mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;

use utils::helpers;

const PRG: &str = "unox";
const SUBCMD: &str = "ls";
const HIDDEN: &str = "./tests/resources/ls/inputs/.hidden";
const EMPTY: &str = "./tests/resources/ls/inputs/empty.txt";
const BUSTLE: &str = "./tests/resources/ls/inputs/bustle.txt";
const FOX: &str = "./tests/resources/ls/inputs/fox.txt";

// --------------------------------------------------
#[test]
fn bad_file() -> Result<()> {
    let bad = helpers::generate_bad_file();
    let expected = format!("{SUBCMD}: {bad}: .* [(]os error 2[)]");
    helpers::skips_bad_entry(PRG, SUBCMD, &[&bad], &expected)
}

// --------------------------------------------------
#[test]
fn no_args() -> Result<()> {
    // Uses current directory by default
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .assert()
        .success()
        .stdout(predicate::str::contains("Cargo.toml"));
    Ok(())
}

// --------------------------------------------------
fn run_short(arg: &str) -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .arg(arg)
        .assert()
        .success()
        .stdout(format!("{arg}\n"));
    Ok(())
}

// --------------------------------------------------
fn run_long(filename: &str, permissions: &str, size: &str) -> Result<()> {
    let cmd = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(["--long", filename])
        .assert()
        .success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let parts: Vec<_> = stdout.split_whitespace().collect();
    assert_eq!(parts.first().unwrap(), &permissions);
    assert_eq!(parts.get(4).unwrap(), &size);
    assert_eq!(parts.last().unwrap(), &filename);
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run_short(EMPTY)
}

#[test]
fn empty_long() -> Result<()> {
    run_long(EMPTY, "-rw-r--r--", "0")
}

// --------------------------------------------------
#[test]
fn bustle() -> Result<()> {
    run_short(BUSTLE)
}

#[test]
fn bustle_long() -> Result<()> {
    run_long(BUSTLE, "-rw-r--r--", "193")
}

// --------------------------------------------------
#[test]
fn fox() -> Result<()> {
    run_short(FOX)
}

#[test]
fn fox_long() -> Result<()> {
    run_long(FOX, "-rw-------", "45")
}

// --------------------------------------------------
#[test]
fn hidden() -> Result<()> {
    run_short(HIDDEN)
}

#[test]
fn hidden_long() -> Result<()> {
    run_long(HIDDEN, "-rw-r--r--", "0")
}

// --------------------------------------------------
fn dir_short(args: &[&str], expected: &[&str]) -> Result<()> {
    let cmd = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .assert()
        .success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    assert_eq!(lines.len(), expected.len());
    for filename in expected {
        assert!(lines.contains(filename));
    }
    Ok(())
}

#[test]
fn dir1() -> Result<()> {
    dir_short(
        &["./tests/resources/ls/inputs/"],
        &[
            "./tests/resources/ls/inputs/empty.txt",
            "./tests/resources/ls/inputs/bustle.txt",
            "./tests/resources/ls/inputs/fox.txt",
            "./tests/resources/ls/inputs/dir",
        ],
    )
}

#[test]
fn dir1_all() -> Result<()> {
    dir_short(
        &["./tests/resources/ls/inputs/", "--all"],
        &[
            "./tests/resources/ls/inputs/empty.txt",
            "./tests/resources/ls/inputs/bustle.txt",
            "./tests/resources/ls/inputs/fox.txt",
            "./tests/resources/ls/inputs/.hidden",
            "./tests/resources/ls/inputs/dir",
        ],
    )
}

#[test]
fn dir2() -> Result<()> {
    dir_short(
        &["./tests/resources/ls/inputs/dir"],
        &["./tests/resources/ls/inputs/dir/spiders.txt"],
    )
}

#[test]
fn dir2_all() -> Result<()> {
    dir_short(
        &["-a", "./tests/resources/ls/inputs/dir"],
        &[
            "./tests/resources/ls/inputs/dir/spiders.txt",
            "./tests/resources/ls/inputs/dir/.gitkeep",
        ],
    )
}

// --------------------------------------------------
#[allow(suspicious_double_ref_op)]
fn dir_long(args: &[&str], expected: &[(&str, &str, &str)]) -> Result<()> {
    let cmd = Command::cargo_bin(PRG)?
        .arg(SUBCMD)
        .args(args)
        .assert()
        .success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    assert_eq!(lines.len(), expected.len());

    let mut check = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let path = parts.last().unwrap().clone();
        let permissions = parts.first().unwrap().clone();
        let size = match permissions.chars().next() {
            Some('d') => "",
            _ => parts.get(4).unwrap().clone(),
        };
        check.push((path, permissions, size));
    }

    for entry in expected {
        assert!(check.contains(entry));
    }

    Ok(())
}

// --------------------------------------------------
#[test]
fn dir1_long() -> Result<()> {
    dir_long(
        &["-l", "./tests/resources/ls/inputs/"],
        &[
            ("./tests/resources/ls/inputs/empty.txt", "-rw-r--r--", "0"),
            (
                "./tests/resources/ls/inputs/bustle.txt",
                "-rw-r--r--",
                "193",
            ),
            ("./tests/resources/ls/inputs/fox.txt", "-rw-------", "45"),
            ("./tests/resources/ls/inputs/dir", "drwxr-xr-x", ""),
        ],
    )
}

#[test]
fn dir1_long_all() -> Result<()> {
    dir_long(
        &["-la", "./tests/resources/ls/inputs/"],
        &[
            ("./tests/resources/ls/inputs/empty.txt", "-rw-r--r--", "0"),
            (
                "./tests/resources/ls/inputs/bustle.txt",
                "-rw-r--r--",
                "193",
            ),
            ("./tests/resources/ls/inputs/fox.txt", "-rw-------", "45"),
            ("./tests/resources/ls/inputs/dir", "drwxr-xr-x", ""),
            ("./tests/resources/ls/inputs/.hidden", "-rw-r--r--", "0"),
        ],
    )
}

#[test]
fn dir2_long() -> Result<()> {
    dir_long(
        &["--long", "./tests/resources/ls/inputs/dir"],
        &[(
            "./tests/resources/ls/inputs/dir/spiders.txt",
            "-rw-r--r--",
            "45",
        )],
    )
}

#[test]
fn dir2_long_all() -> Result<()> {
    dir_long(
        &["./tests/resources/ls/inputs/dir", "--long", "--all"],
        &[
            (
                "./tests/resources/ls/inputs/dir/spiders.txt",
                "-rw-r--r--",
                "45",
            ),
            (
                "./tests/resources/ls/inputs/dir/.gitkeep",
                "-rw-r--r--",
                "0",
            ),
        ],
    )
}
