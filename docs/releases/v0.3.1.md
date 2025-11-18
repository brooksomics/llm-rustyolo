# Release Notes - v0.3.1

## 🐛 Bug Fix Release

This patch release fixes a critical bug in the Homebrew installation detection logic that prevented `rustyolo update` from working correctly for Homebrew users.

## Fixed Issues

### Homebrew Installation Detection for Symlinked Binaries

**Problem:**
When users installed rustyolo via Homebrew, running `rustyolo update` would fail with:
```
[RustyYOLO] Failed to update binary: IoError: No such file or directory (os error 2)
```

**Root Cause:**
The installation detection logic only checked if the binary path contained `/Cellar/rustyolo/`, but Homebrew creates symlinks in `/opt/homebrew/bin/` or `/usr/local/bin/`. When running the symlinked binary, the detection failed to recognize it as a Homebrew installation.

**Solution:**
Enhanced the `detect_installation_method()` function to:
- Detect when binaries are located in Homebrew symlink directories (`/opt/homebrew/bin/`, `/usr/local/bin/`, `/home/linuxbrew/.linuxbrew/bin/`)
- Attempt to resolve symlinks to verify they point to Homebrew Cellar
- Fall back to treating any binary in Homebrew bin directories as Homebrew-installed

**Impact:**
Homebrew users will now see the correct behavior when running `rustyolo update`:
- The command will skip binary updates and display a helpful message
- Users are correctly directed to use `brew upgrade rustyolo` for CLI updates
- Only the Docker image is updated (as intended)

## 📋 Full Changelog

### Fixed
- Homebrew installation detection now correctly identifies symlinked binaries
  - Detects binaries in `/opt/homebrew/bin/` and `/usr/local/bin/`
  - Resolves symlinks to verify they point to Homebrew Cellar
  - Fixes "IoError: No such file or directory" when running `rustyolo update`
  - Homebrew users are now correctly directed to use `brew upgrade rustyolo`

## 📦 Installation

### Via Homebrew (recommended)
```bash
brew upgrade rustyolo
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

### Manual Binary Update
```bash
rustyolo update --binary --yes
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

### From Source
```bash
git pull
cargo build --release
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

## 🔧 Technical Details

### Files Changed
- `src/update.rs` - Enhanced `detect_installation_method()` function

### No Breaking Changes
This is a pure bug fix release with full backward compatibility.

## 🙏 Acknowledgments

Thanks to the user who reported this issue! Your feedback helps make rustyolo better for everyone.

## 📖 Further Reading

- [CHANGELOG.md](CHANGELOG.md) - Full version history
- [CLAUDE.md](CLAUDE.md) - Complete project documentation
- [Updating rustyolo](CLAUDE.md#updating) - Update documentation

---

**Full Diff**: https://github.com/brooksomics/llm-rustyolo/compare/v0.3.0...v0.3.1
