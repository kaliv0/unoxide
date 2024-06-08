use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::{Parser, Subcommand};

// CLI interface --------//
#[derive(Parser)]
#[clap(name = "unox")]
// #[clap(author = "")]
#[clap(version = "0.0.1")]
// #[clap(about = "")]
pub struct Cli {
    #[clap(subcommand)]
    subcommands: Subcommands,
}

impl Cli {
    pub fn run() -> Result<()> {
        // refactor return type -> use anyhow?
        let cli = Self::parse();

        match cli.subcommands {
            Subcommands::Echo { text, omit_newline } => echo(text, omit_newline),
            Subcommands::Cat {
                files,
                number_lines,
                number_nonblank_lines,
            } => cat(files, number_lines, number_nonblank_lines),
            // _ => Ok(()), // throw error?
        }
    }
}

// CLI Subcommands --------//
#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(about = "")] // add description
    Echo {
        #[arg(required(true))]
        text: Vec<String>,

        #[arg(short('n'))]
        omit_newline: bool,
    },
    #[clap(about = "")]
    Cat {
        #[arg(value_name = "FILE", default_value = "-")]
        files: Vec<String>,

        #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
        number_lines: bool,

        #[arg(short('b'), long("number-nonblank"))]
        number_nonblank_lines: bool,
    },
}

// Subcommands functionalities --------//
pub fn cat(files: Vec<String>, number_lines: bool, number_nonblank_lines: bool) -> Result<()> {
    for filename in files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{:6}\t{}", prev_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn echo(text: Vec<String>, omit_newline: bool) -> Result<()> {
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    Ok(())
}

// Utils --------//
pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
