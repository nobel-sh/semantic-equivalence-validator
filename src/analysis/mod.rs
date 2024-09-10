pub mod compare;
pub mod executor;

use compare::{Comparison, ComparisonResult};
use executor::{ExecutionContext, ExecutionError};
use log::info;
use std::path::Path;
use std::time::Duration;
use thiserror::Error;

pub struct AnalysisContext {
    pub gccrs: ExecutionContext,
    pub rustc: ExecutionContext,
}

#[derive(Debug, Error, PartialEq)]
pub enum AnalysisError {
    #[error(transparent)]
    Execution(#[from] ExecutionError),

    #[error("Comparison failed: {0}")]
    ComparisonFailed(ComparisonResult),
}

impl AnalysisContext {
    pub fn new(gccrs_binary: &Path, rustc_binary: &Path, timeout: Duration) -> Self {
        Self {
            gccrs: ExecutionContext::new(gccrs_binary, timeout),
            rustc: ExecutionContext::new(rustc_binary, timeout),
        }
    }

    pub fn analyze(&self) -> Result<(), AnalysisError> {
        info!("Starting analysis...");
        let gccrs_exec_result = self.gccrs.run_binary()?;
        let rustc_exec_result = self.rustc.run_binary()?;
        let compare = Comparison::new(gccrs_exec_result, rustc_exec_result);
        let result = compare.compare();
        if result.is_identical() {
            info!("Analysis complete. Results are equivalent.");
            Ok(())
        } else {
            Err(AnalysisError::ComparisonFailed(result))
        }
    }
}
