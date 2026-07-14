#[test]
fn test_unbumped_ttl_triggers() {
    let contract = r#"
        pub fn store(env: Env) {
            env.storage().persistent().set(&"key", &"value");
        }
    "#;

    let path = std::env::temp_dir().join("test_ttl.rs");
    let results = soroban_lint::lints::unbumped_ttl::check(
        contract,
        path.to_str().unwrap(),
    );
    assert!(!results.is_empty(), "should detect missing TTL bump");
}

#[test]
fn test_unbumped_ttl_passes_with_extend() {
    let contract = r#"
        pub fn store(env: Env) {
            env.storage().persistent().set(&"key", &"value");
            env.storage().persistent().extend_ttl(&"key", 100, 100);
        }
    "#;

    let path = std::env::temp_dir().join("test_ttl_ok.rs");
    let results = soroban_lint::lints::unbumped_ttl::check(
        contract,
        path.to_str().unwrap(),
    );
    assert!(results.is_empty(), "should not flag when TTL extended");
}