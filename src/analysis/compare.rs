use super::executor::ExecutionResult;
use super::result::{AnalysisResult, Diff};

pub struct Comparison {
    gccrs: ExecutionResult,
    rustc: ExecutionResult,
}

impl Comparison {
    pub fn new(gccrs: ExecutionResult, rustc: ExecutionResult) -> Self {
        Comparison { gccrs, rustc }
    }

    pub fn compare(&self) -> AnalysisResult {
        let mut differences = Vec::new();

        if let Some(timeout_diff) = self.compare_timeouts() {
            differences.push(Diff::Timeout(timeout_diff.0, timeout_diff.1));
            return AnalysisResult { differences }; // Skip other section if we timeout
        }

        if let Some(exit_code_diff) = self.compare_exit_code() {
            differences.push(Diff::ExitCode(exit_code_diff.0, exit_code_diff.1));
        }

        if let Some(stdout_diff) = self.compare_output(
            &self.gccrs.output.as_ref().map(|o| (&o.stdout)),
            &self.rustc.output.as_ref().map(|o| (&o.stdout)),
        ) {
            differences.push(Diff::Stdout(stdout_diff.0, stdout_diff.1));
        }

        if let Some(stderr_diff) = self.compare_stderr() {
            differences.push(Diff::Stderr(stderr_diff.0, stderr_diff.1));
        }

        AnalysisResult { differences }
    }

    /// Check only if error exists or not but we dont compare
    /// the error messages as error messages may differ.
    fn compare_stderr(&self) -> Option<(String, String)> {
        let gccrs_stderr = self.gccrs.output.as_ref().map(|o| &o.stderr);
        let rustc_stderr = self.rustc.output.as_ref().map(|o| &o.stderr);

        let gccrs_has_error = gccrs_stderr.map_or(false, |stderr| !stderr.is_empty());
        let rustc_has_error = rustc_stderr.map_or(false, |stderr| !stderr.is_empty());

        if gccrs_has_error != rustc_has_error {
            Some((
                Self::format_output(&gccrs_stderr),
                Self::format_output(&rustc_stderr),
            ))
        } else {
            None
        }
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
