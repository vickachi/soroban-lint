use crate::lints;
use colored::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct LintResult {
    pub rule: String,
    pub severity: String,
    pub file: String,
    pub line: usize,
    pub message: String,
    pub help: String,
}

pub fn run(path: &Path, quiet: bool) -> Vec<LintResult> {
    let mut results = Vec::new();

    // Collect all .rs files
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .collect();

    for entry in &files {
        let file_path = entry.path();
        let Ok(source) = fs::read_to_string(file_path) else {
            continue;
        };

        let file_results = lints::run_all(&source, file_path.to_string_lossy().as_ref());
        for result in &file_results {
            if quiet && result.severity == "info" {
                continue;
            }
            print_result(result);
        }
        results.extend(file_results);
    }

    results
}

fn print_result(result: &LintResult) {
    let severity_label = match result.severity.as_str() {
        "warning" => "[WARN]".yellow().bold(),
        "error" => "[ERROR]".red().bold(),
        _ => "[INFO]".blue().bold(),
    };

    println!(
        "{} {} — {}:{}",
        severity_label,
        result.rule.bold(),
        result.file,
        result.line
    );
    println!("  {}", result.message);
    println!("  {}: {}\n", "Help".dimmed(), result.help.dimmed());
}