# Changelog

All notable changes to rustyolo will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2025-11-16

### Fixed
- Homebrew installation detection now correctly identifies symlinked binaries ([#16](https://github.com/brooksomics/llm-rustyolo/pull/16))
  - Detects binaries in `/opt/homebrew/bin/` and `/usr/local/bin/`
  - Resolves symlinks to verify they point to Homebrew Cellar
  - Fixes "IoError: No such file or directory" when running `rustyolo update` with Homebrew installations
  - Homebrew users are now correctly directed to use `brew upgrade rustyolo` for CLI updates

## [0.3.0] - 2025-11-15

### Added
- `--inject-message` flag to inject custom system prompts into Claude Code ([#15](https://github.com/brooksomics/llm-rustyolo/pull/15))
  - Default message informs Claude about all 4 security layers
  - Customizable message support
  - Can be disabled with `--inject-message none`
  - Uses Claude Code's `--append-system-prompt` flag
- `--seccomp-profile` flag for syscall filtering via seccomp ([#14](https://github.com/brooksomics/llm-rustyolo/pull/14))
  - Embedded conservative default profile (automatically applied)
  - Support for custom seccomp profiles
  - Can be disabled with `--seccomp-profile none`
- Syscall isolation as 4th security layer ([#14](https://github.com/brooksomics/llm-rustyolo/pull/14))
  - Blocks dangerous syscalls: `ptrace`, `mount`, kernel module loading, etc.
  - Default deny policy (allowlist approach)
  - Two included profiles: conservative (default) and restrictive
- Comprehensive seccomp documentation in `seccomp/README.md` ([#14](https://github.com/brooksomics/llm-rustyolo/pull/14))
- `seccomp/seccomp-default.json` - Conservative profile for Claude Code compatibility
- `seccomp/seccomp-restrictive.json` - Maximum security profile

### Changed
- Updated all documentation references from 3 to 4 security layers ([#15](https://github.com/brooksomics/llm-rustyolo/pull/15))
- Enhanced security messaging to include syscall isolation details ([#15](https://github.com/brooksomics/llm-rustyolo/pull/15))
- Refactored `run_agent` function to satisfy clippy `too_many_lines` lint ([#14](https://github.com/brooksomics/llm-rustyolo/pull/14))
- Updated CLI help documentation to include new flags
- Improved intro tagline to "defense-in-depth security"

### Security
- Seccomp syscall filtering blocks privilege escalation vectors at kernel level
- Protection against process debugging, kernel module loading, and container escape attempts
- Default deny policy ensures only necessary syscalls are allowed

## [0.2.0] - Previous Release

### Added
- Automatic update checking on startup
- `--skip-version-check` flag to disable update checks
- `rustyolo update` command with `--binary`, `--image`, and `--yes` flags
- Homebrew installation detection
- Self-update functionality for manual installations
- Docker image update capability

### Changed
- Improved update workflow for both Homebrew and manual installations
- Enhanced user feedback for update operations

## [0.1.0] - Initial Release

### Added
- Core CLI wrapper for running AI agents in Docker
- Filesystem isolation via volume mounts
- Privilege isolation via non-root user
- Network isolation via iptables firewall
- `--allow-domains` flag for network whitelisting
- `--auth-home` flag for persistent authentication
- Support for Claude Code agent
- Automatic Anthropic API domain whitelisting
- Docker image: `ghcr.io/brooksomics/llm-rustyolo`

### Security
- Three-layer security model (filesystem, privilege, network)
- Dynamic iptables firewall built at container startup
- UID/GID matching between host and container
- Read-only volume mount support

---

[0.3.1]: https://github.com/brooksomics/llm-rustyolo/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/brooksomics/llm-rustyolo/releases/tag/v0.1.0
