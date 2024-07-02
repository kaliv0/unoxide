use anyhow::{bail, Result};
use std::{cmp::Ordering::*, io::BufRead};

use super::helpers::file_reader::open_file_map_err;

pub enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

pub fn comm(
    file_1: &str,
    file_2: &str,
    show_col_1: bool,
    show_col_2: bool,
    show_col_3: bool,
    ignore_case: bool,
    delimiter: &str,
) -> Result<()> {
    validate_file_name(file_1, file_2)?;

    let mut file_1_lines = prepare_line_iterator(file_1, ignore_case)?;
    let mut file_2_lines = prepare_line_iterator(file_2, ignore_case)?;

    let log = log_data(show_col_1, show_col_2, show_col_3, delimiter);

    let mut line_1 = file_1_lines.next();
    let mut line_2 = file_2_lines.next();
    while line_1.is_some() || line_2.is_some() {
        match (&line_1, &line_2) {
            (Some(val_1), Some(val_2)) => match val_1.cmp(val_2) {
                Equal => {
                    log(Column::Col3(val_1));
                    line_1 = file_1_lines.next();
                    line_2 = file_2_lines.next();
                }
                Less => {
                    log(Column::Col1(val_1));
                    line_1 = file_1_lines.next();
                }
                Greater => {
                    log(Column::Col2(val_2));
                    line_2 = file_2_lines.next();
                }
            },
            (Some(val_1), None) => {
                log(Column::Col1(val_1));
                line_1 = file_1_lines.next();
            }
            (None, Some(val_2)) => {
                log(Column::Col2(val_2));
                line_2 = file_2_lines.next();
            }
            _ => (),
        }
    }
    Ok(())
}

//-------------
fn validate_file_name(file_1: &str, file_2: &str) -> Result<()> {
    if file_1 == "-" && file_2 == "-" {
        bail!(r#"comm: input files cannot be both STDIN ("-")"#);
    }
    Ok(())
}

fn log_data(
    show_col_1: bool,
    show_col_2: bool,
    show_col_3: bool,
    delimiter: &str,
) -> impl for<'a> Fn(Column<'a>) + '_ {
    // TODO: refactor
    move |col: Column| {
        let mut columns = vec![];
        match col {
            Column::Col1(val) => {
                if show_col_1 {
                    columns.push(val);
                }
            }
            Column::Col2(val) => {
                if show_col_2 {
                    if show_col_1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Column::Col3(val) => {
                if show_col_3 {
                    if show_col_1 {
                        columns.push("");
                    }
                    if show_col_2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };
        if !columns.is_empty() {
            println!("{}", columns.join(delimiter));
        }
    }
}

fn prepare_line_iterator(file: &str, ignore_case: bool) -> Result<impl Iterator<Item = String>> {
    let lines = open_file_map_err(file, "comm")? //TODO: extract const
        .lines()
        .map_while(Result::ok)
        .map(move |line: String| {
            if ignore_case {
                line.to_lowercase()
            } else {
                line
            }
        });
    Ok(lines)
}
