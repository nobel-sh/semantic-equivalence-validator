use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Semantic equivalence validation tool for gccrs")]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,

    /// Run only with optmizations turned off (equivalent to Optimization::Zero)
    #[arg(long, default_value_t = false)]
    pub no_opt: bool,
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
