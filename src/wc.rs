use prettytable::{format, Cell, Row, Table};
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
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    let mut headers: Vec<Cell> = Vec::new();

    if enable_table.lines {
        headers.push(Cell::new(HEADERS.lines).style_spec("b"));
    };

    if enable_table.bytes {
        headers.push(Cell::new(HEADERS.bytes).style_spec("b"));
    }

    if enable_table.chars {
        headers.push(Cell::new(HEADERS.chars).style_spec("b"));
    }

    if enable_table.words {
        headers.push(Cell::new(HEADERS.words).style_spec("b"));
    }

    headers.push(Cell::new(HEADERS.file).style_spec("b"));
    table.set_titles(Row::new(headers));

    let mut lines_total: usize = 0;
    let mut bytes_total: usize = 0;
    let mut chars_total: usize = 0;
    let mut words_total: usize = 0;

    for file in files {
        let mut row_values: Vec<Cell> = Vec::new();

        if enable_table.lines {
            let lines_reader = BufReader::new(File::open(file)?);
            let count = lines_reader.lines().count();
            lines_total += count;
            let out = format!("{}", count);
            row_values.push(Cell::new(&out));
        }

        if enable_table.bytes {
            let metadata = metadata(file)?;
            let count = metadata.len() as usize;
            bytes_total += count;
            let out = format!("{}", count);
            row_values.push(Cell::new(&out));
        }

        if enable_table.chars || enable_table.words {
            if enable_table.chars {
                let count = 88; // todo
                chars_total += count;
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
                words_total += count;
                let out = format!("{}", count);
                row_values.push(Cell::new(&out));
            }
        }

        let fileout = format!("{}", file.display());
        row_values.push(Cell::new(&fileout));
        table.add_row(Row::new(row_values));
    }

    let mut totals: Vec<Cell> = Vec::new();

    if enable_table.lines {
        let lines_total_out = format!("{}", lines_total);
        totals.push(Cell::new(&lines_total_out).style_spec("bFg"));
    };

    if enable_table.bytes {
        let bytes_total_out = format!("{}", bytes_total);
        totals.push(Cell::new(&bytes_total_out).style_spec("bFg"));
    }

    if enable_table.chars {
        let chars_total_out = format!("{}", chars_total);
        totals.push(Cell::new(&chars_total_out).style_spec("bFg"));
    }

    if enable_table.words {
        let words_total_out = format!("{}", words_total);
        totals.push(Cell::new(&words_total_out).style_spec("bFg"));
    }

    let total_out = "total";
    totals.push(Cell::new(&total_out).style_spec("bFg"));

    table.add_row(Row::new(totals));

    table.printstd();
    Ok(())
}
