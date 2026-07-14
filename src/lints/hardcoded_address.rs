use crate::runner::LintResult;

/// Detects hardcoded Stellar addresses (G... 56 chars) in contract source
pub fn check(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }

        // Look for strings that match Stellar address pattern
        // Stellar addresses start with G and are 56 characters long
        let mut chars = line.chars().peekable();
        let mut pos = 0;
        while let Some(ch) = chars.next() {
            if ch == '"' {
                let mut addr = String::new();
                for inner in chars.by_ref() {
                    if inner == '"' {
                        break;
                    }
                    addr.push(inner);
                }
                if addr.starts_with('G')
                    && addr.len() == 56
                    && addr.chars().all(|c| c.is_alphanumeric())
                {
                    results.push(LintResult {
                        rule: "hardcoded_address".to_string(),
                        severity: "warning".to_string(),
                        file: file.to_string(),
                        line: i + 1,
                        message: format!(
                            "Hardcoded Stellar address detected: {}...",
                            &addr[..8]
                        ),
                        help: "Store admin/contract addresses in contract storage or pass them as parameters instead of hardcoding.".to_string(),
                    });
                }
            }
            pos += 1;
        }
        let _ = pos;
    }

    results
}