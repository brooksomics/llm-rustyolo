#!/bin/bash
# Quick setup script for all secret scanning tools
# This script installs and configures all secret detection tools for the repository

set -e

echo "=========================================="
echo "Setting up Secret Scanning Protection"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
    fi
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# 1. Check and install pre-commit
echo "Step 1: Installing pre-commit..."
if command_exists pre-commit; then
    echo "pre-commit is already installed: $(pre-commit --version)"
else
    echo "Installing pre-commit..."
    if command_exists pip; then
        pip install pre-commit
        print_status $? "pre-commit installed via pip"
    elif command_exists pip3; then
        pip3 install pre-commit
        print_status $? "pre-commit installed via pip3"
    elif command_exists brew; then
        brew install pre-commit
        print_status $? "pre-commit installed via brew"
    else
        print_warning "Could not install pre-commit automatically."
        echo "Please install pre-commit manually: https://pre-commit.com/#install"
        exit 1
    fi
fi
echo ""

# 2. Install pre-commit hooks
echo "Step 2: Installing pre-commit hooks..."
pre-commit install
print_status $? "Pre-commit hooks installed in .git/hooks/"
echo ""

# 3. Install detect-secrets (for baseline generation)
echo "Step 3: Installing detect-secrets..."
if command_exists detect-secrets; then
    echo "detect-secrets is already installed: $(detect-secrets --version 2>&1 | head -1)"
else
    echo "Installing detect-secrets..."
    if command_exists pip; then
        pip install detect-secrets
        print_status $? "detect-secrets installed via pip"
    elif command_exists pip3; then
        pip3 install detect-secrets
        print_status $? "detect-secrets installed via pip3"
    else
        print_warning "Could not install detect-secrets automatically."
        echo "Please install it manually: pip install detect-secrets"
    fi
fi
echo ""

# 4. Generate detect-secrets baseline
echo "Step 4: Updating detect-secrets baseline..."
if command_exists detect-secrets; then
    detect-secrets scan --baseline .secrets.baseline > /dev/null 2>&1
    print_status $? "detect-secrets baseline updated"
else
    print_warning "Skipping baseline generation (detect-secrets not installed)"
fi
echo ""

# 5. Setup git-secrets (optional)
echo "Step 5: Setting up git-secrets (optional)..."
if [ -f "./scripts/setup-git-secrets.sh" ]; then
    read -p "Install and configure git-secrets? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ./scripts/setup-git-secrets.sh
    else
        echo "Skipping git-secrets setup"
    fi
else
    print_warning "git-secrets setup script not found"
fi
echo ""

# 6. Test the setup
echo "Step 6: Testing the setup..."
echo "Running pre-commit on all files (this may take a moment)..."
if pre-commit run --all-files; then
    print_status 0 "All pre-commit checks passed!"
else
    print_warning "Some pre-commit checks failed (this is normal for first run)"
    echo "Review the output above to see what needs attention."
fi
echo ""

# Summary
echo "=========================================="
echo "Setup Complete!"
echo "=========================================="
echo ""
echo "What's been configured:"
echo "  ✓ Pre-commit hooks installed"
echo "  ✓ Gitleaks secret scanner"
echo "  ✓ detect-secrets baseline"
echo "  ✓ Private key detection"
echo "  ✓ AWS credential detection"
echo ""
echo "Next steps:"
echo "  1. Review the configuration files:"
echo "     - .pre-commit-config.yaml"
echo "     - .gitleaks.toml"
echo "     - .secrets.baseline"
echo ""
echo "  2. Try making a commit - hooks will run automatically"
echo ""
echo "  3. Review the full documentation:"
echo "     - SECURITY.md"
echo ""
echo "  4. GitHub Actions will automatically scan on push/PR"
echo ""
echo "To manually run all checks:"
echo "  pre-commit run --all-files"
echo ""
