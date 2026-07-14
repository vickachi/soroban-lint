# Pre-Written Issues for Drips Wave

These are ready to create on GitHub. Each one is scoped for a single Wave contributor.
Label each with `Stellar Wave` once your repo is approved on Drips.

---

## ISSUE 1 — [TRIVIAL] Add `hardcoded_address` lint rule

**Label:** `good first issue`, `Stellar Wave`
**Complexity:** Trivial (100 pts)

**Description:**
Implement a new lint rule called `hardcoded_address` that detects when a Stellar address (starting with `G` and 56 characters long) is hardcoded as a string literal inside contract source code.

**Expected behaviour:**
```rust
// Should trigger:
let admin = "GABC...XYZ"; // hardcoded address

// Should NOT trigger:
let admin = env.storage().instance().get(&DataKey::Admin);
```

**Acceptance criteria:**
- New file `src/lints/hardcoded_address.rs`
- Registered in `src/lints/mod.rs`
- At least one passing test
- Added to README lint rules table

---

## ISSUE 2 — [TRIVIAL] Add `unused_storage_key` lint rule

**Label:** `good first issue`, `Stellar Wave`
**Complexity:** Trivial (100 pts)

**Description:**
Implement a lint rule that detects when a storage key enum variant is defined in a `DataKey` enum but never used in a `.set()`, `.get()`, or `.remove()` call anywhere in the file.

**Acceptance criteria:**
- New file `src/lints/unused_storage_key.rs`
- Registered in `src/lints/mod.rs`
- At least one passing test
- Added to README lint rules table

---

## ISSUE 3 — [MEDIUM] Add `--fix` flag skeleton to CLI

**Label:** `Stellar Wave`
**Complexity:** Medium (150 pts)

**Description:**
Add a `--fix` flag to the CLI that, when passed, outputs a message like `Auto-fix not yet available for <rule_name>` for each issue found. This lays the groundwork for future auto-fix functionality.

**Acceptance criteria:**
- `--fix` flag added to the Clap CLI in `main.rs`
- When passed, each result prints a "fix hint" line below the help text
- Tests updated

---

## ISSUE 4 — [MEDIUM] Write integration tests for all existing lint rules

**Label:** `Stellar Wave`
**Complexity:** Medium (150 pts)

**Description:**
Write proper integration tests in the `tests/` folder for all three existing lint rules: `missing_auth_check`, `unbumped_ttl`, and `panic_in_contract`. Each test should include both a "should trigger" and "should not trigger" case.

**Acceptance criteria:**
- Tests live in `tests/` directory
- Each rule has at least 2 test cases (positive + negative)
- All tests pass with `cargo test`

---

## ISSUE 5 — [HIGH] Implement AST-based parsing using `syn`

**Label:** `Stellar Wave`
**Complexity:** High (200 pts)

**Description:**
Currently lint rules use simple string matching, which can produce false positives (e.g. matching commented-out code). Refactor the lint runner to parse Rust source code into an AST using the `syn` crate, and update at least one existing lint rule to use AST traversal instead of string matching.

**Acceptance criteria:**
- `syn` used to parse source into AST
- At least `missing_auth_check` refactored to use AST visitor pattern
- Existing tests still pass
- False positive from commented-out code is fixed

---

## ISSUE 6 — [HIGH] Add SARIF output format support

**Label:** `Stellar Wave`
**Complexity:** High (200 pts)

**Description:**
SARIF (Static Analysis Results Interchange Format) is the standard format used by GitHub Code Scanning to display lint results directly in pull requests. Add a `--format sarif` flag that outputs results as valid SARIF JSON instead of plain text.

**Acceptance criteria:**
- `--format` flag added to CLI (options: `text`, `sarif`)
- SARIF output is valid JSON conforming to the SARIF 2.1.0 schema
- README updated with GitHub Actions usage example