mod analysis;
mod cli;
mod compiler;
mod config;
mod reporting;
mod testsuite;
mod utils;

use crate::analysis::{AnalysisContext, AnalysisError};
use crate::cli::{Cli, Mode};
use crate::compiler::{compile_with, CompilerKind};
use crate::config::{AppConfig, ConfigError};
use crate::testsuite::{TestSuite, TestSuiteError};
use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;
use thiserror::Error;

const GCCRS_OUTPUT_BIN: &str = "out/gccrs.out";
const RUSTC_OUTPUT_BIN: &str = "out/rustc.out";
const ANALYSIS_TIMEOUT: u64 = 5; // in secs

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    TestSuite(#[from] TestSuiteError),

    #[error(transparent)]
    Analysis(#[from] AnalysisError),

    #[error("Compilation error for {compiler}:\n {message}")]
    Compilation { compiler: String, message: String },

    #[error("I/O error for '{file}': {error}")]
    Io {
        file: PathBuf,
        error: std::io::Error,
    },
}

fn init_logger() {
    let env = Env::default()
        .filter_or("RUST_LOG", "info") // default log level is set to info
        .write_style_or("RUST_LOG_STYLE", "auto");

    env_logger::Builder::from_env(env)
        .format_level(true)
        .format_timestamp(None)
        .format_target(false)
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
        Mode::File { rustc, gccrs } => run_file(&rustc, &gccrs, &config),
        Mode::Dir { path } => run_directory(&path, &config),
    }
}

fn run_file(rustc: &Path, gccrs: &Path, config: &AppConfig) -> Result<(), AppError> {
    let testsuite = TestSuite::from_file(rustc, gccrs)?;
    let gccrs_binary = Path::new(GCCRS_OUTPUT_BIN);
    let rustc_binary = Path::new(RUSTC_OUTPUT_BIN);
    let timeout = Duration::from_secs(ANALYSIS_TIMEOUT);

    compile_with(
        &config.rustc.path,
        &testsuite.cases[0].rustc,
        &config.rustc.args,
        CompilerKind::Rustc,
    )?;

    compile_with(
        &config.gccrs.path,
        &testsuite.cases[0].gccrs,
        &config.gccrs.args,
        CompilerKind::Gccrs,
    )?;
    info!("Starting analysis...");
    let context = AnalysisContext::new(gccrs_binary, rustc_binary, timeout);
    context.analyze()?;
    info!("Analysis complete. Results are equivalent.");
    Ok(())
}

fn run_directory(path: &Path, config: &AppConfig) -> Result<(), AppError> {
    info!("Running on '{}' directory", path.display());
    let testsuite = TestSuite::from_dir(path)?;
    info!("Validating [{}] rust files", testsuite.size);
    let gccrs_binary = Path::new(GCCRS_OUTPUT_BIN);
    let rustc_binary = Path::new(RUSTC_OUTPUT_BIN);
    let timeout = Duration::from_secs(ANALYSIS_TIMEOUT);

    for case in testsuite.cases {
        compile_with(
            &config.rustc.path,
            &case.rustc,
            &config.rustc.args,
            CompilerKind::Rustc,
        )?;
        compile_with(
            &config.gccrs.path,
            &case.gccrs,
            &config.gccrs.args,
            CompilerKind::Gccrs,
        )?;
        info!("Starting analysis...");
        let context = AnalysisContext::new(gccrs_binary, rustc_binary, timeout);
        context.analyze()?;
        info!("Results are equivalent.");
    }
    Ok(())
}
