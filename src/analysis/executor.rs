use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Duration;
use thiserror::Error;
use wait_timeout::ChildExt;

pub struct ExecutionContext {
    pub binary: PathBuf,
    pub timeout: Duration,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub output: Option<Output>,
    pub timed_out: bool,
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
            timeout,
        }
    }

    pub fn run_binary(&self) -> Result<ExecutionResult, ExecutionError> {
        let binary = self.binary.clone();
        let timeout = self.timeout;

        let mut child = Command::new(binary)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ExecutionError::Failed(e.to_string()))?;

        match child
            .wait_timeout(timeout)
            .map_err(|e| ExecutionError::Failed(e.to_string()))?
        {
            Some(_) => {
                let output = child
                    .wait_with_output()
                    .map_err(|e| ExecutionError::Failed(e.to_string()))?;

                Ok(ExecutionResult {
                    output: Some(output),
                    timed_out: false,
                })
            }
            None => {
                child
                    .kill()
                    .map_err(|e| ExecutionError::Failed(e.to_string()))?;
                child.wait().ok();

                Ok(ExecutionResult {
                    output: None,
                    timed_out: true,
                })
            }
        }
    }
}
