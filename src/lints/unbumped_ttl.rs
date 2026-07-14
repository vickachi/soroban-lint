use crate::runner::LintResult;

/// Detects persistent storage writes without a TTL bump
pub fn check(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;

        // Look for persistent storage writes
        if line.contains("persistent().set(") || line.contains("persistent().put(") {
            // Check surrounding lines (within 5 lines) for extend_ttl
            let start = if i >= 5 { i - 5 } else { 0 };
            let end = (i + 5).min(source.lines().count());
            let surrounding: String = source.lines().skip(start).take(end - start).collect();

            if !surrounding.contains("extend_ttl") && !surrounding.contains("bump") {
                results.push(LintResult {
                    rule: "unbumped_ttl".to_string(),
                    severity: "info".to_string(),
                    file: file.to_string(),
                    line: line_num,
                    message: "Persistent storage written without TTL extension.".to_string(),
                    help: "Consider calling env.storage().persistent().extend_ttl() after writes to prevent state expiration.".to_string(),
                });
            }
        }
    }

    results
}
