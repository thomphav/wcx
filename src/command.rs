use crate::analyze::{analyze_file, FileResult};
use prettytable::{
    format::{self, TableFormat},
    Cell, Row, Table,
};
use std::path::PathBuf;

struct TableHeaders {
    lines: String,
    bytes: String,
    words: String,
    chars: String,
    file: String,
}

pub struct TotalsCounter {
    enabled: bool,
    lines_total: usize,
    bytes_total: usize,
    chars_total: usize,
    words_total: usize,
}

impl TotalsCounter {
    pub fn new(files: &Vec<PathBuf>) -> TotalsCounter {
        TotalsCounter {
            enabled: files.len() > 1,
            lines_total: 0,
            bytes_total: 0,
            chars_total: 0,
            words_total: 0,
        }
    }
}

pub struct Builder {
    lines_enabled: bool,
    bytes_enabled: bool,
    words_enabled: bool,
    chars_enabled: bool,
    table_format: Option<TableFormat>,
    table: Table,
    totals_counter: TotalsCounter,
}

impl Builder {
    pub fn new(files: &Vec<PathBuf>) -> Builder {
        Builder {
            lines_enabled: false,
            bytes_enabled: false,
            words_enabled: false,
            chars_enabled: false,
            table_format: None,
            table: Table::new(),
            totals_counter: TotalsCounter::new(files),
        }
    }

    pub fn lines_enabled(&mut self, lines_enabled: bool) -> &mut Self {
        self.lines_enabled = lines_enabled;
        self
    }

    pub fn bytes_enabled(&mut self, bytes_enabled: bool) -> &mut Self {
        self.bytes_enabled = bytes_enabled;
        self
    }

    pub fn chars_enabled(&mut self, chars_enabled: bool) -> &mut Self {
        self.chars_enabled = chars_enabled;
        self
    }

    pub fn words_enabled(&mut self, words_enabled: bool) -> &mut Self {
        self.words_enabled = words_enabled;
        self
    }

    pub fn table_format(&mut self, table_format: TableFormat) -> &mut Self {
        self.table_format = Some(table_format);
        self
    }

    pub fn build(&mut self) -> TableManager {
        if let Some(table_format) = self.table_format {
            self.table.set_format(table_format);
        }

        let headers: TableHeaders = TableHeaders {
            lines: String::from("Lines"),
            bytes: String::from("Bytes"),
            words: String::from("Words"),
            chars: String::from("Chars"),
            file: String::from("File"),
        };

        let mut headers_buffer: Vec<Cell> = Vec::new();

        if self.lines_enabled {
            headers_buffer.push(Cell::new(&headers.lines).style_spec("b"));
        };

        if self.bytes_enabled {
            headers_buffer.push(Cell::new(&headers.bytes).style_spec("b"));
        }

        if self.chars_enabled {
            headers_buffer.push(Cell::new(&headers.chars).style_spec("b"));
        }

        if self.words_enabled {
            headers_buffer.push(Cell::new(&headers.words).style_spec("b"));
        }

        headers_buffer.push(Cell::new(&headers.file).style_spec("b"));
        self.table.set_titles(Row::new(headers_buffer));

        TableManager {
            lines_enabled: self.lines_enabled,
            bytes_enabled: self.bytes_enabled,
            chars_enabled: self.bytes_enabled,
            words_enabled: self.bytes_enabled,
            table: self.table,
            totals_counter: self.totals_counter,
        }
    }
}

pub struct TableManager {
    pub lines_enabled: bool,
    pub bytes_enabled: bool,
    pub words_enabled: bool,
    pub chars_enabled: bool,
    pub table: Table,
    pub totals_counter: TotalsCounter,
}

pub fn invoke(
    lines_enabled: bool,
    bytes_enabled: bool,
    chars_enabled: bool,
    words_enabled: bool,
    files: &Vec<PathBuf>,
) -> anyhow::Result<()> {
    let format = *format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR;

    let mut table_manager = Builder::new(files)
        .lines_enabled(lines_enabled)
        .bytes_enabled(bytes_enabled)
        .chars_enabled(chars_enabled)
        .words_enabled(words_enabled)
        .table_format(format)
        .build();

    assert!(!(table_manager.bytes_enabled & table_manager.chars_enabled));

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

fn set_enable_table(
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
    enable_table: &mut TableManager,
) {
    let default = !bytes && !lines && !chars && !words;

    enable_table.lines = lines || default;
    enable_table.chars = chars;
    enable_table.bytes = (bytes || default) && !chars;
    enable_table.words = words || default;
}

fn set_table_headers(table: &mut Table, enable_table: &TableManager, headers: &TableHeaders) {
    let mut headers_buffer: Vec<Cell> = Vec::new();

    if enable_table.lines {
        headers_buffer.push(Cell::new(&headers.lines).style_spec("b"));
    };

    if enable_table.bytes {
        headers_buffer.push(Cell::new(&headers.bytes).style_spec("b"));
    }

    if enable_table.chars {
        headers_buffer.push(Cell::new(&headers.chars).style_spec("b"));
    }

    if enable_table.words {
        headers_buffer.push(Cell::new(&headers.words).style_spec("b"));
    }

    headers_buffer.push(Cell::new(&headers.file).style_spec("b"));
    table.set_titles(Row::new(headers_buffer));
}

fn push_row(count: &usize, row_values: &mut Vec<Cell>) {
    let out = format!("{}", *count);
    row_values.push(Cell::new(&out));
}

fn set_row_values(
    enable_table: &TableManager,
    row_values: &mut Vec<Cell>,
    file_result: &FileResult,
) -> anyhow::Result<()> {
    if enable_table.lines {
        push_row(&file_result.lines, row_values);
    }

    if enable_table.bytes {
        push_row(&file_result.bytes, row_values);
    }

    if enable_table.chars {
        push_row(&file_result.chars, row_values);
    }

    if enable_table.words {
        push_row(&file_result.words, row_values);
    }

    Ok(())
}

fn set_table_row(
    file: &PathBuf,
    enable_table: &TableManager,
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
    enable_table: &TableManager,
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
