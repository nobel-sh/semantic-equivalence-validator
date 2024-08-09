use crate::AppError;
use log::info;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Debug)]
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

pub fn compile_with(
    compiler: &PathBuf,
    src_file_path: &PathBuf,
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
