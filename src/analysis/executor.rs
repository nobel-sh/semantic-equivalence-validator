use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Output;
use std::time::Duration;
use thiserror::Error;

pub struct ExecutionContext {
    pub binary: PathBuf,
    pub _timeout: Duration,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub output: Output,
}

#[derive(Debug, Error, PartialEq)]
pub enum ExecutionError {
    #[error("Execution failed: {0}")]
    Failed(String),
}

impl ExecutionContext {
    pub fn new(binary: &Path, timeout: Duration) -> Self {
        assert!(
            binary.exists() && binary.is_file(),
            "Invalid binary path: {:?}",
            binary
        );
        Self {
            binary: binary.to_path_buf(),
            _timeout: timeout,
        }
    }

    pub fn run_binary(&self) -> Result<ExecutionResult, ExecutionError> {
        let output = Command::new(&self.binary)
            .output()
            .map_err(|e| ExecutionError::Failed(format!("Failed to run binary: {}", e)))?;

        Ok(ExecutionResult { output })
    }
}
