# Contributing to llm-rustyolo

Thank you for your interest in contributing to llm-rustyolo! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Security](#security)

## Code of Conduct

This project follows standard open source community guidelines. Be respectful, constructive, and professional in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/llm-rustyolo.git
   cd llm-rustyolo
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/brooksomics/llm-rustyolo.git
   ```

## Development Setup

### Prerequisites

- **Rust** (latest stable) - Install from [rustup.rs](https://rustup.rs/)
- **Docker** - For testing the container
- **Git** - For version control

### Building the Project

```bash
# Build the CLI
cargo build --release

# Run tests
cargo test

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all
```

### Building the Docker Image

```bash
# Build locally
docker build -t ghcr.io/brooksomics/llm-rustyolo:latest .

# Test the image
docker run -it --rm ghcr.io/brooksomics/llm-rustyolo:latest --help
```

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feat/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring
- `test/description` - Adding or updating tests

### Commit Messages

Follow conventional commit format:

```
<type>: <short description>

<detailed description (optional)>

<footer (optional)>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks (dependencies, CI, etc.)
- `security`: Security improvements

**Examples:**
```
feat: add --dry-run flag for command preview

Add a new --dry-run flag that prints the Docker command without
executing it, helping users understand what will be run.

Closes #123
```

```
fix: resolve IPv6 firewall bypass vulnerability

Disable IPv6 in containers to prevent bypassing iptables rules
that only configure IPv4. This is a critical security fix.

CVE-YYYY-XXXXX
```

## Code Style

### Rust Code

- **Run `cargo fmt`** before committing
- **Run `cargo clippy`** and fix all warnings
- **Add doc comments** to public functions and types
- **Follow Rust naming conventions**:
  - `snake_case` for functions and variables
  - `PascalCase` for types
  - `SCREAMING_SNAKE_CASE` for constants

### Shell Scripts

- Use `shellcheck` for linting
- Follow POSIX compatibility when possible
- Add comments for complex logic

### Documentation

- Use clear, concise language
- Include code examples where helpful
- Update docs when changing functionality
- Keep the CHANGELOG.md up to date

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_validate_volumes
```

### Writing Tests

- Add unit tests for new functions
- Use descriptive test names: `test_<function>_<scenario>`
- Test both success and failure cases
- Include edge cases and security scenarios

**Example:**
```rust
#[test]
fn test_validate_volumes_docker_socket() {
    // Docker socket mounts should be blocked
    let dangerous = vec!["/var/run/docker.sock:/var/run/docker.sock".to_string()];
    let result = validate_volumes(&dangerous);
    assert!(result.is_some());
    assert!(result.unwrap().contains("Docker socket"));
}
```

### Security Testing

For security-related changes:
1. Test with the most restrictive settings
2. Verify that dangerous operations are blocked
3. Add tests that attempt to bypass security measures
4. Document security implications in commit message

## Submitting Changes

### Pull Request Process

1. **Update your fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create a new branch**:
   ```bash
   git checkout -b feat/my-new-feature
   ```

3. **Make your changes**:
   - Write code
   - Add tests
   - Update documentation
   - Run linters and tests

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add my new feature"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feat/my-new-feature
   ```

6. **Create a Pull Request** on GitHub

### Pull Request Checklist

Before submitting, ensure:

- [ ] Code follows the project's code style (`cargo fmt`, `cargo clippy`)
- [ ] All tests pass (`cargo test`)
- [ ] New code has appropriate test coverage
- [ ] Documentation is updated if needed
- [ ] CHANGELOG.md is updated (for significant changes)
- [ ] Commit messages follow conventional format
- [ ] PR description clearly explains the changes
- [ ] Security implications are documented (if applicable)

### PR Description Template

```markdown
## Description
Brief description of what this PR does.

## Motivation
Why is this change needed?

## Changes
- Bullet point list of changes
- Include file modifications
- Note any breaking changes

## Testing
How was this tested?
- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing

## Security Considerations
Any security implications or considerations?

## Related Issues
Closes #123
Relates to #456
```

## Security

### Reporting Security Vulnerabilities

**DO NOT** open a public GitHub issue for security vulnerabilities.

Instead, please email security concerns to the maintainers directly or use GitHub's private security reporting feature.

### Security Guidelines

When contributing code that affects security:

1. **Document security implications** in the PR description
2. **Add security tests** that verify protections work
3. **Consider defense-in-depth** - add multiple layers of protection
4. **Follow principle of least privilege** - minimize permissions
5. **Validate all user input** - especially volume mounts, domains, etc.

### Secret Scanning

This repository uses multiple secret scanning tools. Before committing:

1. Run `cargo test` (includes secret scanning in CI)
2. Ensure no secrets are committed (API keys, tokens, credentials)
3. Use placeholder values in examples

See [docs/security/security-policy.md](docs/security/security-policy.md) for details.

## Development Resources

### Useful Commands

```bash
# Check for outdated dependencies
cargo outdated

# Security audit
cargo install cargo-audit
cargo audit

# Code coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Build release binary
cargo build --release

# Test Docker image
docker build -t test-rustyolo .
docker run -it --rm test-rustyolo claude --help
```

### Documentation

- [Rust Book](https://doc.rust-lang.org/book/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Docker Documentation](https://docs.docker.com/)
- [Seccomp Documentation](https://docs.docker.com/engine/security/seccomp/)

### Project Documentation

- [docs/guides/installation.md](docs/guides/installation.md) - Installation guide
- [docs/security/seccomp.md](docs/security/seccomp.md) - Seccomp profiles
- [docs/development/linting.md](docs/development/linting.md) - Code quality
- [docs/development/release-process.md](docs/development/release-process.md) - Releases

## Questions?

- **Issues**: [GitHub Issues](https://github.com/brooksomics/llm-rustyolo/issues)
- **Discussions**: [GitHub Discussions](https://github.com/brooksomics/llm-rustyolo/discussions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to llm-rustyolo! 🦀🔒
