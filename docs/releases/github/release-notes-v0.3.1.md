## 🐛 Bug Fix: Homebrew Installation Detection

This patch release fixes a critical bug where `rustyolo update` failed for Homebrew users with the error:
```
[RustyYOLO] Failed to update binary: IoError: No such file or directory (os error 2)
```

### What was fixed?

The installation detection logic now correctly identifies Homebrew-installed binaries, even when running from symlinked locations like `/opt/homebrew/bin/rustyolo`.

**Before:** Running `rustyolo update` would attempt to replace the binary directly and fail.

**After:** Running `rustyolo update` correctly skips the binary update and directs users to use `brew upgrade rustyolo` instead. Only the Docker image is updated (as intended).

### Changes

- Enhanced `detect_installation_method()` to recognize Homebrew symlink directories
- Added symlink resolution to verify binaries point to Homebrew Cellar
- Improved user feedback for Homebrew installations

### Installation

**Via Homebrew:**
```bash
brew upgrade rustyolo
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

**Manual Installation:**
```bash
rustyolo update --binary --yes
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

---

**Full Changelog**: https://github.com/brooksomics/llm-rustyolo/compare/v0.3.0...v0.3.1
