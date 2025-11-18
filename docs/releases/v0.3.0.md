# Release Notes - v0.3.0

## 🎉 Major Features

### Sandbox Awareness via `--inject-message`

Claude Code agents can now understand they're running in a sandboxed environment! The new `--inject-message` flag injects custom system prompts to inform the agent about sandbox constraints.

**Key Benefits:**
- Prevents Claude from getting stuck when it encounters sandbox limitations
- Provides context about all 4 security layers automatically
- Fully customizable for your specific use case
- Can be disabled if you prefer

**Usage:**
```bash
# Default behavior - automatic sandbox awareness message
rustyolo claude

# Custom message
rustyolo --inject-message "You're in a restricted environment. Ask before network operations." claude

# Disable the message
rustyolo --inject-message none claude
```

**Default Message Content:**
The agent is informed about:
- Filesystem isolation (mounted directories only)
- Privilege isolation (non-root with limited permissions)
- Network isolation (DNS + whitelisted domains only)
- Syscall isolation (dangerous syscalls blocked via seccomp)

### Seccomp Syscall Filtering

Full integration of seccomp (secure computing) profiles for defense-in-depth security. This is the **4th layer** of protection in rustyolo's security model.

**Features:**
- Embedded conservative default profile (works automatically)
- `--seccomp-profile` flag for custom profiles
- Blocks dangerous syscalls at the kernel level
- Can be disabled for debugging with `--seccomp-profile none`

**Blocked Syscalls Include:**
- `ptrace` - Process debugging/inspection
- `mount` / `umount` - Filesystem manipulation
- `init_module` / `delete_module` - Kernel module loading
- `reboot` / `kexec_load` - System reboot/kernel execution
- `bpf` - Loading eBPF programs
- `keyctl` - Kernel keyring manipulation
- `perf_event_open` - Performance monitoring (potential info leak)
- Many others (see `seccomp/README.md` for full list)

**Usage:**
```bash
# Default profile (conservative, embedded)
rustyolo claude

# Restrictive profile for maximum security
rustyolo --seccomp-profile ./seccomp/seccomp-restrictive.json claude

# Disable seccomp (not recommended, debugging only)
rustyolo --seccomp-profile none claude
```

## 📚 Documentation Updates

- **Updated security model**: All references now correctly state **4 layers** of security (was 3)
- **New seccomp documentation**: Comprehensive guide in `seccomp/README.md`
- **Enhanced CLI reference**: Both new flags fully documented
- **Usage examples**: Added examples for sandbox message customization
- **Architecture section**: Updated to reflect syscall isolation layer

## 🔒 Security Improvements

This release significantly enhances rustyolo's security posture:

1. **Defense-in-depth**: Seccomp adds kernel-level protection against privilege escalation
2. **Sandbox awareness**: Reduces likelihood of agent getting stuck or confused
3. **Configurable security**: Choose between conservative and restrictive seccomp profiles
4. **Better user experience**: Agents can request permission adjustments when needed

## 📋 Full Changelog

### Added
- `--inject-message` flag for custom system prompts (#15)
- Default sandbox awareness message informing agent about 4 security layers (#15)
- `--seccomp-profile` flag for custom seccomp profiles (#14)
- Embedded default seccomp profile (conservative allowlist) (#14)
- Restrictive seccomp profile for maximum security (#14)
- Comprehensive seccomp documentation in `seccomp/README.md` (#14)
- Syscall isolation as 4th security layer (#14)

### Changed
- Updated all documentation from 3 to 4 security layers (#15)
- Enhanced security messaging to include syscall isolation details (#15)
- Refactored `run_agent` function to satisfy clippy `too_many_lines` lint (#14)
- CLI help output now documents both new flags

### Security
- Seccomp syscall filtering blocks dangerous operations at kernel level (#14)
- Default deny policy for syscalls (allowlist approach) (#14)
- Protection against process debugging, kernel module loading, and privilege escalation (#14)

## 🔧 Technical Details

### New Dependencies
None - all features use existing dependencies

### Breaking Changes
None - fully backward compatible

### Migration Guide
No migration needed. New features are opt-in (seccomp uses safe defaults).

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

## 🙏 Acknowledgments

- Thanks to PR #14 for the comprehensive seccomp implementation
- Inspired by Anthropic's sandbox-runtime and security best practices
- Community feedback on sandbox awareness issues

## 🐛 Known Issues

None at this time. Please report issues at: https://github.com/brooksomics/llm-rustyolo/issues

## 📖 Further Reading

- [Seccomp Documentation](seccomp/README.md)
- [CLAUDE.md](CLAUDE.md) - Full project documentation
- [How It Works](CLAUDE.md#how-it-works) - Detailed security architecture

---

**Full Diff**: https://github.com/brooksomics/llm-rustyolo/compare/v0.2.0...v0.3.0
