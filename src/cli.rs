use clap::{Parser, Subcommand};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Read error: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

#[derive(Parser)]
#[command(about = "Semantic equivalence validation tool for gccrs")]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Run the test harness
    Test {
        /// Path to the rust source file that needs to be verified
        filename: PathBuf,
    },
}
