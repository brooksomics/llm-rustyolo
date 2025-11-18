# Git Hooks

This directory contains git hooks for the project.

## Pre-commit Hook

The pre-commit hook runs before each commit to ensure code quality:

1. **rustfmt** - Checks code formatting
2. **clippy** - Runs linting checks

### Installation

To install the pre-commit hook:

```bash
./hooks/install.sh
```

Or manually:

```bash
cp hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### Usage

Once installed, the hooks will run automatically on `git commit`.

To skip the hooks for a specific commit:

```bash
git commit --no-verify
```

### Fixing Issues

If the pre-commit hook fails:

- **Formatting issues**: Run `cargo fmt` to auto-fix
- **Clippy warnings**: Run `cargo clippy --fix` to auto-fix (some issues)
- **Manual fixes**: Address the issues shown in the error output
