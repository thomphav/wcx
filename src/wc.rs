use prettytable::{Cell, Row, Table};
use std::fs::{metadata, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

struct TableHeaders {
    lines: &'static str,
    bytes: &'static str,
    words: &'static str,
    chars: &'static str,
    file: &'static str,
}

const HEADERS: TableHeaders = TableHeaders {
    lines: "Lines",
    bytes: "Bytes",
    words: "Words",
    chars: "Chars",
    file: "File",
};

#[derive(Default)]
struct EnableTable {
    lines: bool,
    bytes: bool,
    words: bool,
    chars: bool,
}

pub fn invoke(
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
    files: &Vec<PathBuf>,
) -> anyhow::Result<()> {
    let mut enable_table: EnableTable = Default::default();
    let default = !bytes && !lines && !chars && !words;

    if lines || default {
        enable_table.lines = true;
    };

    if chars {
        enable_table.chars = true;
    } else if bytes || default {
        enable_table.bytes = true;
    }

    if words || default {
        enable_table.words = true;
    }

    assert!(!(enable_table.bytes & enable_table.chars));

    let mut table = Table::new();
    let mut headers: Vec<Cell> = Vec::new();

    if enable_table.lines {
        headers.push(Cell::new(HEADERS.lines));
    };

    if enable_table.bytes {
        headers.push(Cell::new(HEADERS.bytes));
    }

    if enable_table.chars {
        headers.push(Cell::new(HEADERS.chars));
    }

    if enable_table.words {
        headers.push(Cell::new(HEADERS.words));
    }

    headers.push(Cell::new(HEADERS.file));
    table.add_row(Row::new(headers));

    for file in files {
        let mut row_values: Vec<Cell> = Vec::new();

        if enable_table.lines {
            let lines_reader = BufReader::new(File::open(file)?);
            let count = lines_reader.lines().count();
            let out = format!("{}", count);
            row_values.push(Cell::new(&out));
        }

        if enable_table.bytes {
            let metadata = metadata(file)?;
            let count = metadata.len() as usize;
            let out = format!("{}", count);
            row_values.push(Cell::new(&out));
        }

        if enable_table.chars || enable_table.words {
            if enable_table.chars {
                let count = 88; // todo
                let out = format!("{}", count);
                row_values.push(Cell::new(&out));
            }

            if enable_table.words {
                let mut words_reader = BufReader::new(File::open(file)?);
                let mut buffer = Vec::new();
                let mut count = 0;
                while words_reader.read_until(b' ', &mut buffer)? != 0 {
                    count += 1;
                }
                let out = format!("{}", count);
                row_values.push(Cell::new(&out));
            }
        }

        let fileout = format!("{}", file.display());
        row_values.push(Cell::new(&fileout));
        table.add_row(Row::new(row_values));
    }

    table.printstd();
    Ok(())
}
