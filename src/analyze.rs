use std::fs::{metadata, read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Default)]
pub struct FileResult {
    pub lines: usize,
    pub bytes: usize,
    pub words: usize,
    pub chars: usize,
}

pub fn analyze_file(
    file: &PathBuf,
    lines_enabled: bool,
    bytes_enabled: bool,
    chars_enabled: bool,
    words_enabled: bool,
) -> anyhow::Result<FileResult> {
    let mut file_result: FileResult = Default::default();

    if lines_enabled {
        let count = count_lines_in_file(file)?;
        file_result.lines = count;
    }

    if bytes_enabled {
        let count = count_bytes_in_file(file)?;
        file_result.bytes = count;
    }

    if chars_enabled {
        let count = count_chars_in_file(file);
        file_result.chars = count;
    }

    if words_enabled {
        let count = count_words_in_file(file);
        file_result.words = count;
    }

    Ok(file_result)
}

fn count_bytes_in_file(file: &PathBuf) -> anyhow::Result<usize> {
    let metadata = metadata(file)?;
    let count = usize::try_from(metadata.len())?;

    return Ok(count);
}

fn count_lines_in_file(file: &PathBuf) -> anyhow::Result<usize> {
    let lines_reader = BufReader::new(File::open(file)?);
    let count = lines_reader.lines().count();

    return Ok(count);
}

fn count_chars_in_file(file: &PathBuf) -> usize {
    let decoded_string = read_to_string(file).expect(
        "Failed to read file. Note: character count (`-m`) only works with valid UTF-8 encoded files.",
    );
    let count = decoded_string.chars().count();

    return count;
}

fn count_words_in_file(file: &PathBuf) -> usize {
    let decoded_string = read_to_string(file)
        .expect("Failed to read file. Note: word count (`-w`) only works with valid UTF-8 files.");
    let count = decoded_string.split_whitespace().count();

    return count;
}

#[test]
fn test_count_bytes_in_test_1() {
    let test_file_path = PathBuf::from("assets/test_1.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 449);
}

#[test]
fn test_count_lines_in_test_1() {
    let test_file_path = PathBuf::from("assets/test_1.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 1);
}

#[test]
fn test_count_words_in_test_1() {
    let test_file_path = PathBuf::from("assets/test_1.txt");
    let word_count = count_words_in_file(&test_file_path);

    assert_eq!(word_count, 70);
}

#[test]
fn test_count_chars_in_test_1() {
    let test_file_path = PathBuf::from("assets/test_1.txt");
    let char_count = count_chars_in_file(&test_file_path);

    assert_eq!(char_count, 449);
}

#[test]
fn test_count_bytes_in_test_2() {
    let test_file_path = PathBuf::from("assets/test_2.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 3);
}

#[test]
fn test_count_lines_in_test_2() {
    let test_file_path = PathBuf::from("assets/test_2.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 1);
}

#[test]
fn test_count_words_in_test_2() {
    let test_file_path = PathBuf::from("assets/test_2.txt");
    let word_count = count_words_in_file(&test_file_path);

    assert_eq!(word_count, 1);
}

#[test]
fn test_count_chars_in_test_2() {
    let test_file_path = PathBuf::from("assets/test_2.txt");
    let char_count = count_chars_in_file(&test_file_path);

    assert_eq!(char_count, 2);
}

#[test]
fn test_count_bytes_in_test_3() {
    let test_file_path = PathBuf::from("assets/test_3.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 0);
}

#[test]
fn test_count_lines_in_test_3() {
    let test_file_path = PathBuf::from("assets/test_3.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 0);
}

#[test]
fn test_count_words_in_test_3() {
    let test_file_path = PathBuf::from("assets/test_3.txt");
    let word_count = count_words_in_file(&test_file_path);

    assert_eq!(word_count, 0);
}

#[test]
fn test_count_chars_in_test_3() {
    let test_file_path = PathBuf::from("assets/test_3.txt");
    let char_count = count_chars_in_file(&test_file_path);

    assert_eq!(char_count, 0);
}

#[test]
fn test_count_bytes_in_test_4() {
    let test_file_path = PathBuf::from("assets/test_4.txt");
    let byte_count = count_bytes_in_file(&test_file_path).expect("Failed to count bytes in file");

    assert_eq!(byte_count, 125);
}

#[test]
fn test_count_lines_in_test_4() {
    let test_file_path = PathBuf::from("assets/test_4.txt");
    let line_count = count_lines_in_file(&test_file_path).expect("Failed to count lines in file");

    assert_eq!(line_count, 6);
}

#[test]
fn test_count_words_in_test_4() {
    let test_file_path = PathBuf::from("assets/test_4.txt");
    let word_count = count_words_in_file(&test_file_path);

    assert_eq!(word_count, 15);
}

#[test]
fn test_count_chars_in_test_4() {
    let test_file_path = PathBuf::from("assets/test_4.txt");
    let char_count = count_chars_in_file(&test_file_path);

    assert_eq!(char_count, 83);
}
