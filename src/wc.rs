use prettytable::{format, Cell, Row, Table};
use std::fs::{metadata, File};
use std::io::{BufRead, BufReader, Read};
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

#[derive(Default)]
struct FileResult {
    lines: usize,
    bytes: usize,
    words: usize,
    chars: usize,
}

fn count_bytes_in_file(file: &PathBuf) -> anyhow::Result<usize> {
    let metadata = metadata(file)?;
    let count = metadata.len() as usize;

    return Ok(count);
}

fn count_lines_in_file(file: &PathBuf) -> anyhow::Result<usize> {
    let lines_reader = BufReader::new(File::open(file)?);
    let count = lines_reader.lines().count();

    return Ok(count);
}

fn count_words_in_file(file: &PathBuf) -> anyhow::Result<usize> {
    let mut words_reader = BufReader::new(File::open(file)?);
    let mut byte = [0; 1];
    let mut word_exists: bool = false;
    let mut count = 0;
    let delimiters = b"' '\n\t";
    while words_reader.read(&mut byte)? != 0 {
        if delimiters.contains(&byte[0]) {
            if word_exists {
                count += 1;
                word_exists = false;
                continue;
            }
        }

        if word_exists == false {
            word_exists = true;
        }
    }

    return Ok(count);
}

fn count_chars_in_file(file: &PathBuf) -> anyhow::Result<usize> {
    let mut chars_reader = BufReader::new(File::open(file)?);
    let mut buffer = Vec::new();
    chars_reader.read_to_end(&mut buffer)?;
    let decoded_string = String::from_utf8_lossy(&buffer);
    let count = decoded_string.chars().count();

    return Ok(count);
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

fn analyze_file(file: &PathBuf, enable_table: &EnableTable) -> anyhow::Result<FileResult> {
    let mut file_result: FileResult = Default::default();

    if enable_table.lines {
        let count = count_lines_in_file(file)?;
        file_result.lines = count;
    }

    if enable_table.bytes {
        let count = count_bytes_in_file(file)?;
        file_result.bytes = count;
    }

    if enable_table.chars {
        let count = count_chars_in_file(file)?;
        file_result.chars = count;
    }

    if enable_table.words {
        let count = count_words_in_file(file)?;
        file_result.words = count;
    }

    Ok(file_result)
}

fn set_row_values(
    file: &PathBuf,
    enable_table: &EnableTable,
    row_values: &mut Vec<Cell>,
) -> anyhow::Result<()> {
    if enable_table.lines {
        let count = count_lines_in_file(file)?;

        let out = format!("{}", count);
        row_values.push(Cell::new(&out));
    }

    if enable_table.bytes {
        let count = count_bytes_in_file(file)?;

        let out = format!("{}", count);
        row_values.push(Cell::new(&out));
    }

    if enable_table.chars {
        let count = count_chars_in_file(file)?;

        let out = format!("{}", count);
        row_values.push(Cell::new(&out));
    }

    if enable_table.words {
        let count = count_words_in_file(file)?;

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

    set_row_values(file, enable_table, &mut row_values)?;

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

#[test]
fn test_count_bytes_in_test_1() {
    let test_file_path = PathBuf::from("tests/data/test_1.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 449);
}

#[test]
fn test_count_lines_in_test_1() {
    let test_file_path = PathBuf::from("tests/data/test_1.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 1);
}

#[test]
fn test_count_words_in_test_1() {
    let test_file_path = PathBuf::from("tests/data/test_1.txt");
    let word_count = count_words_in_file(&test_file_path).expect("Failed to count words in file");

    assert_eq!(word_count, 70);
}

#[test]
fn test_count_chars_in_test_1() {
    let test_file_path = PathBuf::from("tests/data/test_1.txt");
    let char_count = count_chars_in_file(&test_file_path).expect("Failed to count chars in file");

    assert_eq!(char_count, 449);
}

#[test]
fn test_count_bytes_in_test_2() {
    let test_file_path = PathBuf::from("tests/data/test_2.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 3);
}

#[test]
fn test_count_lines_in_test_2() {
    let test_file_path = PathBuf::from("tests/data/test_2.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 1);
}

#[test]
fn test_count_words_in_test_2() {
    let test_file_path = PathBuf::from("tests/data/test_2.txt");
    let word_count = count_words_in_file(&test_file_path).expect("Failed to count words in file");

    assert_eq!(word_count, 1);
}

#[test]
fn test_count_chars_in_test_2() {
    let test_file_path = PathBuf::from("tests/data/test_2.txt");
    let char_count = count_chars_in_file(&test_file_path).expect("Failed to count chars in file");

    assert_eq!(char_count, 2);
}

#[test]
fn test_count_bytes_in_test_3() {
    let test_file_path = PathBuf::from("tests/data/test_3.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 0);
}

#[test]
fn test_count_lines_in_test_3() {
    let test_file_path = PathBuf::from("tests/data/test_3.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 0);
}

#[test]
fn test_count_words_in_test_3() {
    let test_file_path = PathBuf::from("tests/data/test_3.txt");
    let word_count = count_words_in_file(&test_file_path).expect("Failed to count words in file");

    assert_eq!(word_count, 0);
}

#[test]
fn test_count_chars_in_test_3() {
    let test_file_path = PathBuf::from("tests/data/test_3.txt");
    let char_count = count_chars_in_file(&test_file_path).expect("Failed to count chars in file");

    assert_eq!(char_count, 0);
}

#[test]
fn test_count_bytes_in_test_4() {
    let test_file_path = PathBuf::from("tests/data/test_4.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 125);
}

#[test]
fn test_count_lines_in_test_4() {
    let test_file_path = PathBuf::from("tests/data/test_4.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 6);
}

#[test]
fn test_count_words_in_test_4() {
    let test_file_path = PathBuf::from("tests/data/test_4.txt");
    let word_count = count_words_in_file(&test_file_path).expect("Failed to count words in file");

    assert_eq!(word_count, 16);
}

#[test]
fn test_count_chars_in_test_4() {
    let test_file_path = PathBuf::from("tests/data/test_4.txt");
    let char_count = count_chars_in_file(&test_file_path).expect("Failed to count chars in file");

    assert_eq!(char_count, 83);
}
