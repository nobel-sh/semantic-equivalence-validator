use super::executor::ExecutionResult;

pub struct Comparison {
    gccrs: ExecutionResult,
    rustc: ExecutionResult,
}

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    pub exit_code_differ: Option<(String, String)>,
    pub stdout_differ: Option<Box<(String, String)>>,
    pub stderr_differ: Option<Box<(String, String)>>,
}

impl ComparisonResult {
    pub fn is_identical(&self) -> bool {
        self.exit_code_differ.is_none()
            && self.stdout_differ.is_none()
            && self.stderr_differ.is_none()
    }
    pub fn has_diff(&self) -> bool {
        self.exit_code_differ.is_some()
            || self.stdout_differ.is_some()
            || self.stderr_differ.is_some()
    }
}

impl std::fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_diff() {
            if let Some((gccrs_exit, rustc_exit)) = &self.exit_code_differ {
                writeln!(
                    f,
                    "\n=== Exit Code Difference ===\n\
                     gccrs: {}\n\
                     rustc: {}\n",
                    gccrs_exit, rustc_exit
                )?;
            }

            if let Some(boxed_stdout) = &self.stdout_differ {
                let (gccrs_stdout, rustc_stdout) = &**boxed_stdout;
                writeln!(
                    f,
                    "=== Stdout Difference ===\n\
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
                    },
                )?;
            }

            if let Some(boxed_stderr) = &self.stderr_differ {
                let (gccrs_stderr, rustc_stderr) = &**boxed_stderr;
                writeln!(
                    f,
                    "=== Stderr Difference ===\n\
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
                    },
                )?;
            }
        } else {
            writeln!(f, "No differences found. The results are identical.")?;
        }

        Ok(())
    }
}

impl Comparison {
    pub fn new(gccrs: ExecutionResult, rustc: ExecutionResult) -> Self {
        Comparison { gccrs, rustc }
    }

    pub fn compare(&self) -> ComparisonResult {
        let exit_code_differ = self.compare_exit_code();
        let stdout_differ = self
            .compare_output(
                "stdout",
                &self.gccrs.output.stdout,
                &self.rustc.output.stdout,
            )
            .map(Box::new);
        let stderr_differ = self
            .compare_output(
                "stderr",
                &self.gccrs.output.stderr,
                &self.rustc.output.stderr,
            )
            .map(Box::new);
        ComparisonResult {
            exit_code_differ,
            stdout_differ,
            stderr_differ,
        }
    }

    fn compare_exit_code(&self) -> Option<(String, String)> {
        let gccrs_exit_code = self.gccrs.output.status.code();
        let rustc_exit_code = self.rustc.output.status.code();

        let format_exit_code = |code: Option<i32>| {
            code.map_or_else(|| "Terminated by signal".to_string(), |c| c.to_string())
        };

        if gccrs_exit_code != rustc_exit_code {
            let gccrs_exit = format_exit_code(gccrs_exit_code);
            let rustc_exit = format_exit_code(rustc_exit_code);
            Some((gccrs_exit, rustc_exit))
        } else {
            None
        }
    }

    fn compare_output(&self, _name: &str, gccrs: &[u8], rustc: &[u8]) -> Option<(String, String)> {
        if gccrs != rustc {
            let gccrs_str = String::from_utf8_lossy(gccrs).into_owned();
            let rustc_str = String::from_utf8_lossy(rustc).into_owned();
            Some((gccrs_str, rustc_str))
        } else {
            None
        }
    }
}
