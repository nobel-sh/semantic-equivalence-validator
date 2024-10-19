mod compare;
mod context;
mod executor;
mod result;

pub use context::AnalysisContext;
pub use executor::ExecutionError;
pub use result::AnalysisResult;
pub use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AnalysisError {
    #[error(transparent)]
    Execution(#[from] ExecutionError),

    #[error("Comparison failed for case '{1}': {0}")]
    ComparisonFailed(AnalysisResult, String),
}
