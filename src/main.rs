use std::process::ExitCode;
mod analyze;
mod cli;
mod command;

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
    let cli::Args {
        lines_enabled,
        bytes_enabled,
        chars_enabled,
        words_enabled,
        format,
        files,
    } = cli::Args::parse_args();

    command::invoke(
        lines_enabled,
        bytes_enabled,
        chars_enabled,
        words_enabled,
        &format,
        &files,
    )?;

    Ok(())
}
