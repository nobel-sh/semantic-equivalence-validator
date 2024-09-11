use crate::utils;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub rustc: PathBuf,
    pub gccrs: PathBuf,
}

impl TestCase {
    pub fn new(name: String, rustc: PathBuf, gccrs: PathBuf) -> Self {
        Self { name, rustc, gccrs }
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

    #[error("Read error: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

fn extract_test_name(file_path: &Path) -> Option<String> {
    file_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|name| name.to_string())
}

impl TestSuite {
    pub fn from_file(rustc_src: &Path, gccrs_src: &Path) -> Result<Self, TestSuiteError> {
        if !rustc_src.exists() || !gccrs_src.exists() {
            let error_msg = format!("File '{}' does not exist", rustc_src.display());
            return Err(TestSuiteError::InvalidPath(error_msg));
        }

        let test_name = extract_test_name(rustc_src)
            .ok_or_else(|| TestSuiteError::InvalidPath("Invalid file name".to_string()))?;

        let case = TestCase::new(test_name, rustc_src.to_path_buf(), gccrs_src.to_path_buf());
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

        let rustc_files = utils::get_files_in_dir(&rustc_dir)?;
        let gccrs_files = utils::get_files_in_dir(&gccrs_dir)?;

        let rustc_file_map: std::collections::HashMap<String, PathBuf> = rustc_files
            .into_iter()
            .filter_map(|file| extract_test_name(&file).map(|name| (name, file)))
            .collect();

        let gccrs_file_map: std::collections::HashMap<String, PathBuf> = gccrs_files
            .into_iter()
            .filter_map(|file| extract_test_name(&file).map(|name| (name, file)))
            .collect();

        if rustc_file_map.len() != gccrs_file_map.len() {
            return Err(TestSuiteError::UnequalFileCount(
                rustc_file_map.len(),
                gccrs_file_map.len(),
            ));
        }

        let mut cases = Vec::new();
        for (name, rustc_file) in rustc_file_map {
            if let Some(gccrs_file) = gccrs_file_map.get(&name) {
                cases.push(TestCase::new(name, rustc_file, gccrs_file.clone()));
            } else {
                return Err(TestSuiteError::InvalidPath(format!(
                    "No matching file for '{}' in gccrs directory",
                    name
                )));
            }
        }
        let size = cases.len();
        Ok(Self { cases, size })
    }
}
