use crate::analyze::{analyze_file, FileResult};
use prettytable::{format, Cell, Row, Table};
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
pub struct EnableTable {
    pub lines: bool,
    pub bytes: bool,
    pub words: bool,
    pub chars: bool,
}

fn set_enable_table(
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
    enable_table: &mut EnableTable,
) {
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
}

fn set_table_format(table: &mut Table) {
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
}

fn set_table_headers(table: &mut Table, enable_table: &EnableTable) {
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
}

fn set_row_values(
    enable_table: &EnableTable,
    row_values: &mut Vec<Cell>,
    file_result: &FileResult,
) -> anyhow::Result<()> {
    if enable_table.lines {
        let count = file_result.lines;

        let out = format!("{}", count);
        row_values.push(Cell::new(&out));
    }

    if enable_table.bytes {
        let count = file_result.bytes;

        let out = format!("{}", count);
        row_values.push(Cell::new(&out));
    }

    if enable_table.chars {
        let count = file_result.chars;

        let out = format!("{}", count);
        row_values.push(Cell::new(&out));
    }

    if enable_table.words {
        let count = file_result.words;

        let out = format!("{}", count);
        row_values.push(Cell::new(&out));
    }

    Ok(())
}

fn set_table_row(
    file: &PathBuf,
    enable_table: &EnableTable,
    table: &mut Table,
    lines_total: &mut usize,
    bytes_total: &mut usize,
    chars_total: &mut usize,
    words_total: &mut usize,
    totals_enabled: &bool,
) -> anyhow::Result<()> {
    let mut row_values: Vec<Cell> = Vec::new();

    let file_result = analyze_file(file, &enable_table)?;

    if *totals_enabled {
        *lines_total += file_result.lines;
        *bytes_total += file_result.bytes;
        *chars_total += file_result.chars;
        *words_total += file_result.words;
    }

    set_row_values(enable_table, &mut row_values, &file_result)?;

    let filename = format!("{}", file.display());
    row_values.push(Cell::new(&filename));
    table.add_row(Row::new(row_values));

    Ok(())
}

fn set_table_totals(
    table: &mut Table,
    enable_table: &EnableTable,
    lines_total: &usize,
    bytes_total: &usize,
    chars_total: &usize,
    words_total: &usize,
) {
    let mut totals: Vec<Cell> = Vec::new();

    if enable_table.lines {
        let lines_total_out = format!("{}", *lines_total);
        totals.push(Cell::new(&lines_total_out).style_spec("bFg"));
    };

    if enable_table.bytes {
        let bytes_total_out = format!("{}", *bytes_total);
        totals.push(Cell::new(&bytes_total_out).style_spec("bFg"));
    }

    if enable_table.chars {
        let chars_total_out = format!("{}", *chars_total);
        totals.push(Cell::new(&chars_total_out).style_spec("bFg"));
    }

    if enable_table.words {
        let words_total_out = format!("{}", *words_total);
        totals.push(Cell::new(&words_total_out).style_spec("bFg"));
    }

    let total_out = "total";
    totals.push(Cell::new(&total_out).style_spec("bFg"));

    table.add_row(Row::new(totals));
}

pub fn invoke(
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
    files: &Vec<PathBuf>,
) -> anyhow::Result<()> {
    let mut enable_table: EnableTable = Default::default();
    set_enable_table(bytes, lines, chars, words, &mut enable_table);

    assert!(!(enable_table.bytes & enable_table.chars));

    let mut table = Table::new();
    set_table_format(&mut table);
    set_table_headers(&mut table, &enable_table);

    let mut lines_total: usize = 0;
    let mut bytes_total: usize = 0;
    let mut chars_total: usize = 0;
    let mut words_total: usize = 0;
    let totals_enabled: bool = files.len() > 1;

    for file in files {
        set_table_row(
            file,
            &enable_table,
            &mut table,
            &mut lines_total,
            &mut bytes_total,
            &mut chars_total,
            &mut words_total,
            &totals_enabled,
        )?;
    }

    if totals_enabled {
        set_table_totals(
            &mut table,
            &enable_table,
            &mut lines_total,
            &mut bytes_total,
            &mut chars_total,
            &mut words_total,
        );
    }

    table.printstd();
    Ok(())
}
