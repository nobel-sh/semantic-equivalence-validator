use super::executor::ExecutionResult;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Difference {
    ExitCode(String, String),
    Stdout(String, String),
    Stderr(String, String),
    Timeout(bool, bool),
}

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    pub differences: Vec<Difference>,
}

impl ComparisonResult {
    pub fn is_identical(&self) -> bool {
        self.differences.is_empty()
    }

    pub fn has_diff(&self) -> bool {
        !self.is_identical()
    }
}

impl fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_diff() {
            for diff in &self.differences {
                match diff {
                    Difference::Timeout(gccrs_timed_out, rustc_timed_out) => {
                        writeln!(
                            f,
                            "\n=== Timeout Difference ===\n\
                             gccrs timed out: {}\n\
                             rustc timed out: {}\n",
                            gccrs_timed_out, rustc_timed_out
                        )?;
                    }
                    Difference::ExitCode(gccrs_exit, rustc_exit) => {
                        writeln!(
                            f,
                            "\n=== Exit Code Difference ===\n\
                             gccrs: {}\n\
                             rustc: {}\n",
                            gccrs_exit, rustc_exit
                        )?;
                    }
                    Difference::Stdout(gccrs_stdout, rustc_stdout) => {
                        writeln!(
                            f,
                            "\n=== Stdout Difference ===\n\
                             gccrs:\n  {}\n\
                             rustc:\n  {}\n",
                            if gccrs_stdout.is_empty() {
                                "(empty)"
                            } else {
                                gccrs_stdout
                            },
                            if rustc_stdout.is_empty() {
                                "(empty)"
                            } else {
                                rustc_stdout
                            }
                        )?;
                    }
                    Difference::Stderr(gccrs_stderr, rustc_stderr) => {
                        writeln!(
                            f,
                            "\n=== Stderr Difference ===\n\
                             gccrs:\n  {}\n\
                             rustc:\n  {}\n",
                            if gccrs_stderr.is_empty() {
                                "(empty)"
                            } else {
                                gccrs_stderr
                            },
                            if rustc_stderr.is_empty() {
                                "(empty)"
                            } else {
                                rustc_stderr
                            }
                        )?;
                    }
                }
            }
        } else {
            writeln!(f, "No differences found. The results are identical.")?;
        }
        Ok(())
    }
}

pub struct Comparison {
    gccrs: ExecutionResult,
    rustc: ExecutionResult,
}

impl Comparison {
    pub fn new(gccrs: ExecutionResult, rustc: ExecutionResult) -> Self {
        Comparison { gccrs, rustc }
    }

    pub fn compare(&self) -> ComparisonResult {
        let mut differences = Vec::new();

        if let Some(timeout_diff) = self.compare_timeouts() {
            differences.push(Difference::Timeout(timeout_diff.0, timeout_diff.1));
            return ComparisonResult { differences }; // Skip other section if we timeout
        }

        if let Some(exit_code_diff) = self.compare_exit_code() {
            differences.push(Difference::ExitCode(exit_code_diff.0, exit_code_diff.1));
        }

        if let Some(stdout_diff) = self.compare_output(
            &self.gccrs.output.as_ref().and_then(|o| Some(&o.stdout)),
            &self.rustc.output.as_ref().and_then(|o| Some(&o.stdout)),
        ) {
            differences.push(Difference::Stdout(stdout_diff.0, stdout_diff.1));
        }

        if let Some(stderr_diff) = self.compare_output(
            &self.gccrs.output.as_ref().and_then(|o| Some(&o.stderr)),
            &self.rustc.output.as_ref().and_then(|o| Some(&o.stderr)),
        ) {
            differences.push(Difference::Stderr(stderr_diff.0, stderr_diff.1));
        }

        ComparisonResult { differences }
    }

    fn compare_timeouts(&self) -> Option<(bool, bool)> {
        if self.gccrs.timed_out != self.rustc.timed_out {
            Some((self.gccrs.timed_out, self.rustc.timed_out))
        } else {
            None
        }
    }

    fn format_exit_code(code: Option<i32>) -> String {
        code.map_or_else(|| "Terminated by signal".to_string(), |c| c.to_string())
    }

    fn compare_exit_code(&self) -> Option<(String, String)> {
        let gccrs_exit_code = self.gccrs.output.as_ref().and_then(|o| o.status.code());
        let rustc_exit_code = self.rustc.output.as_ref().and_then(|o| o.status.code());

        if gccrs_exit_code != rustc_exit_code {
            Some((
                Self::format_exit_code(gccrs_exit_code),
                Self::format_exit_code(rustc_exit_code),
            ))
        } else {
            None
        }
    }

    fn format_output(opt_output: &Option<&Vec<u8>>) -> String {
        opt_output
            .map(|output| String::from_utf8_lossy(output).into_owned())
            .unwrap_or_else(|| "No output (timed out)".to_string())
    }

    fn compare_output(
        &self,
        gccrs: &Option<&Vec<u8>>,
        rustc: &Option<&Vec<u8>>,
    ) -> Option<(String, String)> {
        let gccrs_output = Self::format_output(gccrs);
        let rustc_output = Self::format_output(rustc);

        if gccrs_output != rustc_output {
            Some((gccrs_output, rustc_output))
        } else {
            None
        }
    }
}
