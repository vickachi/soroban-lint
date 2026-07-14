#[test]
fn test_panic_triggers() {
    let contract = r#"
        pub fn withdraw(env: Env, amount: i128) {
            if amount <= 0 {
                panic!("invalid amount");
            }
        }
    "#;

    let path = std::env::temp_dir().join("test_panic.rs");
    let results = soroban_lint::lints::panic_in_contract::check(
        contract,
        path.to_str().unwrap(),
    );
    assert!(!results.is_empty(), "should detect panic! usage");
}

#[test]
fn test_panic_passes_with_result() {
    let contract = r#"
        pub fn withdraw(env: Env, amount: i128) -> Result<(), Error> {
            if amount <= 0 {
                return Err(Error::InvalidAmount);
            }
            Ok(())
        }
    "#;

    let path = std::env::temp_dir().join("test_panic_ok.rs");
    let results = soroban_lint::lints::panic_in_contract::check(
        contract,
        path.to_str().unwrap(),
    );
    assert!(results.is_empty(), "should not flag Result-based errors");
}