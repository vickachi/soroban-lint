mod missing_auth_check;
mod unbumped_ttl;
mod panic_in_contract;

use crate::runner::LintResult;

/// Run all lint rules against the given source file
pub fn run_all(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();
    results.extend(missing_auth_check::check(source, file));
    results.extend(unbumped_ttl::check(source, file));
    results.extend(panic_in_contract::check(source, file));
    results
}