## 🎉 What's New in v0.3.0

This release adds two major features that significantly enhance both security and usability:

### 🧠 Sandbox Awareness (`--inject-message`)

Claude Code now understands it's running in a sandboxed environment! No more getting stuck when hitting sandbox limitations.

```bash
# Automatic sandbox awareness (default)
rustyolo claude

# Custom message
rustyolo --inject-message "You're in a restricted environment." claude

# Disable
rustyolo --inject-message none claude
```

The default message informs Claude about all 4 security layers, helping it ask for permission when needed instead of silently failing.

### 🔒 Syscall Filtering (`--seccomp-profile`)

Complete seccomp integration adds kernel-level protection as the **4th security layer**:

```bash
# Uses embedded conservative profile (default)
rustyolo claude

# Maximum security
rustyolo --seccomp-profile ./seccomp/seccomp-restrictive.json claude
```

**Blocks dangerous syscalls** like `ptrace`, `mount`, kernel module loading, system reboots, and privilege escalation vectors.

## 📊 The 4 Security Layers

1. **Filesystem Isolation** - Only mounted directories accessible
2. **Privilege Isolation** - Non-root user with limited permissions
3. **Network Isolation** - DNS + whitelisted domains only
4. **Syscall Isolation** ⭐ NEW - Dangerous syscalls blocked at kernel level

## 📋 Changelog

### Added
- `--inject-message` flag for custom system prompts
- `--seccomp-profile` flag for syscall filtering
- Embedded default seccomp profile (conservative)
- Restrictive seccomp profile option
- Comprehensive seccomp documentation

### Changed
- Updated all docs to reflect 4 security layers
- Enhanced CLI help output
- Refactored code for better maintainability

### Security
- Seccomp blocks privilege escalation syscalls
- Default deny policy for syscalls (allowlist)
- Protection against kernel exploits

## 🔧 Installation

**Homebrew:**
```bash
brew upgrade rustyolo
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

**Manual:**
```bash
rustyolo update --binary --yes
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

## 📖 Documentation

- [Full Documentation](https://github.com/brooksomics/llm-rustyolo/blob/main/CLAUDE.md)
- [Seccomp Guide](https://github.com/brooksomics/llm-rustyolo/blob/main/seccomp/README.md)
- [Release Notes](https://github.com/brooksomics/llm-rustyolo/blob/main/RELEASE_NOTES_v0.3.0.md)

**Full Diff**: https://github.com/brooksomics/llm-rustyolo/compare/v0.2.0...v0.3.0
