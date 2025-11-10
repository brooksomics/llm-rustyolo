#!/bin/bash
# Setup script for git-secrets
# This script installs and configures git-secrets for the repository

set -e

echo "Setting up git-secrets for llm-rustyolo..."

# Check if git-secrets is installed
if ! command -v git-secrets &> /dev/null; then
    echo "git-secrets is not installed. Installing..."

    # Detect OS and install accordingly
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        if command -v brew &> /dev/null; then
            brew install git-secrets
        else
            echo "Error: Homebrew not found. Please install git-secrets manually."
            echo "Visit: https://github.com/awslabs/git-secrets"
            exit 1
        fi
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        echo "Installing git-secrets from source..."
        cd /tmp
        git clone https://github.com/awslabs/git-secrets.git
        cd git-secrets
        sudo make install
        cd -
        rm -rf /tmp/git-secrets
    else
        echo "Error: Unsupported OS. Please install git-secrets manually."
        echo "Visit: https://github.com/awslabs/git-secrets"
        exit 1
    fi
fi

echo "git-secrets is installed: $(which git-secrets)"

# Initialize git-secrets in the repository
echo "Initializing git-secrets in repository..."
git secrets --install -f

# Register AWS patterns
echo "Registering AWS secret patterns..."
git secrets --register-aws

# Add custom patterns for common secrets
echo "Adding custom secret patterns..."

# API keys and tokens
git secrets --add '[aA]pi[_-]?[kK]ey.*[=:]\s*["\047]?[a-zA-Z0-9]{20,}["\047]?'
git secrets --add '[aA]pi[_-]?[tT]oken.*[=:]\s*["\047]?[a-zA-Z0-9]{20,}["\047]?'
git secrets --add '[aA]ccess[_-]?[tT]oken.*[=:]\s*["\047]?[a-zA-Z0-9]{20,}["\047]?'

# Private keys (match the BEGIN keyword to avoid leading dash issues)
git secrets --add 'BEGIN\s+(RSA|DSA|EC|OPENSSH)\s+PRIVATE\s+KEY'

# Generic secrets
git secrets --add '[sS]ecret.*[=:]\s*["\047]?[a-zA-Z0-9]{20,}["\047]?'
git secrets --add '[pP]assword.*[=:]\s*["\047]?[^\s]{8,}["\047]?'

# GitHub tokens
git secrets --add 'ghp_[a-zA-Z0-9]{36}'
git secrets --add 'gho_[a-zA-Z0-9]{36}'
git secrets --add 'github_pat_[a-zA-Z0-9]{22}_[a-zA-Z0-9]{59}'

# Slack tokens
git secrets --add 'xox[baprs]-[0-9]{10,12}-[0-9]{10,12}-[a-zA-Z0-9]{24,32}'

# Docker/NPM tokens
git secrets --add 'npm_[a-zA-Z0-9]{36}'

# Add allowed patterns (to reduce false positives)
echo "Adding allowed patterns..."
git secrets --add --allowed 'example\.com'
git secrets --add --allowed 'localhost'
git secrets --add --allowed 'EXAMPLE_'
git secrets --add --allowed 'YOUR_.*_HERE'
git secrets --add --allowed 'TODO'
git secrets --add --allowed 'FIXME'

echo ""
echo "✅ git-secrets setup complete!"
echo ""
echo "git-secrets will now scan commits for secrets automatically."
echo "To manually scan the repository, run:"
echo "  git secrets --scan"
echo ""
echo "To scan the entire history:"
echo "  git secrets --scan-history"
echo ""
