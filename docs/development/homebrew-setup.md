# Homebrew Distribution Setup - Implementation Complete

This document summarizes the implementation of Issue #8 (Homebrew distribution) and provides next steps for publishing.

## ✅ What's Been Implemented

### 1. GitHub Actions Workflow for Release Binaries ✅
- **File**: `.github/workflows/release.yml`
- **What it does**: Automatically builds macOS (Intel and Apple Silicon) and Linux binaries when a new GitHub release is created
- **Platforms supported**:
  - `x86_64-apple-darwin` (macOS Intel)
  - `aarch64-apple-darwin` (macOS Apple Silicon)
  - `x86_64-unknown-linux-gnu` (Linux)
- **Output**: Creates `.tar.gz` files and uploads them to the GitHub release

### 2. Installation Method Detection ✅
- **File**: `src/update.rs`
- **New functions**:
  - `detect_installation_method()` - Detects if rustyolo was installed via Homebrew or manually
  - Modified `update_binary()` - Shows appropriate update instructions based on installation method
- **Behavior**:
  - Homebrew users see: "Use `brew upgrade rustyolo` to update"
  - Manual users can still use: `rustyolo update --binary`
  - Both can use: `rustyolo update --image` for Docker updates

### 3. Homebrew Formula Template ✅
- **Directory**: `homebrew/`
- **Files**:
  - `rustyolo.rb` - The Homebrew formula template with proper structure
  - `README.md` - Comprehensive setup and maintenance instructions
- **Features**:
  - Multi-platform support (macOS Intel/ARM, Linux)
  - SHA256 checksum verification
  - Helpful installation messages (caveats)
  - Docker dependency declaration

### 4. Updated Documentation ✅
- **Files updated**:
  - `README.md` - Homebrew as primary installation method
  - `CLAUDE.md` - Separate sections for Homebrew vs manual installation/updates
  - `INSTALL.md` - Comprehensive guide with both methods
- **Key changes**:
  - Homebrew listed as "Option 1 (Recommended)"
  - Clear separation between update methods
  - Updated all installation examples

## 🚀 Next Steps to Publish

### Step 1: Create the Homebrew Tap Repository

```bash
# Create a new repository on GitHub
# Name: homebrew-rustyolo
# Organization/User: brooksomics

# Clone it locally
git clone https://github.com/brooksomics/homebrew-rustyolo.git
cd homebrew-rustyolo

# Create the Formula directory
mkdir -p Formula

# Copy the formula template
cp /path/to/llm-rustyolo/homebrew/rustyolo.rb Formula/rustyolo.rb

# Create a README
cat > README.md <<'EOF'
# RustyYOLO Homebrew Tap

Homebrew tap for [rustyolo](https://github.com/brooksomics/llm-rustyolo), a secure, firewalled wrapper for running AI agents in YOLO mode.

## Installation

\`\`\`bash
brew tap brooksomics/rustyolo
brew install rustyolo
\`\`\`

Or install directly:

\`\`\`bash
brew install brooksomics/rustyolo/rustyolo
\`\`\`

## Updating

\`\`\`bash
brew upgrade rustyolo
\`\`\`

## Usage

See the [main repository](https://github.com/brooksomics/llm-rustyolo) for usage instructions.
EOF

# Commit and push
git add .
git commit -m "Initial Homebrew formula for rustyolo"
git push origin main
```

### Step 2: Create a GitHub Release

Before users can install via Homebrew, you need to create a release with the binaries:

```bash
# In the llm-rustyolo repository
cd llm-rustyolo

# Make sure all changes are committed
git add .
git commit -m "feat: Add Homebrew distribution support"
git push

# Create a new tag (bump version if needed)
git tag v0.2.0
git push origin v0.2.0

# Create a GitHub release using the gh CLI or web interface
gh release create v0.2.0 \
  --title "v0.2.0 - Homebrew Distribution Support" \
  --notes "## What's New
```

- 🍺 Homebrew distribution support
- 📦 Automated binary builds for macOS and Linux
- 🔍 Installation method detection
- 📚 Updated documentation

## Installation via Homebrew

```bash
brew tap brooksomics/rustyolo
brew install rustyolo
```

## Manual Installation

