use crate::analysis::AnalysisError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorReporter {
    Analysis(AnalysisError),
    Compilation { message: String },
}

impl fmt::Display for ErrorReporter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorReporter::Analysis(e) => write!(f, "Analysis error: {}", e),
            ErrorReporter::Compilation { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl From<AnalysisError> for ErrorReporter {
    fn from(error: AnalysisError) -> Self {
        ErrorReporter::Analysis(error)
    }
}
