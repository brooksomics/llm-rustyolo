#!/bin/bash
# Install pre-commit hooks for this repository

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_HOOKS_DIR="$(git rev-parse --git-dir)/hooks"

echo "Installing pre-commit hook..."

# Copy the pre-commit hook
cp "$SCRIPT_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
chmod +x "$GIT_HOOKS_DIR/pre-commit"

echo "Pre-commit hook installed successfully!"
echo ""
echo "The hook will run 'cargo fmt' and 'cargo clippy' before each commit."
echo "To skip the hook for a specific commit, use: git commit --no-verify"
