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
use crate::testsuite::{TestCase, TestSuite, TestSuiteError};
use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;
use thiserror::Error;

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

    #[error("{1} difference found.")]
    MultipleErrors(Vec<AppError>, usize),
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
    let gccrs_binary = Path::new("out/gccrs.out");
    let rustc_binary = Path::new("out/rustc.out");
    let timeout = Duration::from_secs(ANALYSIS_TIMEOUT);

    compile_and_analyze_case(
        &testsuite.cases[0],
        config,
        gccrs_binary,
        rustc_binary,
        timeout,
    )
}

fn run_directory(path: &Path, config: &AppConfig) -> Result<(), AppError> {
    info!("Running on '{}' directory", path.display());
    let testsuite = TestSuite::from_dir(path)?;
    info!("Validating [{}] test cases", testsuite.size);

    let timeout = Duration::from_secs(ANALYSIS_TIMEOUT);
    let gccrs_binary = Path::new("out/gccrs.out");
    let rustc_binary = Path::new("out/rustc.out");

    let errors: Vec<_> = testsuite
        .cases
        .iter()
        .filter_map(|case| {
            compile_and_analyze_case(case, config, gccrs_binary, rustc_binary, timeout).err()
        })
        .collect();

    if !errors.is_empty() {
        let mut count = 0;
        for error in &errors {
            error!("{}", error);
            count += 1;
        }
        return Err(AppError::MultipleErrors(errors, count));
    } else {
        Ok(())
    }
}

fn compile_and_analyze_case(
    case: &TestCase,
    config: &AppConfig,
    gccrs_binary: &Path,
    rustc_binary: &Path,
    timeout: Duration,
) -> Result<(), AppError> {
    compile_with_compiler(
        &config.rustc.path,
        &case.rustc,
        &config.rustc.args,
        CompilerKind::Rustc,
    )?;
    compile_with_compiler(
        &config.gccrs.path,
        &case.gccrs,
        &config.gccrs.args,
        CompilerKind::Gccrs,
    )?;

    info!("Starting analysis for case '{}' ...", case.name);
    let context = AnalysisContext::new(case.name.clone(), gccrs_binary, rustc_binary, timeout);

    context.analyze().map_err(AppError::Analysis)?;

    info!("Results are equivalent for case '{}'.", case.name);
    Ok(())
}

fn compile_with_compiler(
    compiler_path: &Path,
    input_file: &Path,
    args: &[String],
    kind: CompilerKind,
) -> Result<(), AppError> {
    compile_with(compiler_path, input_file, args, kind.clone()).map_err(|e| AppError::Compilation {
        compiler: kind.to_string(),
        message: e.to_string(),
    })
}