See the [Installation Guide](https://github.com/brooksomics/llm-rustyolo/blob/main/INSTALL.md) for detailed instructions."
```

The GitHub Actions workflow will automatically build the binaries and attach them to the release.

### Step 3: Update the Homebrew Formula with SHA256 Checksums

After the release is created and binaries are uploaded:

```bash
# Download the release assets
cd /tmp
wget https://github.com/brooksomics/llm-rustyolo/releases/download/v0.2.0/rustyolo-x86_64-apple-darwin.tar.gz
wget https://github.com/brooksomics/llm-rustyolo/releases/download/v0.2.0/rustyolo-aarch64-apple-darwin.tar.gz
wget https://github.com/brooksomics/llm-rustyolo/releases/download/v0.2.0/rustyolo-x86_64-unknown-linux-gnu.tar.gz

# Calculate checksums
shasum -a 256 rustyolo-x86_64-apple-darwin.tar.gz
shasum -a 256 rustyolo-aarch64-apple-darwin.tar.gz
shasum -a 256 rustyolo-x86_64-unknown-linux-gnu.tar.gz

# Update the formula
cd homebrew-rustyolo

# Edit Formula/rustyolo.rb and:
# 1. Update version to "0.2.0"
# 2. Update all URLs from v0.1.1 to v0.2.0
# 3. Add the SHA256 checksums from above

# Commit and push
git add Formula/rustyolo.rb
git commit -m "Update rustyolo to v0.2.0 with checksums"
git push origin main
```

### Step 4: Test the Installation

```bash
# On a test machine (or use a Docker container)
brew tap brooksomics/rustyolo
brew install rustyolo

# Verify
rustyolo --version

# Test the installation method detection
rustyolo update --binary
# Should show: "rustyolo was installed via Homebrew. To update, run: brew upgrade rustyolo"
```

### Step 5: Announce and Document

Update the main README to announce Homebrew support:

```markdown
## 🎉 Now Available via Homebrew!

\`\`\`bash
brew tap brooksomics/rustyolo
brew install rustyolo
\`\`\`

See the [Installation Guide](./INSTALL.md) for more details.
```

## 📋 Checklist Before Publishing

- [ ] Create `brooksomics/homebrew-rustyolo` repository on GitHub
- [ ] Copy formula template to the tap repository
- [ ] Bump version in `Cargo.toml` to `0.2.0` (or appropriate version)
- [ ] Commit all changes in `llm-rustyolo` repository
- [ ] Create and push a new git tag (e.g., `v0.2.0`)
- [ ] Create a GitHub release (triggers binary builds)
- [ ] Wait for GitHub Actions to complete and upload binaries
- [ ] Download binaries and calculate SHA256 checksums
- [ ] Update Homebrew formula with correct version, URLs, and checksums
- [ ] Test installation via Homebrew
- [ ] Test update detection works correctly
- [ ] Update main README with announcement
- [ ] Close issue #8 on GitHub

## 🔄 Future Releases

For future releases, follow these steps:

1. **Update version** in `Cargo.toml`
2. **Commit changes** to main branch
3. **Create new tag and release** (GitHub Actions handles binary builds)
4. **Calculate new checksums** for the release assets
5. **Update Homebrew formula** in the tap repository
6. **Test the upgrade** works correctly

You can optionally automate this with a GitHub Action in the tap repository (see `homebrew/README.md` for details).

## 📚 Related Files

- `.github/workflows/release.yml` - Automated binary builds
- `src/update.rs` - Installation method detection
- `homebrew/rustyolo.rb` - Homebrew formula template
- `homebrew/README.md` - Detailed Homebrew setup instructions
- `README.md` - Updated with Homebrew installation
- `CLAUDE.md` - Updated with separate update instructions
- `INSTALL.md` - Comprehensive installation guide

## 🎯 Success Criteria Met

- ✅ Easy installation: `brew install brooksomics/rustyolo/rustyolo`
- ✅ Easy updates: `brew upgrade rustyolo`
- ✅ Low maintenance: Automated binary builds via GitHub Actions
- ✅ macOS-first: Both Intel and Apple Silicon supported
- ✅ Installation method detection: No version conflicts
- ✅ Clear error messages: Users know which update method to use

## 🐛 Known Issues / Limitations

- Docker image is still self-hosted (by design, per issue #7 decision)
- Homebrew formula requires manual SHA256 updates for each release
  - Can be automated with GitHub Actions (optional)
- Formula caveat text mentions building Docker image locally
  - Users still need to clone repo and run `docker build`
  - Could be improved by publishing Docker image to ghcr.io (future work)

## 📞 Support

If users encounter issues:
1. Check they have Docker installed and running
2. Verify Homebrew is up to date: `brew update`
3. Try reinstalling: `brew uninstall rustyolo && brew install rustyolo`
4. Check for issues at: https://github.com/brooksomics/llm-rustyolo/issues
