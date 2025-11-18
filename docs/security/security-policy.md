# Security Policy

## Secret Scanning and Protection

This repository implements multiple layers of secret detection to prevent accidentally committing sensitive information like API keys, tokens, passwords, and private keys.

### Protection Layers

We use a defense-in-depth approach with three complementary tools:

1. **Pre-commit Hooks** - Block secrets before they're committed locally
2. **GitHub Actions CI** - Scan all commits and PRs in CI/CD
3. **git-secrets** - Additional local protection with custom patterns

## Setup Instructions

### 1. Pre-commit Hooks (Recommended)

Pre-commit hooks run automatically before each commit to catch secrets early.

#### Installation

```bash
# Install pre-commit (choose one method)
pip install pre-commit
# or
brew install pre-commit  # macOS
# or
conda install -c conda-forge pre-commit

# Install the hooks in this repository
pre-commit install
```

#### What's Included

The pre-commit configuration (`.pre-commit-config.yaml`) includes:

- **Gitleaks** - Comprehensive secret scanner with 140+ rules
- **detect-secrets** - Yelp's secret detection with baseline support
- **detect-private-key** - Specifically scans for SSH/GPG private keys
- **detect-aws-credentials** - AWS credential detection
- **General checks** - Trailing whitespace, large files, merge conflicts

#### Usage

```bash
# Runs automatically on git commit
git commit -m "your message"

# Run manually on all files
pre-commit run --all-files

# Run only secret detection hooks
pre-commit run gitleaks --all-files
pre-commit run detect-secrets --all-files

# Update hook versions
pre-commit autoupdate
```

#### Bypassing Hooks (Use with Caution!)

```bash
# Skip all pre-commit hooks (NOT RECOMMENDED)
git commit --no-verify -m "your message"

# Better: Update the baseline for false positives
detect-secrets scan --baseline .secrets.baseline
```

### 2. GitHub Actions Secret Scanning

All pushes and pull requests are automatically scanned by three tools in CI:

- **Gitleaks** - Fast, comprehensive scanning
- **TruffleHog** - Deep scanning with verification
- **detect-secrets** - Additional validation

These workflows run automatically - no setup required! Check the "Actions" tab to see results.

**Workflow file**: `.github/workflows/secret-scanning.yml`

### 3. git-secrets (Optional but Recommended)

git-secrets provides an additional layer of protection with customizable patterns.

#### Installation

```bash
# Run our setup script (installs and configures git-secrets)
./scripts/setup-git-secrets.sh
```

Or install manually:

```bash
# macOS
brew install git-secrets

# Linux (from source)
git clone https://github.com/awslabs/git-secrets.git
cd git-secrets
sudo make install

# Initialize in repository
git secrets --install
git secrets --register-aws
```

#### Usage

```bash
# Scan current changes
git secrets --scan

# Scan entire repository history
git secrets --scan-history

# Add custom patterns
git secrets --add 'YOUR_CUSTOM_PATTERN'

# Add allowed patterns (to reduce false positives)
git secrets --add --allowed 'example\.com'
```

## What Gets Detected

Our tools scan for:

### API Keys and Tokens
- GitHub Personal Access Tokens (PAT)
- AWS Access Keys and Secret Keys
- Google Cloud API keys
- Slack tokens
- NPM tokens
- Discord bot tokens
- JWT tokens
- Generic API keys

### Cloud Provider Credentials
- AWS credentials
- Azure storage keys
- Google Cloud credentials
- IBM Cloud credentials
- Heroku API keys

### Private Keys
- SSH private keys (RSA, DSA, EC, OpenSSH)
- PGP/GPG private keys
- SSL/TLS private keys

### Passwords and Secrets
- Database connection strings
- Basic auth credentials
- Password fields in configuration
- High-entropy strings (likely secrets)

### Other Sensitive Data
- OAuth tokens
- Stripe API keys
- SendGrid API keys
- Twilio API keys
- Mailchimp API keys

## Configuration Files

### `.gitleaks.toml`
Configures gitleaks behavior, including allowlisted files and false positive patterns.

