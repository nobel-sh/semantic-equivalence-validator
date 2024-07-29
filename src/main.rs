mod cli;
mod config;

use crate::cli::{Cli, CliError, Mode};
use crate::config::{AppConfig, ConfigError};
use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    Cli(#[from] CliError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("Compilation error for {compiler}:\n {message}")]
    Compilation { compiler: String, message: String },

    #[error("I/O error for '{file}': {error}")]
    Io {
        file: PathBuf,
        error: std::io::Error,
    },

    #[error("Unequal file count: rustc: {0} but gccrs: {1}")]
    UnequalFileCount(usize, usize),
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

fn init_logger() {
    let env = Env::default()
        .filter_or("RUST_LOG", "info") // default log level is set to info
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::Builder::from_env(env)
        .format_level(true)
        .format_timestamp(None)
        .init()
}

fn main() -> ExitCode {
    init_logger();

    if let Err(e) = run_app() {
        error!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run_app() -> Result<(), AppError> {
    let config = AppConfig::load("config/Compiler.toml")?;
    info!("Config file read successful");

    let args = Cli::parse();
    match args.mode {
        Mode::File { rustc, gccrs } => process_file(&rustc, &gccrs, &config),
        Mode::Dir { path } => process_directory(&path, &config),
    }
}

fn process_file(rustc: &PathBuf, gccrs: &PathBuf, config: &AppConfig) -> Result<(), AppError> {
    if !rustc.exists() || !gccrs.exists() {
        let error_msg = format!("file '{}' does not exist", rustc.display());
        return Err(CliError::InvalidPath(error_msg).into());
    }

    compile_with(
        &config.rustc.path,
        rustc,
        &config.rustc.args,
        Compiler::Rustc,
    )?;

    compile_with(
        &config.gccrs.path,
        gccrs,
        &config.gccrs.args,
        Compiler::Gccrs,
    )?;
    Ok(())
}

fn process_directory(path: &Path, config: &AppConfig) -> Result<(), AppError> {
    info!("Running on '{}' directory", path.display());
    if !path.exists() || !path.is_dir() {
        let msg = format!("Directory '{}' does not exist", path.display());
        return Err(CliError::InvalidPath(msg).into());
    }

    let rustc_dir = path.join("rustc");
    let gccrs_dir = path.join("gccrs");

    if !rustc_dir.exists() || !gccrs_dir.exists() {
        let msg = format!(
            "Required subdirectories 'rustc' and 'gccrs' not found in '{}'",
            path.display()
        );
        error!("{}", msg);
        return Err(CliError::InvalidPath(msg).into());
    }

    let gccrs_files = get_files_in_directory(&gccrs_dir)?;
    let rustc_files = get_files_in_directory(&rustc_dir)?;
    if gccrs_files.len() != rustc_files.len() {
        return Err(AppError::UnequalFileCount(
            rustc_files.len(),
            gccrs_files.len(),
        ));
    }

    info!("Validating {} rust files", rustc_files.len());
    let file_count = rustc_files.len();
    for index in 0..file_count {
        compile_with(
            &config.rustc.path,
            &rustc_files[index],
            &config.rustc.args,
            Compiler::Rustc,
        )?;
        compile_with(
            &config.gccrs.path,
            &gccrs_files[index],
            &config.gccrs.args,
            Compiler::Gccrs,
        )?;
    }

    Ok(())
}

fn get_files_in_directory(dir: &PathBuf) -> Result<Vec<PathBuf>, AppError> {
    info!("Reading files from directory: {}", dir.display());
    let files = read_dir(dir)
        .map_err(|e| AppError::Io {
            file: dir.clone(),
            error: e,
        })?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    Ok(files)
}

fn compile_with(
    compiler: &PathBuf,
    src_file_path: &PathBuf,
    args: &[String],
    compiler_type: Compiler,
) -> Result<(), AppError> {
    info!(
        "Compiling '{}' with {}",
        src_file_path.display(),
        compiler_type.name()
    );

    let output = Command::new(compiler)
        .arg(src_file_path)
        .args(args)
        .output()
        .map_err(|e| AppError::Io {
            file: src_file_path.clone(),
            error: e,
        })?;

    if !output.status.success() {
        return Err(AppError::Compilation {
            compiler: compiler_type.name().to_string(),
            message: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    Ok(())
}
