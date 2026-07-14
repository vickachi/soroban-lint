use crate::runner::LintResult;

/// Detects use of panic! macro in Soroban contracts (should use Result/Error instead)
pub fn check(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();

        // Skip comments
        if trimmed.starts_with("//") {
            continue;
        }

        if line.contains("panic!(") || line.contains("panic!(\"") {
            results.push(LintResult {
                rule: "panic_in_contract".to_string(),
                severity: "warning".to_string(),
                file: file.to_string(),
                line: line_num,
                message: "Use of panic!() detected in contract code.".to_string(),
                help: "Soroban contracts should return errors via Result<T, ContractError> instead of panicking. This gives callers better error handling.".to_string(),
            });
        }
    }

    results
}