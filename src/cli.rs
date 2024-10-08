use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Count number of lines in the file
    #[arg(short = 'l')]
    pub lines_enabled: bool,

    /// Count number of bytes in the file
    #[arg(short = 'c')]
    pub bytes_enabled: bool,

    /// Count number of characters in the file (UTF-8 encoded files only)
    #[arg(short = 'm')]
    pub chars_enabled: bool,

    /// Count number of words in the file (UTF-8 encoded files only)
    #[arg(short = 'w')]
    pub words_enabled: bool,

    // get_possible_values = &["no_title", "no_linesep_with_title", "no_linesep", "no_colsep", "clean", "borders_only", "no_border", "no_border_line_separator", "box_chars"],
    // Style of the table
    #[arg(short = 'f',  default_value_t = String::from("no_border_line_separator"))]
    pub format: String,

    /// Count 1 or many files
    #[arg(required = true, num_args = 1..)]
    pub files: Vec<PathBuf>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
