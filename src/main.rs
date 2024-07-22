mod cli;
mod config;

use crate::cli::{Cli, CliError, Mode};
use crate::config::{AppConfig, ConfigError};
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    Cli(#[from] CliError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("Compilation error: {0}")]
    Compilation(String),
}

#[derive(Debug)]
enum Compiler {
    Rustc,
    Gccrs,
}

impl Compiler {
    fn name(&self) -> &'static str {
        match self {
            Compiler::Rustc => "rustc",
            Compiler::Gccrs => "gccrs",
        }
    }
}

fn main() -> ExitCode {
    if let Err(e) = run_app() {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run_app() -> Result<(), AppError> {
    let args = Cli::parse();

    match args.mode {
        Mode::Test { filename } => {
            if !filename.exists() {
                let error_msg = format!("file '{}' does not exist", filename.display());
                return Err(CliError::InvalidPath(error_msg).into());
            }
            // simple check to see if we can read the file
            File::open(&filename).map_err(CliError::FileReadError)?;

            let config = AppConfig::load("Config.toml")?;

            compile_with(
                &config.gccrs.path,
                &filename,
                &config.gccrs.args,
                Compiler::Gccrs,
            )?;
            compile_with(
                &config.rustc.path,
                &filename,
                &config.rustc.args,
                Compiler::Rustc,
            )?;
        }
    }
    Ok(())
}

fn compile_with(
    compiler: &PathBuf,
    src_file_path: &PathBuf,
    args: &[String],
    compiler_type: Compiler,
) -> Result<(), AppError> {
    println!(
        "Compiling '{}' with {}",
        src_file_path.display(),
        compiler_type.name()
    );
    let status = Command::new(compiler)
        .arg(src_file_path)
        .args(args)
        .status()
        .map_err(|e| {
            AppError::Compilation(format!("Failed to execute {}: {}", compiler_type.name(), e))
        })?;

    if !status.success() {
        return Err(AppError::Compilation(format!(
            "{} compilation failed with exit code: {}",
            compiler_type.name(),
            status
        )));
    }

    println!("{} compilation successful", compiler_type.name());
    Ok(())
}

