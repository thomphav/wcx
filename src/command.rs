use crate::analyze::{analyze_file, FileResult};
use prettytable::{
    format::{self, TableFormat},
    Cell, Row, Table,
};
use std::path::PathBuf;

/// Used to keep track of the String titles that the TableManager will insert into the header row
/// of the prettyTable::Table
struct TableHeaders {
    lines: String,
    bytes: String,
    words: String,
    chars: String,
    file: String,
}

/// Adds up total counts for each wcx flag enabled.
///
///
/// New instances of `TotalsCounter` are obtained via [`TotalsCounter::new(files_len)`], where `files_len` is the number of
/// files being provided to the TableManager. The TotalsCounter will be set to 'enabled' if more than one file is provided.

///
/// See function level documentation for details on the various configuration
/// settings.
///
/// [`build`]: method@Self::add_to_totals
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

/// Builds TableManager with provided enable flags and prettytable::format::TableFormat configuration.
///
/// Methods can be chained in order to set the configuration values. The
/// TableManager is constructed by calling [`build(files_len)`], where `files_len` is the number of
/// files being provided to the TableManager. The TableManager will sum up totals in
/// the last row of the table if more than one file is provided.
///
/// New instances of `Builder` are obtained via [`Builder::new`]
///
/// # Example
///
/// ```
///    // build table manager
///    let table_manager = Builder::new()
///        .enable_flags(false, true, true, false)
///        .table_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR)
///        .build(files_len)
/// ```
pub struct Builder {
    lines_enabled: bool,
    bytes_enabled: bool,
    words_enabled: bool,
    chars_enabled: bool,
    table_format: Option<TableFormat>,
}

impl Builder {
    /// Returns a new builder.
    ///
    /// Configuration methods can be chained on the return value.
    pub fn new() -> Builder {
        Builder {
            lines_enabled: false,
            bytes_enabled: false,
            words_enabled: false,
            chars_enabled: false,
            table_format: None,
        }
    }

    /// Enables each of the four wcx count modes onto the TableManager.
    ///
    /// lines_enabled: The number of lines in each input file is written to the table.
    /// bytes_enabled: The number of bytes in each input file is written to the table.
    /// chars_enabled: The number of characters in each input file is written to the table.
    /// words_enablec: The number of words in each input file is written to the table.
    ///
    /// If no flags are provided, each of the count modes will be written to the table.
    pub fn enable_flags(
        &mut self,
        lines_enabled: bool,
        bytes_enabled: bool,
        chars_enabled: bool,
        words_enabled: bool,
    ) -> &mut Self {
        let default: bool = !lines_enabled && !bytes_enabled && !chars_enabled && !words_enabled;

        self.lines_enabled = lines_enabled || default;
        self.bytes_enabled = bytes_enabled || default;
        self.chars_enabled = chars_enabled || default;
        self.words_enabled = words_enabled || default;
        self
    }

    /// Updates table format configuration value, which will be updated onto the actual table once
    /// Builder::build is called
    pub fn table_format(&mut self, table_format: TableFormat) -> &mut Self {
        self.table_format = Some(table_format);
        self
    }

    /// Creates the `Table` and sets its format if one was provided. Uses the enable flags to
    /// correctly insert titles into the header row of the `Table`.
    ///
    /// Then creates the configured `TableManager`, which houses the enable flags, `Table`, and
    /// `TotalsCounter`. The returned `TableManager` can now be used to add more rows to the table.
    ///
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

    for file in files {
        table_manager.set_table_row(file)?;
    }

    if table_manager.totals_counter.enabled {
        table_manager.set_table_totals();
    }

    table_manager.print_table();
    Ok(())
}
