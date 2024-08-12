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
    pub fn new(files_len: usize) -> TotalsCounter {
        TotalsCounter {
            enabled: files_len > 1,
            lines_total: 0,
            bytes_total: 0,
            chars_total: 0,
            words_total: 0,
        }
    }

    pub fn add_to_totals(&mut self, file_result: &FileResult) {
        self.lines_total += file_result.lines;
        self.bytes_total += file_result.bytes;
        self.chars_total += file_result.chars;
        self.words_total += file_result.words;
    }
}

pub struct Builder {
    lines_enabled: bool,
    bytes_enabled: bool,
    words_enabled: bool,
    chars_enabled: bool,
    table_format: Option<TableFormat>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            lines_enabled: false,
            bytes_enabled: false,
            words_enabled: false,
            chars_enabled: false,
            table_format: None,
        }
    }

    pub fn enable_flags(
        &mut self,
        lines_enabled: bool,
        bytes_enabled: bool,
        chars_enabled: bool,
        words_enabled: bool,
    ) -> &mut Self {
        let default: bool = !lines_enabled && !bytes_enabled && !chars_enabled && !words_enabled;

        self.lines_enabled = lines_enabled || default;
        self.bytes_enabled = (bytes_enabled || default) && !chars_enabled;
        self.chars_enabled = chars_enabled;
        self.words_enabled = words_enabled || default;
        self
    }

    pub fn table_format(&mut self, table_format: TableFormat) -> &mut Self {
        self.table_format = Some(table_format);
        self
    }

    pub fn build(&mut self, files_len: usize) -> TableManager {
        let totals_counter: TotalsCounter = TotalsCounter::new(files_len);

        let mut table: Table = Table::new();

        if let Some(table_format) = self.table_format {
            table.set_format(table_format);
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
        table.set_titles(Row::new(headers_buffer));

        TableManager {
            lines_enabled: self.lines_enabled,
            bytes_enabled: self.bytes_enabled,
            chars_enabled: self.chars_enabled,
            words_enabled: self.words_enabled,
            table,
            totals_counter,
        }
    }
}

pub struct TableManager {
    pub lines_enabled: bool,
    pub bytes_enabled: bool,
    pub chars_enabled: bool,
    pub words_enabled: bool,
    pub table: Table,
    pub totals_counter: TotalsCounter,
}

impl TableManager {
    pub fn set_table_row(&mut self, file: &PathBuf) -> anyhow::Result<()> {
        let mut row_values: Vec<Cell> = Vec::new();

        let file_result: FileResult = analyze_file(
            file,
            self.lines_enabled,
            self.bytes_enabled,
            self.chars_enabled,
            self.words_enabled,
        )?;

        if self.totals_counter.enabled {
            self.totals_counter.add_to_totals(&file_result);
        }

        self.set_row_values(&mut row_values, file, &file_result);
        self.table.add_row(Row::new(row_values));

        Ok(())
    }

    pub fn set_table_totals(&mut self) {
        let mut totals: Vec<Cell> = Vec::new();

        let TotalsCounter {
            lines_total,
            bytes_total,
            chars_total,
            words_total,
            ..
        } = self.totals_counter;

        if self.lines_enabled {
            Self::push_totals_row_value(&lines_total, &mut totals);
        };

        if self.bytes_enabled {
            Self::push_totals_row_value(&bytes_total, &mut totals);
        }

        if self.chars_enabled {
            Self::push_totals_row_value(&chars_total, &mut totals);
        }

        if self.words_enabled {
            Self::push_totals_row_value(&words_total, &mut totals);
        }

        let total_out = "total";
        totals.push(Cell::new(&total_out).style_spec("bFg"));

        self.table.add_row(Row::new(totals));
    }

    pub fn set_row_values(
        &mut self,
        row_values: &mut Vec<Cell>,
        file: &PathBuf,
        file_result: &FileResult,
    ) {
        if self.lines_enabled {
            Self::push_row_value(&file_result.lines, row_values);
        }

        if self.bytes_enabled {
            Self::push_row_value(&file_result.bytes, row_values);
        }

        if self.chars_enabled {
            Self::push_row_value(&file_result.chars, row_values);
        }

        if self.words_enabled {
            Self::push_row_value(&file_result.words, row_values);
        }

        let filename = format!("{}", file.display());
        row_values.push(Cell::new(&filename));
    }

    pub fn push_row_value(count: &usize, row_values: &mut Vec<Cell>) {
        let out = format!("{}", *count);
        row_values.push(Cell::new(&out));
    }

    pub fn push_totals_row_value(count: &usize, row_values: &mut Vec<Cell>) {
        let out = format!("{}", *count);
        row_values.push(Cell::new(&out).style_spec("bFg"));
    }

    pub fn print_table(&self) {
        self.table.printstd();
    }
}

