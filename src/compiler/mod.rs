use crate::AppError;
use log::info;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum CompilerKind {
    Rustc,
    Gccrs,
}

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Compilation Error on {compiler}:\n {message}")]
    CompilationError { compiler: String, message: String },

    #[error("Execution Error on {compiler}:\n {message}")]
    ExecutionError { compiler: String, message: String },
}

pub struct CompilationOutput {}

impl CompilerKind {
    fn name(&self) -> &'static str {
        match self {
            CompilerKind::Rustc => "rustc",
            CompilerKind::Gccrs => "gccrs",
        }
    }
}

impl std::fmt::Display for CompilerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerKind::Rustc => write!(f, "rustc"),
            CompilerKind::Gccrs => write!(f, "gccrs"),
        }
    }
}

pub fn compile_with(
    compiler: &Path,
    src_file_path: &Path,
    args: &[String],
    compiler_type: CompilerKind,
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
            file: src_file_path.to_path_buf(),
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
