use colored::*;
use similar::{ChangeTag, TextDiff};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Diff {
    ExitCode(String, String),
    Stdout(String, String),
    Stderr(String, String),
    Timeout(bool, bool),
}

impl fmt::Display for Diff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Diff::Timeout(gccrs_timed_out, rustc_timed_out) => {
                writeln!(f, "{}", "=== Timeout Diff ===".bold())?;
                writeln!(
                    f,
                    "gccrs timed out: {}\nrustc timed out: {}",
                    gccrs_timed_out, rustc_timed_out
                )?;
            }
            Diff::ExitCode(gccrs_exit, rustc_exit) => {
                writeln!(f, "\n{}", "=== Exit Code Diff ===".bold())?;
                writeln!(f, "gccrs: {}\nrustc: {}", gccrs_exit, rustc_exit)?;
            }
            Diff::Stdout(gccrs_stdout, rustc_stdout) => {
                writeln!(f, "{}", "=== Stdout Diff ===".bold())?;
                print_diff(f, gccrs_stdout, rustc_stdout)?;
            }
            Diff::Stderr(gccrs_stderr, rustc_stderr) => {
                writeln!(f, "{}", "=== Stderr Diff ===".bold())?;
                print_diff(f, gccrs_stderr, rustc_stderr)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct AnalysisResult {
    pub differences: Vec<Diff>,
}

impl AnalysisResult {
    pub fn is_identical(&self) -> bool {
        self.differences.is_empty()
    }

    pub fn has_diff(&self) -> bool {
        !self.is_identical()
    }
}

impl fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_diff() {
            for diff in &self.differences {
                writeln!(f, "{}", diff)?;
            }
        } else {
            writeln!(f, "No differences found. The results are identical.")?;
        }
        Ok(())
    }
}

fn print_diff(f: &mut fmt::Formatter<'_>, rustc: &str, gccrs: &str) -> fmt::Result {
    let diff = TextDiff::from_lines(rustc, gccrs);

    for group in diff.grouped_ops(3).iter() {
        writeln!(
            f,
            "Legend: {}, {}",
            format!("- gccrs").red(),
            format!("+ rustc").green()
        )?;

        for op in group {
            for change in diff.iter_changes(op) {
                let line_number = change
                    .old_index()
                    .map_or_else(|| change.new_index().map(|i| i + 1), |i| Some(i + 1));

                match change.tag() {
                    ChangeTag::Delete => {
                        write!(
                            f,
                            "{:4} - | {}",
                            line_number.unwrap_or(0).to_string().red(),
                            change.to_string().red()
                        )?;
                    }
                    ChangeTag::Insert => {
                        write!(
                            f,
                            "{:4} + | {}",
                            line_number.unwrap_or(0).to_string().green(),
                            change.to_string().green()
                        )?;
                    }
                    ChangeTag::Equal => {
                        write!(f, "{:4}   | {}", line_number.unwrap_or(0), change)?;
                    }
                }
            }
        }
    }
    Ok(())
}
