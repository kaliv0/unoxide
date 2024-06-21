use crate::utils::file_reader;
use anyhow::{anyhow, Ok, Result};
use std::{
    fs::File,
    io::{self, BufRead, Write},
};

pub fn uniq(in_file: &str, out_file: Option<&str>, show_count: bool) -> Result<()> {
    // refactor as other subcommands with match statements?
    // probably used here with closure to avoid packing everything inside Ok match arm?!
    let mut file = file_reader::open(in_file).map_err(|e| anyhow!("{in_file}: {e}"))?;

    let mut output_file: Box<dyn Write> = match out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    // let mut print = |num: u64, text: &str| -> Result<()> {
    //     if num > 0 {
    //         if show_count {
    //             write!(output_file, "{num:>4} {text}")?;
    //         } else {
    //             write!(output_file, "{text}")?;
    //         }
    //     };
    //     Ok(())
    // };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            // output_file = log_data(show_count, output_file, count, &previous)?; //fix
            write!(output_file, "{}", format_data(show_count, count, &previous))?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    // print(count, &previous)?;
    write!(output_file, "{}", format_data(show_count, count, &previous))?;
    // log_data(show_count, output_file, count, &previous)?;
    Ok(())
}

//----------------------
fn format_data(flag: bool, count: u64, text: &str) -> String {
    if count > 0 {
        // move outside function?
        if flag {
            format!("{count:>4} {text}")
        } else {
            format!("{text}")
        }
    } else {
        "".to_string()
    }
}
