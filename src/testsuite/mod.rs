use crate::utils;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct TestCase {
    // pub name: String,
    pub rustc: PathBuf,
    pub gccrs: PathBuf,
}

impl TestCase {
    pub fn new(rustc: PathBuf, gccrs: PathBuf) -> Self {
        Self { rustc, gccrs }
    }
}

pub struct TestSuite {
    pub cases: Vec<TestCase>,
    pub size: usize,
}

#[derive(Debug, Error)]
pub enum TestSuiteError {
    #[error("Unequal file count: rustc: {0} but gccrs: {1}")]
    UnequalFileCount(usize, usize),

    #[error("Read error : {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

impl TestSuite {
    pub fn from_file(rustc_src: &Path, gccrs_src: &Path) -> Result<Self, TestSuiteError> {
        if !rustc_src.exists() || !gccrs_src.exists() {
            let error_msg = format!("file '{}' does not exist", rustc_src.display());
            return Err(TestSuiteError::InvalidPath(error_msg));
        }
        let case = TestCase::new(rustc_src.to_path_buf(), gccrs_src.to_path_buf());
        let cases = vec![case];
        Ok(Self { cases, size: 1 })
    }

    pub fn from_dir(path: &Path) -> Result<Self, TestSuiteError> {
        if !path.exists() || !path.is_dir() {
            let msg = format!("Directory '{}' does not exist", path.display());
            return Err(TestSuiteError::InvalidPath(msg));
        }

        let rustc_dir = path.join("rustc");
        let gccrs_dir = path.join("gccrs");
        if !rustc_dir.exists() || !gccrs_dir.exists() {
            let msg = format!(
                "Required subdirectories 'rustc' and 'gccrs' not found in '{}'",
                path.display()
            );
            return Err(TestSuiteError::InvalidPath(msg));
        }

        let gccrs_files = utils::get_files_in_dir(&gccrs_dir)?;
        let rustc_files = utils::get_files_in_dir(&rustc_dir)?;
        if gccrs_files.len() != rustc_files.len() {
            return Err(TestSuiteError::UnequalFileCount(
                rustc_files.len(),
                gccrs_files.len(),
            ));
        }

        let mut cases = Vec::new();
        let file_count = gccrs_files.len();

        // TODO: add a check to see if a file existing in rustc
        // testsuite also exists in gccrs testsuite and vice-versa.
        for index in 0..file_count {
            let case = TestCase::new(
                rustc_files[index].to_path_buf(),
                gccrs_files[index].to_path_buf(),
            );
            cases.push(case);
        }
        let size = cases.len();
        Ok(Self { cases, size })
    }
}
