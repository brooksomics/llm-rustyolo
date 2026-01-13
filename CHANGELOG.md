# Changelog

All notable changes to rustyolo will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1] - 2026-01-13

### Changed
- **Dependency updates** ([#28](https://github.com/brooksomics/llm-rustyolo/pull/28), [#27](https://github.com/brooksomics/llm-rustyolo/pull/27), [#26](https://github.com/brooksomics/llm-rustyolo/pull/26), [#25](https://github.com/brooksomics/llm-rustyolo/pull/25), [#24](https://github.com/brooksomics/llm-rustyolo/pull/24), [#22](https://github.com/brooksomics/llm-rustyolo/pull/22), [#21](https://github.com/brooksomics/llm-rustyolo/pull/21), [#20](https://github.com/brooksomics/llm-rustyolo/pull/20), [#19](https://github.com/brooksomics/llm-rustyolo/pull/19))
  - Updated `reqwest` from 0.12 to 0.13 for improved HTTP client performance
  - Updated `self_update` to 0.42 for better auto-update functionality
  - Updated `dirs` to 6.0.0 for improved directory path handling
  - Updated `toml` to 0.9 for configuration parsing
  - Updated GitHub Actions workflows:
    - `actions/cache` to v5
    - `actions/checkout` to v6
    - `actions/setup-python` to v6
    - `docker/build-push-action` to v6
  - Updated Docker base image to `node:25-slim`

### Fixed
- **License compliance** ([#30](https://github.com/brooksomics/llm-rustyolo/pull/30), [#29](https://github.com/brooksomics/llm-rustyolo/pull/29))
  - Added OpenSSL license to cargo-deny allowlist
  - Added ISC and CDLA-Permissive-2.0 licenses to cargo-deny allowlist
  - Resolved cargo-deny license validation errors for dependency chain

### Security
- Updated dependencies include security patches and improvements
- Maintained strict license compliance via cargo-deny

## [0.5.0] - 2025-11-19

### Added
- **Configuration file support** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - `.rustyolo.toml` for project-level configuration
  - CLI arguments override config file values
  - Support for default settings, resource limits, and security options
  - `.rustyolo.toml.example` with comprehensive examples
  - `docs/guides/configuration.md` guide
- **Dry-run mode** with `--dry-run` flag ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Preview Docker commands before execution
  - Verify configuration without running containers
  - Useful for debugging and CI/CD pipelines
- **Comprehensive test suite** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - 15 unit tests for volume validation
  - 2 unit tests for seccomp profile setup
  - 4 unit tests for configuration file parsing
  - Total of 21 unit tests
- **Automated dependency management** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Dependabot configuration for Cargo, GitHub Actions, and Docker
  - Automatic weekly security updates
  - Grouped updates for better PR management
- **Security audit CI workflows** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - `cargo-audit` for dependency vulnerability scanning
  - `cargo-deny` for license compliance and security policies
  - Weekly automated scans on schedule
  - Runs on all PRs and pushes to main
- **Contributing guidelines** in `CONTRIBUTING.md` ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Development setup instructions
  - Code style requirements
  - Pull request process
  - Security reporting guidelines
- **License declaration** in `Cargo.toml` ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - MIT license properly declared for cargo-deny compliance

### Changed
- **Documentation reorganization** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Moved 15 documentation files into structured `docs/` hierarchy
  - New structure: `docs/guides/`, `docs/security/`, `docs/development/`
  - Created `docs/README.md` index for easy navigation
  - Cleaner repository root directory
  - Updated all cross-references
- **Enhanced security hardening** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - `--cap-drop=ALL` to remove all Linux capabilities by default
  - `--cap-add=NET_ADMIN` for iptables firewall management (required)
  - `--cap-add=CHOWN` for container permission fixing (required)
  - `--cap-add=SETUID` for gosu user switching (required)
  - `--cap-add=SETGID` for gosu group switching (required)
  - `--security-opt no-new-privileges` to prevent privilege escalation
  - Surgical capability model: deny by default, allow only what's necessary
- **Improved security audit configuration** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Created `deny.toml` with version 2 format
  - Explicit license allowlist (MIT, Apache-2.0, BSD-3-Clause, Unicode-3.0, etc.)
  - Advisory ignore list for documented exceptions
  - Fail on actual vulnerabilities, warn on unmaintained transitive deps
- **Code organization and quality** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Extracted magic strings to named constants
  - Added comprehensive doc comments to security functions
  - Created `src/config.rs` module for configuration logic
  - Improved code formatting and linting compliance

### Fixed
- **Container usermod permission failure** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Added `--cap-add=CHOWN` to allow usermod to change ownership
  - Made usermod command resilient to capability restrictions with `2>/dev/null || true`
  - Ensures permissions are correctly set even if usermod partially fails
- **Container user switching failure** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Added `--cap-add=SETUID` and `--cap-add=SETGID` for gosu
  - Fixes "operation not permitted" error when dropping privileges
  - Ensures agent runs as non-root user successfully
- **CI cargo-deny configuration errors** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Migrated `deny.toml` to version 2 format
  - Fixed deprecated field errors (unlicensed, copyleft, vulnerability)
  - Proper Unicode-3.0 license handling for ICU crates
  - Ignored RUSTSEC-2025-0119 (number_prefix unmaintained) with documentation
- **CI cargo-audit false positives** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Removed `--deny warnings` flag from cargo-audit
  - Now only fails on actual security vulnerabilities
  - Unmaintained crate warnings handled by cargo-deny policy
- **Missing license declaration** ([#PR](https://github.com/brooksomics/llm-rustyolo/pull/PR))
  - Added `license = "MIT"` to Cargo.toml
  - Resolves cargo-deny unlicensed error

### Security
- **Defense-in-depth strengthened**: Surgical Linux capability management
- **Privilege escalation prevention**: `no-new-privileges` security option
- **Automated security monitoring**: cargo-audit and cargo-deny in CI
- **Dependency updates**: Dependabot for automatic security patches
- **License compliance**: Explicit license allowlist with cargo-deny

### Documentation
- Comprehensive configuration guide with 5 example configs
- Contributing guidelines for open source collaboration
- Improved documentation discoverability via hierarchical organization
- Clear security model documentation with capability explanations

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

[0.5.1]: https://github.com/brooksomics/llm-rustyolo/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/brooksomics/llm-rustyolo/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brooksomics/llm-rustyolo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/brooksomics/llm-rustyolo/releases/tag/v0.1.0
