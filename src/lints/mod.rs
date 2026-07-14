mod missing_auth_check;
mod unbumped_ttl;
mod panic_in_contract;
mod hardcoded_address;
mod unused_storage_key;
mod missing_error_type;

pub use missing_auth_check::check as missing_auth_check_check;
pub use unbumped_ttl::check as unbumped_ttl_check;
pub use panic_in_contract::check as panic_in_contract_check;

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