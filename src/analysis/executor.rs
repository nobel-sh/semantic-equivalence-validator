use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use thiserror::Error;

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
        let (sender, receiver) = mpsc::channel();
        let binary = self.binary.clone();
        let timeout = self.timeout;

        thread::spawn(move || {
            let output = Command::new(binary)
                .output()
                .map_err(|e| ExecutionError::Failed(e.to_string()));
            sender.send(output).unwrap();
        });

        match receiver.recv_timeout(timeout) {
            Ok(Ok(output)) => Ok(ExecutionResult {
                output: Some(output),
                timed_out: false,
            }),
            Ok(Err(e)) => Err(e),
            Err(_) => Ok(ExecutionResult {
                output: None,
                timed_out: true,
            }),
        }
    }
}
