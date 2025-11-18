# Release Notes: v0.4.0 - Critical Security Update

## 🔒 Security-Focused Release

Version 0.4.0 addresses **7 critical and high-severity security issues** identified through comprehensive security review. This release significantly strengthens all four defense-in-depth security layers.

## ⚠️ CRITICAL Security Fixes

### 1. IPv6 Firewall Bypass (CVE-level)
**Severity:** CRITICAL

**Issue:** The firewall was only configured for IPv4 (iptables), allowing complete bypass of all network restrictions if IPv6 was enabled in the container.

**Fix:** IPv6 is now disabled via sysctl (`net.ipv6.conf.all.disable_ipv6=1`), eliminating this attack vector entirely.

**Impact:** Prevents complete network isolation bypass.

### 2. Command Injection Vulnerability
**Severity:** CRITICAL

**Issue:** User-supplied domain names were passed to shell commands (`dig`) without validation, allowing arbitrary command execution via crafted input like `domain; rm -rf /`.

**Fix:** All domains and IPs are now validated against strict regex patterns before processing. Invalid input is logged and skipped.

**Impact:** Prevents arbitrary code execution via malicious domain names.

### 3. DNS Resolution Configuration Bug
**Severity:** HIGH

**Issue:** Docker wasn't configured to use the allowed DNS servers, causing DNS resolution failures when DNS restrictions were enabled.

**Fix:** Added `--dns` flags to docker run command to ensure container uses specified DNS servers.

**Impact:** Fixes DNS resolution while maintaining DNS exfiltration protections.

## 🚀 New Security Features

### Resource Limits (DoS Prevention)
Protect your host system from resource exhaustion attacks, crypto mining, and fork bombs.

**New flags:**
- `--memory` (default: 4g) - Limit container memory usage
- `--cpus` (default: 4) - Limit CPU cores available
- `--pids-limit` (default: 256) - Prevent process explosion

**Usage:**
```bash
# Use defaults (recommended)
rustyolo claude

# Custom limits for large projects
rustyolo --memory 8g --cpus 8 claude

# Disable limits (not recommended)
rustyolo --memory unlimited claude
```

### DNS Exfiltration Prevention
Restrict DNS queries to trusted nameservers only, preventing DNS tunneling attacks.

**New flag:**
- `--dns-servers` (default: `8.8.8.8 8.8.4.4 1.1.1.1 1.0.0.1`)

**Usage:**
```bash
# Use default trusted DNS servers (Google + Cloudflare)
rustyolo claude

# Custom DNS servers
rustyolo --dns-servers "8.8.8.8 1.1.1.1" claude

# Disable restrictions (not recommended)
rustyolo --dns-servers any claude
```

**Security benefit:** Prevents data exfiltration via DNS queries to attacker-controlled nameservers.

### Audit Logging
Enable forensic logging to detect attacks and debug security policy issues.

**New flag:**
- `--audit-log [none|basic|verbose]`

**Levels:**
- `none` (default) - No overhead, clean logs
- `basic` - Log blocked network connections only
- `verbose` - Log all security events (allowed + blocked)

**Usage:**
```bash
# Enable basic logging
rustyolo --audit-log basic claude

# View logs after session
docker ps -a  # Get container ID
docker logs <container-id> | grep AUDIT

# Example log output:
# [AUDIT-BLOCK] SRC=172.17.0.2 DST=1.2.3.4 PROTO=TCP DPT=443
# [AUDIT-DNS-ALLOW] SRC=172.17.0.2 DST=8.8.8.8 PROTO=UDP DPT=53
```

**Security benefit:** Detect exfiltration attempts, debug connection failures, maintain audit trail.

### Volume Mount Validation
Prevent container escape via dangerous volume mounts.

**Blocked mounts:**
- Docker socket (`/var/run/docker.sock`) - Enables complete container escape
- System directories (`/proc`, `/sys`, `/dev`, `/boot`, `/etc`) - Enables system compromise

**Usage:**
```bash
# These will be rejected with clear error messages:
rustyolo -v /var/run/docker.sock:/var/run/docker.sock claude  # ❌ Blocked
rustyolo -v /proc:/proc claude                                # ❌ Blocked

# These are allowed:
rustyolo -v ~/.ssh:/home/agent/.ssh:ro claude                 # ✅ Allowed
rustyolo -v /home/user/myproc:/app/myproc claude              # ✅ Allowed (smart detection)
```

**Security benefit:** Prevents the most common container escape technique.

## 📚 Documentation Improvements

### New Troubleshooting Section
- **Dynamic IP Resolution Issue**: Comprehensive guide with workarounds
  - Explains why connections may fail mid-session with CDN-backed services
  - Provides simple container restart workaround
  - Shows how to use audit logging to confirm the issue

### Updated Security Documentation
- Added dynamic IP limitation to "What This Does NOT Protect Against"
- Updated resource limits section with new CLI flags
- Provided clear examples for all new features

## 🔧 Other Improvements

### Enhanced Bash Security
- Upgraded from `set -e` to `set -euo pipefail` in entrypoint.sh
- Fail on undefined variables and pipeline errors
- More robust error handling

## 📦 Upgrade Instructions

### For Homebrew Users
```bash
# Update the CLI
brew upgrade rustyolo

# Update the Docker image
rustyolo update --image
```

### For Manual Installations
```bash
# Update the CLI binary
rustyolo update --binary

# Or update both
rustyolo update
```

### For Source Builds
```bash
git pull
cargo build --release
cp target/release/rustyolo /usr/local/bin/
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

## ⚠️ Breaking Changes

**None.** All new features have sensible defaults and existing usage patterns continue to work.

## 🔐 Security Summary

**Before v0.4.0:**
- 2 critical vulnerabilities (IPv6 bypass, command injection)
- 3 high-severity issues (no resource limits, DNS exfiltration, socket mount escape)
- 2 medium issues (no audit logging, dynamic IP limitation)

**After v0.4.0:**
- ✅ All critical vulnerabilities patched
- ✅ All high-severity issues addressed
- ✅ Medium issues mitigated with features and documentation
- ✅ All 4 security layers significantly hardened

## 🙏 Acknowledgments

This release was driven by a comprehensive security review identifying critical issues and missing features. Special thanks to the security community for best practices around container isolation.

## 📋 Full Changelog

See [CHANGELOG.md](./CHANGELOG.md) for complete details.

## 🐛 Known Issues

- **Dynamic IP Resolution**: Domains are resolved to IPs once at startup. For long sessions (1+ hours), CDN IP changes may cause connection failures. **Workaround:** Restart the container or use audit logging to detect the issue.

## 💬 Questions or Issues?

- **Bug reports:** [GitHub Issues](https://github.com/brooksomics/llm-rustyolo/issues)
- **Security issues:** Please report privately via email
- **Documentation:** See [CLAUDE.md](./CLAUDE.md) for comprehensive usage guide

---

**Recommendation:** This is a critical security update. All users should upgrade immediately.
