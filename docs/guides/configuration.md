# Configuration File Guide

RustyYOLO supports project-level configuration files to avoid typing long command-line arguments repeatedly. This guide explains how to create and use `.rustyolo.toml` configuration files.

## Quick Start

1. **Copy the example configuration**:
   ```bash
   cp .rustyolo.toml.example .rustyolo.toml
   ```

2. **Edit `.rustyolo.toml`** to customize your settings

3. **Run rustyolo** - it will automatically load the configuration:
   ```bash
   rustyolo claude
   ```

## How It Works

- **Automatic Loading**: If `.rustyolo.toml` exists in the current directory, it's loaded automatically
- **CLI Override**: Command-line arguments always take precedence over config file settings
- **Gitignored**: `.rustyolo.toml` is gitignored by default, so project-specific settings won't be committed

## Configuration File Structure

The configuration file uses TOML format with three main sections:

### `[default]` Section

Default runtime configuration:

```toml
[default]
# Space-separated domains to allow network access
allow_domains = "github.com pypi.org npmjs.org"

# Volume mounts (array of strings)
volumes = [
    "~/.ssh:/home/agent/.ssh:ro",
    "~/.gitconfig:/home/agent/.gitconfig:ro"
]

# Environment variables (array of KEY=VALUE strings)
env = ["MY_VAR=value", "DEBUG=true"]

# Persistent auth directory
auth_home = "~/.config/rustyolo"

# Docker image (defaults to ghcr.io/brooksomics/llm-rustyolo:latest)
image = "ghcr.io/brooksomics/llm-rustyolo:latest"

# Agent to run (defaults to "claude")
agent = "claude"
```

### `[resources]` Section

Resource limits to prevent DoS attacks:

```toml
[resources]
# Memory limit (default: "4g")
memory = "8g"

# CPU limit (default: "4")
cpus = "6"

# Process limit (default: "256")
pids_limit = "512"
```

### `[security]` Section

Security configuration:

```toml
[security]
# Seccomp profile path or "none"
seccomp_profile = "./seccomp/seccomp-restrictive.json"

# Allowed DNS servers (default: "8.8.8.8 8.8.4.4 1.1.1.1 1.0.0.1")
dns_servers = "8.8.8.8 1.1.1.1"

# Audit logging: "none" (default), "basic", "verbose"
audit_log = "basic"

# Custom system prompt injection message
inject_message = "You are in a restricted environment"
```

## Example Configurations

### Example 1: Python Development

Perfect for Python projects that need GitHub and PyPI access:

```toml
[default]
allow_domains = "github.com api.github.com pypi.org files.pythonhosted.org"
volumes = [
    "~/.ssh:/home/agent/.ssh:ro",
    "~/.gitconfig:/home/agent/.gitconfig:ro"
]
auth_home = "~/.config/rustyolo"

[resources]
memory = "4g"
cpus = "4"

[security]
audit_log = "basic"
```

### Example 2: JavaScript/Node.js Development

For JavaScript/TypeScript projects:

```toml
[default]
allow_domains = "github.com api.github.com npmjs.org registry.npmjs.org"
volumes = [
    "~/.ssh:/home/agent/.ssh:ro",
    "~/.gitconfig:/home/agent/.gitconfig:ro",
    "~/.npmrc:/home/agent/.npmrc:ro"
]

[resources]
memory = "6g"
cpus = "4"
```

### Example 3: Maximum Security

Minimal network access with strict resource limits:

```toml
[default]
allow_domains = "api.anthropic.com anthropic.com"

[resources]
memory = "2g"
cpus = "2"
pids_limit = "128"

[security]
seccomp_profile = "./seccomp/seccomp-restrictive.json"
audit_log = "verbose"
```

### Example 4: Claude Only (No External Network)

For local code analysis with Claude:

```toml
[default]
# No allow_domains = no network access except Anthropic API
volumes = ["~/.gitconfig:/home/agent/.gitconfig:ro"]

[resources]
memory = "4g"
cpus = "4"

[security]
audit_log = "basic"
```

## CLI Override Examples

CLI arguments always override config file settings:

```bash
# Config says: allow_domains = "github.com"
# This CLI arg adds pypi.org:
rustyolo --allow-domains "github.com pypi.org" claude

# Config says: memory = "4g"
# This CLI arg changes to 8g:
rustyolo --memory 8g claude

# Config provides volumes
# These additional volumes are MERGED (CLI + config):
rustyolo -v ~/.aws:/home/agent/.aws:ro claude
```

**Note**: For volumes and environment variables, CLI arguments **replace** config values entirely, not merge with them.

## Validation and Error Handling

