mod optimization;

use crate::AppError;
use log::info;
pub use optimization::Optimization;
use std::path::Path;
use std::process::Command;


pub const OPTIMIZATION_LEVELS: [Optimization; 6] = [
    Optimization::Zero,
    Optimization::One,
    Optimization::Two,
    Optimization::Three,
    Optimization::S,
    Optimization::Z,
];

#[derive(Debug, Clone, Copy)]
pub enum CompilerKind {
    Rustc,
    Gccrs,
}

impl CompilerKind {
    fn name(self) -> &'static str {
        match self {
            Self::Rustc => "rustc",
            Self::Gccrs => "gccrs",
        }
    }
}

impl std::fmt::Display for CompilerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

pub fn compile_with(
    compiler: &Path,
    src_file_path: &Path,
    args: &[String],
    compiler_kind: CompilerKind,
) -> Result<(), AppError> {
    info!(
        "Compiling '{}' with {}",
        src_file_path.display(),
        compiler_kind
    );

    for level in &OPTIMIZATION_LEVELS {
        let binary_path = format!("out/{}_{}.out", compiler_kind.name(), level.as_str());
        let output = Command::new(compiler)
            .arg(src_file_path)
            .args(args)
            .args(level.for_compiler(compiler_kind))
            .arg("-o")
            .arg(&binary_path)
            .output()
            .map_err(|e| AppError::Io {
                file: src_file_path.to_path_buf(),
                error: e,
            })?;

        if !output.status.success() {
            return Err(AppError::Compilation {
                compiler: compiler_kind.name().to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
    }

    Ok(())
}