pub fn invoke(
    lines_enabled: bool,
    bytes_enabled: bool,
    chars_enabled: bool,
    words_enabled: bool,
    files: &Vec<PathBuf>,
) -> anyhow::Result<()> {
    let format = *format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR;

    let mut table_manager = Builder::new()
        .enable_flags(lines_enabled, bytes_enabled, chars_enabled, words_enabled)
        .table_format(format)
        .build(files.len());

    assert!(!(table_manager.bytes_enabled & table_manager.chars_enabled));

    for file in files {
        table_manager.set_table_row(file)?;
    }

    if table_manager.totals_counter.enabled {
        table_manager.set_table_totals();
    }

    table_manager.print_table();
    Ok(())
}

// fn set_enable_table(
//     bytes: bool,
//     lines: bool,
//     chars: bool,
//     words: bool,
//     enable_table: &mut TableManager,
// ) {
//     let default = !bytes && !lines && !chars && !words;
//
//     enable_table.lines = lines || default;
//     enable_table.chars = chars;
//     enable_table.bytes = (bytes || default) && !chars;
//     enable_table.words = words || default;
// }
//
// fn set_table_headers(table: &mut Table, enable_table: &TableManager, headers: &TableHeaders) {
//     let mut headers_buffer: Vec<Cell> = Vec::new();
//
//     if enable_table.lines {
//         headers_buffer.push(Cell::new(&headers.lines).style_spec("b"));
//     };
//
//     if enable_table.bytes {
//         headers_buffer.push(Cell::new(&headers.bytes).style_spec("b"));
//     }
//
//     if enable_table.chars {
//         headers_buffer.push(Cell::new(&headers.chars).style_spec("b"));
//     }
//
//     if enable_table.words {
//         headers_buffer.push(Cell::new(&headers.words).style_spec("b"));
//     }
//
//     headers_buffer.push(Cell::new(&headers.file).style_spec("b"));
//     table.set_titles(Row::new(headers_buffer));
// }
//
// fn push_row(count: &usize, row_values: &mut Vec<Cell>) {
//     let out = format!("{}", *count);
//     row_values.push(Cell::new(&out));
// }

// fn set_row_values(
//     enable_table: &TableManager,
//     row_values: &mut Vec<Cell>,
//     file_result: &FileResult,
// ) -> anyhow::Result<()> {
//     if enable_table.lines {
//         push_row(&file_result.lines, row_values);
//     }
//
//     if enable_table.bytes {
//         push_row(&file_result.bytes, row_values);
//     }
//
//     if enable_table.chars {
//         push_row(&file_result.chars, row_values);
//     }
//
//     if enable_table.words {
//         push_row(&file_result.words, row_values);
//     }
//
//     Ok(())
// }
//
// fn set_table_row(
//     file: &PathBuf,
//     enable_table: &TableManager,
//     table: &mut Table,
//     lines_total: &mut usize,
//     bytes_total: &mut usize,
//     chars_total: &mut usize,
//     words_total: &mut usize,
//     totals_enabled: &bool,
// ) -> anyhow::Result<()> {
//     let mut row_values: Vec<Cell> = Vec::new();
//
//     let file_result = analyze_file(file, &enable_table)?;
//
//     if *totals_enabled {
//         *lines_total += file_result.lines;
//         *bytes_total += file_result.bytes;
//         *chars_total += file_result.chars;
//         *words_total += file_result.words;
//     }
//
//     set_row_values(enable_table, &mut row_values, &file_result)?;
//
//     let filename = format!("{}", file.display());
//     row_values.push(Cell::new(&filename));
//     table.add_row(Row::new(row_values));
//
//     Ok(())
// }

// fn set_table_totals(
//     table: &mut Table,
//     enable_table: &TableManager,
//     lines_total: &usize,
//     bytes_total: &usize,
//     chars_total: &usize,
//     words_total: &usize,
// ) {
//     let mut totals: Vec<Cell> = Vec::new();
//
//     if enable_table.lines {
//         let lines_total_out = format!("{}", *lines_total);
//         totals.push(Cell::new(&lines_total_out).style_spec("bFg"));
//     };
//
//     if enable_table.bytes {
//         let bytes_total_out = format!("{}", *bytes_total);
//         totals.push(Cell::new(&bytes_total_out).style_spec("bFg"));
//     }
//
//     if enable_table.chars {
//         let chars_total_out = format!("{}", *chars_total);
//         totals.push(Cell::new(&chars_total_out).style_spec("bFg"));
//     }
//
//     if enable_table.words {
//         let words_total_out = format!("{}", *words_total);
//         totals.push(Cell::new(&words_total_out).style_spec("bFg"));
//     }
//
//     let total_out = "total";
//     totals.push(Cell::new(&total_out).style_spec("bFg"));
//
//     table.add_row(Row::new(totals));
// }
