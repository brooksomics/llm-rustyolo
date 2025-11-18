# Changelog

All notable changes to rustyolo will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-11-16

### Added
- **Resource limits** with configurable defaults ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - `--memory` flag (default: 4g) - Prevents memory exhaustion attacks
  - `--cpus` flag (default: 4) - Prevents CPU monopolization
  - `--pids-limit` flag (default: 256) - Prevents fork bombs
  - All limits can be customized or disabled with `unlimited`
- **DNS server restrictions** to prevent exfiltration ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - `--dns-servers` flag (default: Google & Cloudflare public DNS)
  - Restricts DNS queries to trusted nameservers only
  - Prevents DNS tunneling data exfiltration attacks
  - Option to disable with `--dns-servers any` (not recommended)
- **Audit logging** for security forensics and debugging ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - `--audit-log` flag with `none`, `basic`, `verbose` levels
  - Basic: Logs blocked network connections
  - Verbose: Logs all security events (allowed + blocked)
  - Accessible via `docker logs <container-id>`
- **Volume validation** to prevent container escape ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Blocks Docker socket mounts (`/var/run/docker.sock`)
  - Blocks dangerous system directories (`/proc`, `/sys`, `/dev`, `/boot`, `/etc`)
  - Clear error messages explaining security risks
  - Validates all user-supplied volume mounts before execution

### Fixed
- **CRITICAL: IPv6 firewall bypass** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Firewall was only configured for IPv4, allowing complete bypass via IPv6
  - Fix: Disable IPv6 via sysctl to eliminate the attack vector
  - Eliminates an entire class of network-based attacks
- **CRITICAL: Command injection vulnerability** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - User-supplied domains were passed to shell without validation
  - Fix: Validate domains against `^[a-zA-Z0-9._-]+$` before processing
  - Prevents arbitrary command execution via malicious domain names
- **DNS resolution failure** with restricted DNS servers ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Docker wasn't configured to use the allowed DNS servers
  - Fix: Add `--dns` flags to docker run command
  - Ensures container uses correct DNS servers for resolution

### Changed
- Improved bash defensive practices in `entrypoint.sh`
  - Changed from `set -e` to `set -euo pipefail`
  - Fail on undefined variables and pipeline errors
- Enhanced security documentation in `CLAUDE.md`
  - Added dynamic IP resolution limitation to "What This Does NOT Protect Against"
  - New troubleshooting section for mid-session connection failures
  - Updated resource limits documentation with new CLI flags
  - Clear workarounds and examples for common issues

### Security
- **Defense-in-depth improvements**: All 4 security layers significantly hardened
- **IPv6 attack vector eliminated**: Complete IPv6 bypass vulnerability patched
- **Command injection prevented**: Input validation for all user-supplied domains and IPs
- **DNS exfiltration mitigated**: DNS traffic restricted to trusted public nameservers
- **Container escape blocked**: Dangerous volume mounts detected and rejected
- **Resource DoS prevention**: CPU, memory, and PID limits protect host system
- **Audit capabilities**: Optional logging enables attack detection and forensics

### Documentation
- Added comprehensive troubleshooting for dynamic IP issues
- Documented DNS server restrictions and configuration
- Explained resource limit defaults and customization
- Provided audit logging usage examples
- Clarified known limitations with workarounds

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

[0.4.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/brooksomics/llm-rustyolo/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/brooksomics/llm-rustyolo/releases/tag/v0.1.0
