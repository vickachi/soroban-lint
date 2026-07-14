use crate::runner::LintResult;

/// Detects functions that mutate storage without calling require_auth()
pub fn check(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();
    let mut in_fn = false;
    let mut fn_start_line = 0;
    let mut has_auth = false;
    let mut has_storage_write = false;

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();

        // Detect function start
        if trimmed.starts_with("pub fn") || trimmed.starts_with("fn ") {
            // If we were already in a function, check the previous one
            if in_fn && has_storage_write && !has_auth {
                results.push(LintResult {
                    rule: "missing_auth_check".to_string(),
                    severity: "warning".to_string(),
                    file: file.to_string(),
                    line: fn_start_line,
                    message: "Function mutates contract state without calling require_auth()."
                        .to_string(),
                    help: "Add env.require_auth(&caller) before any state mutation.".to_string(),
                });
            }
            in_fn = true;
            fn_start_line = line_num;
            has_auth = false;
            has_storage_write = false;
        }

        if in_fn {
            if line.contains("require_auth") || line.contains("require_auth_for_args") {
                has_auth = true;
            }
            if line.contains(".set(") || line.contains(".put(") || line.contains(".remove(") {
                has_storage_write = true;
            }
        }
    }

    // Check last function
    if in_fn && has_storage_write && !has_auth {
        results.push(LintResult {
            rule: "missing_auth_check".to_string(),
            severity: "warning".to_string(),
            file: file.to_string(),
            line: fn_start_line,
            message: "Function mutates contract state without calling require_auth().".to_string(),
            help: "Add env.require_auth(&caller) before any state mutation.".to_string(),
        });
    }

    results
}
