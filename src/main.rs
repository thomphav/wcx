use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};
pub mod wc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Wc {
        #[arg(short = 'c')]
        bytes: bool,

        #[arg(short = 'l')]
        lines: bool,

        #[arg(short = 'm')]
        chars: bool,

        #[arg(short = 'w')]
        words: bool,

        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Wc {
            bytes,
            lines,
            chars,
            words,
            files,
        } => wc::invoke(bytes, lines, chars, words, &files)?,
    }

    Ok(())
}
