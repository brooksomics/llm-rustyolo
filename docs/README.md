# llm-rustyolo Documentation

Welcome to the llm-rustyolo documentation! This directory contains comprehensive guides, security documentation, and development resources.

## 📚 Getting Started

- **[Installation Guide](guides/installation.md)** - Detailed setup instructions for Homebrew and manual installation
- **[Main README](../README.md)** - Project overview and quick start
- **[CLAUDE.md](../CLAUDE.md)** - Complete documentation on architecture, security layers, and advanced usage

## 🔒 Security

- **[Security Policy](security/security-policy.md)** - Secret scanning setup and security best practices
- **[Seccomp Profiles](security/seccomp.md)** - Syscall filtering configuration and custom profiles

## 📖 Guides

- **[Installation](guides/installation.md)** - Step-by-step installation for all platforms
- **[Hooks](guides/hooks.md)** - Pre-commit hooks and secret scanning setup

## 🛠️ Development

- **[Release Process](development/release-process.md)** - How to create and publish releases
- **[Homebrew Setup](development/homebrew-setup.md)** - Publishing to Homebrew tap
- **[Linting](development/linting.md)** - Code quality tools and standards
- **[Release Checklists](development/release-checklists/)** - Version-specific release procedures

## 📝 Release Notes

- **[v0.4.0](releases/v0.4.0.md)** - Security hardening and DNS restrictions
- **[v0.3.1](releases/v0.3.1.md)** - Bug fixes and improvements
- **[v0.3.0](releases/v0.3.0.md)** - Resource limits and audit logging
- **[v0.1.0](releases/v0.1.0.md)** - Initial release
- **[GitHub Release Notes](releases/github/)** - Formatted release notes for GitHub

## 📋 Project Files

Key project files in the root directory:

- **[CLAUDE.md](../CLAUDE.md)** - Claude Code specific context and detailed documentation
- **[CHANGELOG.md](../CHANGELOG.md)** - Chronological list of all changes
- **[CONTRIBUTING.md](../CONTRIBUTING.md)** - Contribution guidelines
- **[README.md](../README.md)** - Main project README

## 🏗️ Architecture Overview

llm-rustyolo implements **four independent security layers**:

1. **Filesystem Isolation** - Only mounted directories are visible
2. **Privilege Isolation** - Runs as non-root user with matched UID/GID
3. **Network Isolation** - iptables firewall with domain whitelist
4. **Syscall Isolation** - Seccomp blocks 40+ dangerous syscalls

See [CLAUDE.md](../CLAUDE.md) for detailed architecture documentation.

## 🤝 Contributing

Want to contribute? Check out:

- **[CONTRIBUTING.md](../CONTRIBUTING.md)** - Contribution guidelines and development setup
- **[Linting Guide](development/linting.md)** - Code quality standards
- **[Release Process](development/release-process.md)** - How releases are managed

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/brooksomics/llm-rustyolo/issues)
- **Discussions**: [GitHub Discussions](https://github.com/brooksomics/llm-rustyolo/discussions)
- **License**: MIT License

---

**Last Updated**: 2025-11-18
