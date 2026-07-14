# Contributing to soroban-lint

Thanks for your interest in contributing! This project is part of the **Stellar Wave Program** on [Drips](https://drips.network/wave/stellar), so contributors can earn USDC rewards for merged pull requests.

---

## How to Contribute via Drips Wave

1. Go to the [Stellar Wave Program](https://drips.network/wave/stellar) on Drips
2. Find issues from this repo tagged with the **Stellar Wave** label
3. Click **Apply** on the issue you want to work on
4. Wait to be assigned by the maintainer — **do not start coding before assignment**
5. Fork the repo, make your changes, open a PR
6. Once your PR is merged and the issue is marked resolved, you earn points and rewards!

---

## Setting Up Locally

**Requirements:**
- Rust 1.75+ (`rustup update`)
- Cargo

```bash
git clone https://github.com/YOUR_USERNAME/soroban-lint
cd soroban-lint
cargo build
cargo test
```

---

## How Lint Rules Work

Each lint rule lives in its own file under `src/lints/`. A rule is a function with this signature:

```rust
pub fn check(source: &str, file: &str) -> Vec<LintResult>
```

It receives the raw source code of a `.rs` file and the file path, and returns a list of issues found.

**To add a new lint rule:**

1. Create a new file: `src/lints/your_rule_name.rs`
2. Implement the `check(source: &str, file: &str) -> Vec<LintResult>` function
3. Register it in `src/lints/mod.rs` by adding:
   ```rust
   mod your_rule_name;
   // and inside run_all():
   results.extend(your_rule_name::check(source, file));
   ```
4. Add a test in `tests/` with a sample contract that triggers your rule
5. Document it in `README.md` under the Lint Rules table

---

## Severity Levels

| Level | When to use |
|-------|-------------|
| `"error"` | Will definitely cause a bug or security issue |
| `"warning"` | Likely problematic, should be reviewed |
| `"info"` | Best practice suggestion |

---

## Pull Request Checklist

- [ ] My rule has a clear, specific name (snake_case)
- [ ] I've written at least one test with a sample contract
- [ ] I've added it to the README lint rules table
- [ ] `cargo test` passes
- [ ] `cargo clippy` has no warnings

---

## Code of Conduct

Be respectful, be constructive. This is a learning-friendly project — questions are welcome in the issues!