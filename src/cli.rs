use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'l')]
    pub lines_enabled: bool,

    #[arg(short = 'c')]
    pub bytes_enabled: bool,

    #[arg(short = 'm')]
    pub chars_enabled: bool,

    #[arg(short = 'w')]
    pub words_enabled: bool,

    #[arg(required = true, num_args = 1..)]
    pub files: Vec<PathBuf>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