### `.secrets.baseline`
Baseline file for detect-secrets. Contains known false positives that have been audited.

To update the baseline:
```bash
detect-secrets scan --baseline .secrets.baseline
detect-secrets audit .secrets.baseline
```

### `.pre-commit-config.yaml`
Defines all pre-commit hooks, including secret scanners and code quality tools.

## Best Practices

### DO:
- ✅ Use environment variables for secrets
- ✅ Store secrets in `.env` files (already gitignored)
- ✅ Use secret management tools (AWS Secrets Manager, HashiCorp Vault, etc.)
- ✅ Run `pre-commit run --all-files` before pushing
- ✅ Review pre-commit output carefully
- ✅ Update baselines when you have legitimate false positives

### DON'T:
- ❌ Commit actual secrets, even in test files
- ❌ Use `--no-verify` to bypass hooks without good reason
- ❌ Ignore secret scanning failures in CI
- ❌ Store credentials in code comments
- ❌ Commit `.env` files (they're gitignored for a reason!)

## Handling False Positives

### Option 1: Update Gitleaks Config
Add the pattern to `.gitleaks.toml`:

```toml
[allowlist]
regexes = [
    '''your-false-positive-pattern''',
]
```

### Option 2: Update detect-secrets Baseline
```bash
detect-secrets scan --baseline .secrets.baseline
detect-secrets audit .secrets.baseline
```

### Option 3: Add git-secrets Allowed Pattern
```bash
git secrets --add --allowed 'your-pattern'
```

### Option 4: Inline Comment (Gitleaks)
```python
secret = "not-really-a-secret"  # gitleaks:allow pragma: allowlist secret
```

## Emergency: Committed a Secret

If you accidentally committed a secret:

### 1. Immediately Rotate the Secret
The moment a secret is pushed, consider it compromised. Rotate it immediately.

### 2. Remove from Git History
```bash
# Option A: Use BFG Repo-Cleaner (recommended)
brew install bfg
bfg --replace-text secrets.txt  # File containing secrets to remove

# Option B: Use git-filter-repo
git filter-repo --path-filter path/to/file --invert-paths

# Option C: Interactive rebase (for recent commits)
git rebase -i HEAD~N  # N = number of commits back
```

### 3. Force Push (After Team Coordination!)
```bash
git push --force-with-lease
```

### 4. Notify Your Team
Inform everyone who may have pulled the compromised commits.

## GitHub Secret Scanning

GitHub also provides automatic secret scanning at the platform level:

- **Push Protection** - Blocks pushes containing secrets (if enabled)
- **Secret Scanning** - Scans all commits for known secret patterns
- **Partner Patterns** - Automatic notification to service providers

Enable these in your repository settings:
`Settings → Code security and analysis → Secret scanning`

## Testing the Setup

### Test Pre-commit
```bash
# Create a test file with a fake secret (pragma: allowlist secret)
echo "aws_access_key_id=AKIAIOSFODNN7EXAMPLE" > test-secret.txt  # pragma: allowlist secret
git add test-secret.txt
git commit -m "test"  # Should be blocked!
rm test-secret.txt
```

### Test GitHub Actions
The workflows run automatically on every push/PR.

### Test git-secrets
```bash
echo "api_key=EXAMPLE_SECRET_KEY_REPLACE_WITH_REAL_TO_TEST" > test.txt
git secrets --scan test.txt  # Should detect the secret
rm test.txt
```

## Additional Resources

- [Gitleaks Documentation](https://github.com/gitleaks/gitleaks)
- [detect-secrets Documentation](https://github.com/Yelp/detect-secrets)
- [git-secrets Documentation](https://github.com/awslabs/git-secrets)
- [Pre-commit Documentation](https://pre-commit.com/)
- [GitHub Secret Scanning](https://docs.github.com/en/code-security/secret-scanning/about-secret-scanning)

## Reporting Security Issues

If you discover a security vulnerability, please email [your-email] instead of using the issue tracker.

## Questions?

For questions about secret scanning setup, open an issue or check the documentation links above.
