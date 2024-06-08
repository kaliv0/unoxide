use crate::utils::reader::open;
use anyhow::Result;
use std::io::BufRead;

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
