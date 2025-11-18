# Release Process for rustyolo

## Overview

This document explains how to create a new release of rustyolo with precompiled binaries for the auto-update functionality.

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- **0.x.y** - Pre-1.0 development versions
  - `0.x.0` - New features
  - `0.x.y` - Bug fixes
- **1.0.0** - First stable release
- **x.y.z** - Production versions
  - `x.0.0` - Breaking changes
  - `x.y.0` - New features (backward compatible)
  - `x.y.z` - Bug fixes

## Creating a Release

### Step 1: Update Version

1. Update the version in `Cargo.toml`:
   ```toml
   [package]
   version = "0.1.0"  # Change this to your new version
   ```

2. Commit the version change:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.1.0"
   git push
   ```

### Step 2: Build Release Binaries

For the auto-update feature to work, you need to build binaries for supported platforms.

#### Linux (x86_64)

```bash
# Already built in this environment
cargo build --release
mkdir -p releases
tar -czf releases/rustyolo-x86_64-unknown-linux-gnu.tar.gz -C target/release rustyolo
```

#### macOS (Intel)

On a macOS machine with Intel CPU:
```bash
cargo build --release
tar -czf rustyolo-x86_64-apple-darwin.tar.gz -C target/release rustyolo
```

#### macOS (Apple Silicon)

On a macOS machine with Apple Silicon:
```bash
cargo build --release
tar -czf rustyolo-aarch64-apple-darwin.tar.gz -C target/release rustyolo
```

#### Cross-compilation (Alternative)

You can use cross-compilation tools like [cross](https://github.com/cross-rs/cross):
```bash
cargo install cross

# Build for macOS (from Linux)
cross build --release --target x86_64-apple-darwin
tar -czf rustyolo-x86_64-apple-darwin.tar.gz -C target/x86_64-apple-darwin/release rustyolo

# Build for macOS ARM (from Linux)
cross build --release --target aarch64-apple-darwin
tar -czf rustyolo-aarch64-apple-darwin.tar.gz -C target/aarch64-apple-darwin/release rustyolo
```

### Step 3: Create GitHub Release

1. **Go to GitHub Releases:**
   - Navigate to: https://github.com/brooksomics/llm-rustyolo/releases
   - Click "Draft a new release"

2. **Create a Tag:**
   - Tag version: `v0.1.0` (must start with 'v' and match Cargo.toml version)
   - Target: `main` branch (or your release branch)

3. **Fill in Release Details:**
   - Release title: `v0.1.0 - Initial Release`
   - Description: Write release notes (see template below)

4. **Attach Binaries:**
   - Drag and drop or upload the `.tar.gz` files:
     - `rustyolo-x86_64-unknown-linux-gnu.tar.gz`
     - `rustyolo-x86_64-apple-darwin.tar.gz` (if available)
     - `rustyolo-aarch64-apple-darwin.tar.gz` (if available)

5. **Publish:**
   - For pre-1.0 versions, check "Set as a pre-release"
   - Click "Publish release"

### Step 4: Test Auto-Update

After publishing, test that auto-update works:

```bash
# With an older version installed
rustyolo update

# Should detect and download the new version
```

## Release Notes Template

```markdown
## What's New in v0.1.0

### Features
- Feature 1 description
- Feature 2 description

### Bug Fixes
- Bug fix 1
- Bug fix 2

### Breaking Changes
- Breaking change 1 (if any)

## Installation

### Download Binary
Download the appropriate binary for your platform from the assets below.

### Via Auto-Update
If you have an older version:
```bash
rustyolo update
```

### From Source
```bash
git clone https://github.com/brooksomics/llm-rustyolo
cd llm-rustyolo
git checkout v0.1.0
cargo build --release
```

## Docker Image
```bash
docker pull llm-rustyolo:latest
```

## Full Changelog
See the [commits since last release](https://github.com/brooksomics/llm-rustyolo/compare/v0.0.1...v0.1.0)
```

## Automation (Future)

Consider setting up GitHub Actions to automate this process:

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Package
        run: |
          tar -czf rustyolo-${{ matrix.target }}.tar.gz \
            -C target/${{ matrix.target }}/release rustyolo

      - name: Upload to Release
        uses: softprops/action-gh-release@v1
        with:
          files: rustyolo-${{ matrix.target }}.tar.gz
```

## Quick Reference

| Platform | Target Triple | Binary Name |
|----------|--------------|-------------|
| Linux (x86_64) | x86_64-unknown-linux-gnu | rustyolo-x86_64-unknown-linux-gnu.tar.gz |
| macOS (Intel) | x86_64-apple-darwin | rustyolo-x86_64-apple-darwin.tar.gz |
| macOS (Apple Silicon) | aarch64-apple-darwin | rustyolo-aarch64-apple-darwin.tar.gz |
| Windows (x86_64) | x86_64-pc-windows-msvc | rustyolo-x86_64-pc-windows-msvc.tar.gz |

## Troubleshooting

### "Asset not found" during auto-update
- Ensure the binary is attached to the release with the exact naming convention
- Check that the tag starts with 'v' (e.g., v0.1.0, not 0.1.0)

### Version mismatch
- Ensure Cargo.toml version matches the git tag (minus the 'v' prefix)

### Platform not detected
- The `self_update` crate auto-detects the platform
- Ensure you're building for the correct target triple
