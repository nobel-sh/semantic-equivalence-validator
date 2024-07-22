mod cli;

use crate::cli::{Cli, CliError, Mode};
use clap::Parser;
use std::{fs, process::ExitCode};

fn main() -> ExitCode {
    if let Err(e) = handle_cli() {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

pub fn handle_cli() -> Result<(), CliError> {
    let args = Cli::parse();

    match args.mode {
        Mode::Test { filename } => {
            if !filename.exists() {
                return Err(CliError::InvalidPath(format!(
                    "file '{}' does not exist",
                    filename.display()
                )));
            }
            let _ = fs::read_to_string(filename)?;
        }
    }
    Ok(())
}

