# Linting and Code Quality

This project uses Rust's official linting and formatting tools to maintain code quality.

## Tools

### 1. rustfmt (Formatter)
The official Rust code formatter. Configuration is in `rustfmt.toml`.

**Usage:**
```bash
# Format all code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

**Configuration:** See `rustfmt.toml` for current settings. The configuration uses only stable features. If you're using nightly Rust, you can uncomment the additional options at the bottom of the file.

### 2. clippy (Linter)
The official Rust linter that catches common mistakes and suggests improvements. Configuration is in `Cargo.toml` under `[lints.clippy]`.

**Usage:**
```bash
# Run clippy
cargo clippy

# Run clippy with all checks (same as CI)
cargo clippy --all-targets --all-features -- -D warnings

# Auto-fix some issues
cargo clippy --fix
```

**Lint Levels:**
- `correctness` - Denied (errors that will break your code)
- `suspicious` - Warned (code that looks wrong)
- `complexity` - Warned (unnecessarily complex code)
- `perf` - Warned (potential performance issues)
- `style` - Warned (style improvements)
- `pedantic` - Warned (best practices, some rules disabled)

## Pre-commit Hooks

Pre-commit hooks automatically run `rustfmt` and `clippy` before each commit.

### Installation

```bash
# Run the installation script
./hooks/install.sh
```

Or manually:
```bash
cp hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### Usage

Once installed, the hooks run automatically on `git commit`. If checks fail:

1. **Formatting issues:** Run `cargo fmt` and commit again
2. **Clippy warnings:** Fix the issues or run `cargo clippy --fix`
3. **Skip hooks (not recommended):** Use `git commit --no-verify`

See `hooks/README.md` for more details.

## Continuous Integration (CI)

GitHub Actions automatically runs all checks on push and pull requests. See `.github/workflows/rust-ci.yml` for the full workflow.

The CI runs:
1. **Format Check** - Ensures code is formatted with `rustfmt`
2. **Clippy Lints** - Runs all linting checks with warnings denied
3. **Tests** - Runs the test suite
4. **Build** - Ensures the release build succeeds

**Important:** CI uses `-D warnings` to treat all warnings as errors. Fix all clippy warnings before pushing.

## Quick Reference

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Run linter (strict, same as CI)
cargo clippy --all-targets --all-features -- -D warnings

# Fix auto-fixable issues
cargo clippy --fix

# Run tests
cargo test

# Build release
cargo build --release
```

## Common Issues

### "Clippy warnings are errors in CI but not locally"

Run clippy with `-D warnings` to match CI behavior:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### "Formatting looks different on CI"

Make sure you're running the latest stable Rust:
```bash
rustup update stable
```

### "Want to use nightly rustfmt features"

1. Install nightly: `rustup install nightly`
2. Uncomment nightly features in `rustfmt.toml`
3. Run with nightly: `cargo +nightly fmt`

Note: CI uses stable Rust, so keep nightly features commented for production.

## Customization

### Modify rustfmt settings

Edit `rustfmt.toml`. See [rustfmt documentation](https://rust-lang.github.io/rustfmt/) for all options.

### Modify clippy settings

Edit the `[lints.clippy]` section in `Cargo.toml`. See [clippy documentation](https://rust-lang.github.io/rust-clippy/stable/index.html) for all lints.

### Disable specific clippy warnings

Add `#[allow(clippy::lint_name)]` above the code:
```rust
#[allow(clippy::must_use_candidate)]
fn my_function() -> String {
    // ...
}
```

Or disable for the whole file:
```rust
#![allow(clippy::must_use_candidate)]
```
