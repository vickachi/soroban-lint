use crate::runner::LintResult;

/// Detects contracts that use panic! but have no ContractError enum defined
pub fn check(source: &str, file: &str) -> Vec<LintResult> {
    let mut results = Vec::new();

    let has_panic = source.contains("panic!(");
    let has_error_enum = source.contains("enum ContractError")
        || source.contains("enum Error")
        || source.contains("contracterror");

    if has_panic && !has_error_enum {
        results.push(LintResult {
            rule: "missing_error_type".to_string(),
            severity: "warning".to_string(),
            file: file.to_string(),
            line: 1,
            message: "Contract uses panic!() but has no error enum defined.".to_string(),
            help: "Define a ContractError enum with #[contracterror] attribute and return Result<T, ContractError> instead of panicking.".to_string(),
        });
    }

    results
}