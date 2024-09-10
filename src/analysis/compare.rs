use super::executor::ExecutionResult;

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    pub exit_code_differ: Option<(String, String)>,
    pub stdout_differ: Option<Box<(String, String)>>,
    pub stderr_differ: Option<Box<(String, String)>>,
    pub timeout_differ: Option<(bool, bool)>,
}

impl ComparisonResult {
    pub fn is_identical(&self) -> bool {
        self.exit_code_differ.is_none()
            && self.stdout_differ.is_none()
            && self.stderr_differ.is_none()
            && self.timeout_differ.is_none()
    }
    pub fn has_diff(&self) -> bool {
        self.exit_code_differ.is_some()
            || self.stdout_differ.is_some()
            || self.stderr_differ.is_some()
            || self.timeout_differ.is_some()
    }
}

impl std::fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_diff() {
            if let Some((gccrs_timed_out, rustc_timed_out)) = self.timeout_differ {
                writeln!(
                    f,
                    "\n=== Timeout Difference ===\n\
                     gccrs timed out: {}\n\
                     rustc timed out: {}\n",
                    gccrs_timed_out, rustc_timed_out
                )?;
            }

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

pub struct Comparison {
    gccrs: ExecutionResult,
    rustc: ExecutionResult,
}

impl Comparison {
    pub fn new(gccrs: ExecutionResult, rustc: ExecutionResult) -> Self {
        Comparison { gccrs, rustc }
    }

    pub fn compare(&self) -> ComparisonResult {
        let timeout_differ = self.compare_timeouts();
        if timeout_differ.is_some() {
            return ComparisonResult {
                exit_code_differ: None,
                stdout_differ: None,
                stderr_differ: None,
                timeout_differ,
            };
        }
        let exit_code_differ = self.compare_exit_code();
        let stdout_differ = self
            .compare_output(
                "stdout",
                &self.gccrs.output.as_ref().map(|o| &o.stdout),
                &self.rustc.output.as_ref().map(|o| &o.stdout),
            )
            .map(Box::new);
        let stderr_differ = self
            .compare_output(
                "stderr",
                &self.gccrs.output.as_ref().map(|o| &o.stderr),
                &self.rustc.output.as_ref().map(|o| &o.stderr),
            )
            .map(Box::new);

        ComparisonResult {
            exit_code_differ,
            stdout_differ,
            stderr_differ,
            timeout_differ,
        }
    }

    fn compare_timeouts(&self) -> Option<(bool, bool)> {
        if self.gccrs.timed_out != self.rustc.timed_out {
            Some((self.gccrs.timed_out, self.rustc.timed_out))
        } else {
            None
        }
    }

    fn compare_exit_code(&self) -> Option<(String, String)> {
        let format_exit_code = |code: Option<i32>| {
            code.map_or_else(|| "Terminated by signal".to_string(), |c| c.to_string())
        };

        // Check if both outputs are present
        let gccrs_exit_code = self.gccrs.output.as_ref().and_then(|o| o.status.code());
        let rustc_exit_code = self.rustc.output.as_ref().and_then(|o| o.status.code());

        if gccrs_exit_code != rustc_exit_code {
            let gccrs_exit = format_exit_code(gccrs_exit_code);
            let rustc_exit = format_exit_code(rustc_exit_code);
            Some((gccrs_exit, rustc_exit))
        } else {
            None
        }
    }

    fn compare_output(
        &self,
        _name: &str,
        gccrs: &Option<&Vec<u8>>,
        rustc: &Option<&Vec<u8>>,
    ) -> Option<(String, String)> {
        match (gccrs, rustc) {
            (Some(g), Some(r)) if g != r => {
                let gccrs_str = String::from_utf8_lossy(g).into_owned();
                let rustc_str = String::from_utf8_lossy(r).into_owned();
                Some((gccrs_str, rustc_str))
            }
            (Some(g), None) => Some((
                String::from_utf8_lossy(g).into_owned(),
                "No output (timed out)".to_string(),
            )),
            (None, Some(r)) => Some((
                "No output (timed out)".to_string(),
                String::from_utf8_lossy(r).into_owned(),
            )),
            _ => None,
        }
    }
}
