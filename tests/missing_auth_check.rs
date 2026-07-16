use std::fs;

#[test]
fn test_missing_auth_check_triggers() {
    let contract = r#"
        pub fn transfer(env: Env) {
            env.storage().instance().set(&"key", &"value");
        }
    "#;

    let path = std::env::temp_dir().join("test_missing_auth.rs");
    fs::write(&path, contract).unwrap();

    let results = soroban_lint::lints::missing_auth_check::check(contract, path.to_str().unwrap());
    assert!(!results.is_empty(), "should detect missing auth");
}

#[test]
fn test_missing_auth_check_ignores_comment_mentioning_require_auth() {
    // Regression test: a comment that mentions require_auth() must not
    // silence a real missing-auth violation in the code below it.
    let contract = r#"
        pub fn transfer(env: Env) {
            // TODO: call require_auth() here eventually
            env.storage().instance().set(&"key", &"value");
        }
    "#;

    let path = std::env::temp_dir().join("test_missing_auth_comment.rs");
    let results = soroban_lint::lints::missing_auth_check::check(contract, path.to_str().unwrap());
    assert!(
        !results.is_empty(),
        "a comment mentioning require_auth() should not mask a real violation"
    );
}

#[test]
fn test_missing_auth_check_passes_with_auth() {
    let contract = r#"
        pub fn transfer(env: Env, caller: Address) {
            caller.require_auth();
            env.storage().instance().set(&"key", &"value");
        }
    "#;

    let path = std::env::temp_dir().join("test_auth_ok.rs");
    let results = soroban_lint::lints::missing_auth_check::check(contract, path.to_str().unwrap());
    assert!(results.is_empty(), "should not flag when auth present");
}