RustyYOLO validates your configuration file:

### Unknown Fields

The config parser rejects unknown fields:

```toml
[default]
typo_field = "value"  # ❌ Error: unknown field
```

Error message:
```
Failed to parse config file: unknown field `typo_field`, expected one of ...
```

### Invalid Values

Invalid resource values are caught:

```toml
[resources]
memory = "not-a-number"  # ❌ Invalid
```

### Missing File

If `.rustyolo.toml` doesn't exist, rustyolo silently continues with defaults.

## Best Practices

### 1. **Start with the Example**

Copy `.rustyolo.toml.example` and customize:
```bash
cp .rustyolo.toml.example .rustyolo.toml
vim .rustyolo.toml
```

### 2. **Use Per-Project Configs**

Create different `.rustyolo.toml` files for different projects:

```bash
# Python project
cd ~/python-project
cat > .rustyolo.toml <<EOF
[default]
allow_domains = "github.com pypi.org"
volumes = ["~/.ssh:/home/agent/.ssh:ro"]
EOF

# Node.js project
cd ~/nodejs-project
cat > .rustyolo.toml <<EOF
[default]
allow_domains = "github.com npmjs.org"
volumes = ["~/.ssh:/home/agent/.ssh:ro", "~/.npmrc:/home/agent/.npmrc:ro"]
EOF
```

### 3. **Version Control**

**Don't commit** `.rustyolo.toml` (it's gitignored by default). Each developer should have their own configuration.

**Do commit** a `.rustyolo.toml.example` with recommended settings:

```bash
# Create an example for your team
cp .rustyolo.toml .rustyolo.toml.example
git add .rustyolo.toml.example
git commit -m "docs: add recommended rustyolo configuration"
```

### 4. **Security Considerations**

- **Minimize domains**: Only whitelist domains you actually need
- **Read-only volumes**: Mount sensitive files as `:ro` (read-only)
- **Resource limits**: Don't disable resource limits unless necessary
- **Audit logging**: Use `audit_log = "basic"` for debugging network issues

### 5. **Testing Your Config**

Test your configuration with `--dry-run`:

```bash
rustyolo --dry-run claude
```

This shows the exact Docker command that will be executed, including all merged settings.

## Troubleshooting

### "Failed to parse config file"

**Problem**: TOML syntax error in `.rustyolo.toml`

**Solution**: Check the error message for the line number and field name. Common issues:
- Missing quotes around strings
- Incorrect array syntax
- Typos in field names

### "Loaded configuration from .rustyolo.toml" but settings not applied

**Problem**: CLI arguments are overriding config settings

**Solution**: This is intentional! CLI args always win. To use config values:
- Don't provide that CLI arg
- OR update the config file to match your desired settings

### Config file not being loaded

**Problem**: `.rustyolo.toml` exists but rustyolo doesn't load it

**Checklist**:
- File is in the current working directory (not subdirectory)
- File is named exactly `.rustyolo.toml` (check for typos)
- File has valid TOML syntax (test with `rustyolo --dry-run`)

## Reference

### All Configuration Options

Complete list of all available configuration options:

| Section | Field | Type | Default | Description |
|---------|-------|------|---------|-------------|
| `default` | `allow_domains` | String | none | Space-separated domains for network access |
| `default` | `volumes` | Array<String> | [] | Volume mounts (host:container[:options]) |
| `default` | `env` | Array<String> | [] | Environment variables (KEY=VALUE) |
| `default` | `auth_home` | Path | `~/.config/rustyolo` | Persistent auth directory |
| `default` | `image` | String | `ghcr.io/brooksomics/llm-rustyolo:latest` | Docker image |
| `default` | `agent` | String | `"claude"` | Agent to run |
| `resources` | `memory` | String | `"4g"` | Memory limit |
| `resources` | `cpus` | String | `"4"` | CPU limit |
| `resources` | `pids_limit` | String | `"256"` | Process limit |
| `security` | `seccomp_profile` | String | embedded default | Seccomp profile path |
| `security` | `dns_servers` | String | `"8.8.8.8 8.8.4.4 1.1.1.1 1.0.0.1"` | Allowed DNS servers |
| `security` | `audit_log` | String | `"none"` | Audit log level |
| `security` | `inject_message` | String | default message | System prompt injection |

### Related Documentation

- [Installation Guide](installation.md) - Setup instructions
- [Security Policy](../security/security-policy.md) - Security best practices
- [Seccomp Profiles](../security/seccomp.md) - Syscall filtering
- [Main README](../../README.md) - Project overview

---

**Questions?** Open an issue on [GitHub](https://github.com/brooksomics/llm-rustyolo/issues)
