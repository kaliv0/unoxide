pub mod utils;

use anyhow::Result;

use utils::helpers;

const PRG: &str = "unx";
const SUBCMD: &str = "echo";

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    helpers::dies_recommends_usage(PRG, SUBCMD)
}

// --------------------------------------------------
fn run(args: &[&str]) -> Result<()> {
    helpers::run(PRG, SUBCMD, args)
}

// --------------------------------------------------
#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"])
}

// --------------------------------------------------
#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"])
}

// --------------------------------------------------
#[test]
fn hello_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"])
}
