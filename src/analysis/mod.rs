pub mod compare;
pub mod executor;
use compare::{Comparison, ComparisonResult};
use executor::{ExecutionContext, ExecutionError};
use std::path::Path;
use std::time::Duration;
use thiserror::Error;

pub struct AnalysisContext {
    pub testname: String,
    pub gccrs: ExecutionContext,
    pub rustc: ExecutionContext,
}

#[derive(Debug, Error, PartialEq)]
pub enum AnalysisError {
    #[error(transparent)]
    Execution(#[from] ExecutionError),

    #[error("Comparison failed for case '{1}': {0}")]
    ComparisonFailed(ComparisonResult, String),
}

impl AnalysisContext {
    pub fn new(
        testname: String,
        gccrs_binary: &Path,
        rustc_binary: &Path,
        timeout: Duration,
    ) -> Self {
        Self {
            testname,
            gccrs: ExecutionContext::new(gccrs_binary, timeout),
            rustc: ExecutionContext::new(rustc_binary, timeout),
        }
    }

    pub fn analyze(&self) -> Result<(), AnalysisError> {
        let gccrs_exec_result = self.gccrs.run_binary()?;
        let rustc_exec_result = self.rustc.run_binary()?;
        let compare = Comparison::new(gccrs_exec_result, rustc_exec_result);
        let result = compare.compare();
        if result.is_identical() {
            Ok(())
        } else {
            Err(AnalysisError::ComparisonFailed(
                result,
                self.testname.clone(),
            ))
        }
    }
}
