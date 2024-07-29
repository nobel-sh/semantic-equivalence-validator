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
    /// Validate directly using the source file path
    File {
        /// Path to the rust source file for rustc
        rustc: PathBuf,

        /// Path to the rust source file for gccrs
        gccrs: PathBuf,
    },
    /// Validate using directories of rust source files
    Dir {
        /// Path to the root directory containing "rustc" and "gccrs" subdirectories
        path: PathBuf,
    },
}
