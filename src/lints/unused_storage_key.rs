use crate::runner::LintResult;

/// Detects DataKey enum variants that are defined but never used in storage calls
pub fn check(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();

    // Find DataKey enum variants
    let mut in_datakey_enum = false;
    let mut variants: Vec<(String, usize)> = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.contains("enum DataKey") {
            in_datakey_enum = true;
            continue;
        }

        if in_datakey_enum {
            if trimmed == "}" {
                in_datakey_enum = false;
                continue;
            }
            let variant = trimmed.trim_end_matches(',').trim().to_string();
            if !variant.is_empty() && !variant.starts_with("//") {
                variants.push((variant, i + 1));
            }
        }
    }

    // Check if each variant is used in storage calls
    for (variant, line_num) in &variants {
        let usage_pattern = format!("DataKey::{}", variant);
        if !source.contains(&usage_pattern) {
            results.push(LintResult {
                rule: "unused_storage_key".to_string(),
                severity: "info".to_string(),
                file: file.to_string(),
                line: *line_num,
                message: format!(
                    "DataKey::{} is defined but never used in any storage call.",
                    variant
                ),
                help: "Remove unused storage keys or add the missing storage operations."
                    .to_string(),
            });
        }
    }

    results
}
