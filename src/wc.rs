use prettytable::{Cell, Row, Table};
use std::fs::{metadata, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

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
struct FlagData {
    count: usize,
    enabled: bool,
}

#[derive(Default)]
struct Data {
    bytes: FlagData,
    lines: FlagData,
    chars: FlagData,
    words: FlagData,
}

pub fn invoke(
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
    file: &Path,
) -> anyhow::Result<()> {
    let default = !bytes && !lines && !chars && !words;
    let metadata = metadata(file)?;

    let mut data: Data = Default::default();

    if bytes || default {
        data.bytes.enabled = true;
        data.bytes.count = metadata.len() as usize;
    }

    if lines || chars || words || default {
        let reader = BufReader::new(File::open(file)?);

        if lines || default {
            data.lines.enabled = true;
            data.lines.count = reader.lines().count();
        }

        let mut reader = BufReader::new(File::open(file)?);

        // for byte in reader.bytes() {
        if chars && !bytes {
            data.chars.enabled = true;
            data.chars.count = 88;
        }

        if words || default {
            let mut buffer = Vec::new();
            let mut inc = 0;
            while reader.read_until(b' ', &mut buffer)? != 0 {
                inc += 1;
            }
            data.words.enabled = true;
            data.words.count = inc;
        }
        // }
    }

    assert!(!(data.bytes.enabled & data.chars.enabled));

    let mut header: Vec<Cell> = Vec::new();
    let mut values: Vec<Cell> = Vec::new();
    let mut table = Table::new();

    if data.lines.enabled {
        let out = format!("{}", data.lines.count);
        header.push(Cell::new(HEADERS.lines));
        values.push(Cell::new(&out));
    }

    if data.words.enabled {
        let out = format!("{}", data.words.count);
        header.push(Cell::new(HEADERS.words));
        values.push(Cell::new(&out));
    }

    if data.bytes.enabled {
        let out = format!("{}", data.bytes.count);
        header.push(Cell::new(HEADERS.bytes));
        values.push(Cell::new(&out));
    }

    if data.chars.enabled {
        let out = format!("{}", data.chars.count);
        header.push(Cell::new(HEADERS.chars));
        values.push(Cell::new(&out));
    }

    let fileout = format!("{}", file.display());
    header.push(Cell::new(HEADERS.file));
    values.push(Cell::new(&fileout));

    table.add_row(Row::new(header));
    table.add_row(Row::new(values));
    table.printstd();
    Ok(())
}
