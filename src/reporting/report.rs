use super::error_reporter::ErrorReporter;
use crate::analysis::AnalysisError;
use colored::*;
use log::info;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Report {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub errors: Vec<ErrorReporter>,
    pub analysis_reports: Vec<AnalysisReport>,
    start_time: Instant,
}
#[derive(Debug)]
pub struct AnalysisReport {
    pub test_name: String,
    pub result: Result<(), AnalysisError>,
    pub duration: Duration,
}

impl Report {
    pub fn new() -> Self {
        Self {
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            errors: Vec::new(),
            analysis_reports: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn add_result(
        &mut self,
        test_name: String,
        result: Result<(), AnalysisError>,
        duration: Duration,
    ) {
        self.total_tests += 1;
        match &result {
            Ok(_) => self.passed_tests += 1,
            Err(_) => self.failed_tests += 1,
        }

        self.analysis_reports.push(AnalysisReport {
            test_name,
            result,
            duration,
        });
    }

    pub fn add_error(&mut self, error: ErrorReporter) {
        self.errors.push(error);
    }

    pub fn print_summary(&self) {
        info!("Testing complete. Summary below:");

        if !self.errors.is_empty() {
            println!("{}", "Errors:".bold().underline().bright_red());
            for (i, error) in self.errors.iter().enumerate() {
                println!("{}", "-".repeat(40).dimmed());
                println!("{}", format!("{} {}:", "Error", (i + 1)).bold());
                println!("{}\n{}", error, "-".repeat(40).dimmed());
            }
        }

        if !self.analysis_reports.is_empty() {
            println!("{}", "Analysis Results:".bold().underline().cyan());
            for result in &self.analysis_reports {
                println!("{}", "-".repeat(40).dimmed());
                println!("{}", format!("Report for '{}': ", result.test_name).bold());
                match &result.result {
                    Ok(_) => println!("{}", "Passed".green()),
                    Err(e) => match e {
                        AnalysisError::Execution(exec_error) => {
                            println!("{}", format!("Execution Error: {}", exec_error).red())
                        }
                        AnalysisError::ComparisonFailed(comparison_result, _) => {
                            println!("{}", "Comparison Failed".red());
                            println!("{}", comparison_result);
                        }
                    },
                }
                println!("Duration: {:.2?}", result.duration);
                println!("{}", "-".repeat(40).dimmed());
            }
        }

        println!("{}", "Test Summary:".bold().underline().green());
        println!(
            "{} {}",
            "Total tests:".bold(),
            self.total_tests.to_string().white()
        );
        println!(
            "{} {}",
            "Passed tests:".bold(),
            self.passed_tests.to_string().green()
        );
        println!(
            "{} {}",
            "Failed tests:".bold(),
            self.failed_tests.to_string().red()
        );

        let total_duration = self.start_time.elapsed();
        println!(
            "{} {}",
            "Total duration:".bold(),
            format!("{:.2?}", total_duration).cyan()
        );
        println!("{}", "-".repeat(40).dimmed());
    }
}
