pub mod hardcoded_address;
pub mod missing_auth_check;
pub mod missing_error_type;
pub mod panic_in_contract;
pub mod unbumped_ttl;
pub mod unused_storage_key;

use crate::runner::LintResult;

/// Run all lint rules against the given source file
pub fn run_all(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();
    results.extend(missing_auth_check::check(source, file));
    results.extend(unbumped_ttl::check(source, file));
    results.extend(panic_in_contract::check(source, file));
    results.extend(hardcoded_address::check(source, file));
    results.extend(unused_storage_key::check(source, file));
    results.extend(missing_error_type::check(source, file));
    results
}
