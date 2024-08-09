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

    if word_exists {
        count += 1;
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
            let count = count_lines_in_file(file)?;

            lines_total += count;
            let out = format!("{}", count);
            row_values.push(Cell::new(&out));
        }

        if enable_table.bytes {
            let count = count_bytes_in_file(file)?;

            bytes_total += count;
            let out = format!("{}", count);
            row_values.push(Cell::new(&out));
        }

        if enable_table.chars || enable_table.words {
            if enable_table.chars {
                let count = count_chars_in_file(file)?;

                chars_total += count;
                let out = format!("{}", count);
                row_values.push(Cell::new(&out));
            }

            if enable_table.words {
                let count = count_words_in_file(file)?;

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
