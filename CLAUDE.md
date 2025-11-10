# llm-rustyolo

A secure, firewalled wrapper for running AI agents (like Claude Code) in "YOLO mode" with complete protection against the "lethal trifecta" of security risks.

## What is this?

This project provides a Rust-based CLI wrapper that runs AI coding agents inside a hardened Docker container with three layers of security:

1. **Filesystem Isolation**: The agent only sees your project directory and explicitly mounted volumes (like read-only `~/.ssh`). It cannot access your host filesystem.

2. **Privilege Isolation**: The agent runs as a non-root user inside the container, with file permissions matched to your host user.

3. **Network Isolation**: A dynamic iptables firewall is built at startup, blocking all outbound network traffic except for DNS and a whitelist of trusted domains you provide.

This allows you to safely run agents with permission-skipping flags (like `--dangerously-skip-permissions`) without worrying about data exfiltration, filesystem damage, or network abuse.

## Architecture

The project consists of two components:

1. **Rust CLI (`rustyolo`)**: A wrapper you run on your host machine. It parses your arguments (volumes, network rules, auth paths) and constructs a secure `docker run` command.

2. **Docker Image (`llm-rustyolo`)**: Contains the AI agents (Claude Code, etc.) and an entrypoint script that:
   - Builds the iptables firewall based on your trusted domains
   - Syncs the container user's UID/GID to match your host user
   - Drops privileges and runs the agent as a non-root user

## Inspiration

This project was inspired by:
- Simon Willison's blog post: ["Living dangerously with Claude"](https://simonwillison.net/2025/Oct/22/living-dangerously-with-claude/)
- [deva.sh](https://github.com/thevibeworks/deva) - for auth and volume mounting patterns
- [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) - for sandboxing concepts

The key innovation here is combining flexible auth/volume mounting with strict network firewalling, all written in Rust for better performance and type safety.

## Setup

### 1. Build the Rust CLI

```bash
# Clone the repo
git clone <your-repo-url>
cd llm-rustyolo

# Build the release binary
cargo build --release

# Copy to your PATH (optional but recommended)
cp target/release/rustyolo /usr/local/bin/
```

### 2. Build the Docker Image

```bash
docker build -t llm-rustyolo:latest .
```

This will:
- Start from the official Node.js image
- Install iptables, dnsutils, and gosu
- Install Claude Code (and other agents as they become available)
- Create a non-root `agent` user
- Copy the entrypoint script

## Updating

`rustyolo` includes built-in auto-update functionality to keep both the CLI binary and Docker image up-to-date.

### Automatic Update Checks

By default, `rustyolo` checks for updates on startup and displays a warning if a new version is available:

```bash
[RustyYOLO] ⚠️  New version 0.2.0 available! (current: 0.1.0)
[RustyYOLO]    Run 'rustyolo update' to upgrade.
```

To skip this check (e.g., for scripts or CI), use the `--skip-version-check` flag:

```bash
rustyolo --skip-version-check claude
```

### Manual Updates

#### Update Everything (Recommended)

Update both the binary and Docker image:

```bash
rustyolo update
```

This will:
1. Check GitHub releases for the latest binary version
2. Download and install the new binary
3. Pull the latest Docker image

#### Update Binary Only

```bash
rustyolo update --binary
```

#### Update Docker Image Only

```bash
rustyolo update --image
```

#### Skip Confirmation

Use the `--yes` flag to skip the update confirmation prompt:

```bash
rustyolo update --yes
```

### Manual Update Process (Without Auto-Update)

If you prefer to update manually or if auto-update fails:

**1. Update the Rust CLI Binary:**
```bash
cd llm-rustyolo
git pull
cargo build --release
cp target/release/rustyolo /usr/local/bin/
```

**2. Update the Docker Image:**
```bash
docker build -t llm-rustyolo:latest .
```

## Usage

### Basic Usage: Running Claude with Network Access

```bash
cd ~/my-project

rustyolo \
  --allow-domains "github.com api.github.com pypi.org files.pythonhosted.org" \
  -v ~/.ssh:/home/agent/.ssh:ro \
  -v ~/.gitconfig:/home/agent/.gitconfig:ro \
  --auth-home ~/.config/rustyolo \
  claude
```

This allows Claude to:
- Access GitHub (for git operations)
- Access PyPI (for pip installs)
- Use your SSH keys (read-only)
- Use your git config (read-only)
- Store auth tokens persistently in `~/.config/rustyolo`

### Running with No Network Access

```bash
rustyolo \
  -v ~/.ssh:/home/agent/.ssh:ro \
  --auth-home ~/.config/rustyolo \
  claude
```

This completely blocks all network traffic except DNS.

### Running Custom Commands

Pass any arguments after the agent name:

```bash
# Get help
rustyolo claude --help

# Run with a specific prompt
rustyolo claude -p "Add error handling to the API"

# Start an interactive session (without danger flag)
rustyolo claude
```

If you provide arguments, `rustyolo` won't add the default `--dangerously-skip-permissions` flag.

### Environment Variables

You can set trusted domains via environment variable:

```bash
export TRUSTED_DOMAINS="github.com api.github.com"
rustyolo claude
```

Pass custom environment variables to the container:

```bash
rustyolo -e MY_VAR=value -e ANOTHER=var claude
```

## CLI Reference

### Main Command

```
A secure, firewalled Docker wrapper for AI agents.

Usage: rustyolo [OPTIONS] [AGENT] [AGENT_ARGS]...
       rustyolo update [OPTIONS]

Subcommands:
  update    Update rustyolo components (binary and/or Docker image)

Arguments:
  [AGENT]
          The agent to run (e.g., 'claude', 'aider')
          [default: claude]

  [AGENT_ARGS]...
          Arguments to pass directly to the agent

Options:
  -v, --volume <VOLUMES>
          Additional volumes to mount
          Example: -v ~/.ssh:/home/agent/.ssh:ro

  -e, --env <ENVS>
          Environment variables to pass
          Example: -e MY_VAR=value

  --allow-domains <ALLOW_DOMAINS>
          Space-separated list of domains to allow outbound traffic to.
          All other traffic (except DNS) will be blocked.
          [env: TRUSTED_DOMAINS=]

  --auth-home <AUTH_HOME>
          Mount a persistent auth directory.
          Maps your local dir to '/home/agent/.config/rustyolo' in the container.
          Recommended: ~/.config/rustyolo

  --image <IMAGE>
          The Docker image to use
          [default: llm-rustyolo:latest]

  --skip-version-check
          Skip version check on startup

  -h, --help
          Print help

  -V, --version
          Print version
```

### Update Subcommand

```
Update rustyolo components (binary and/or Docker image)

Usage: rustyolo update [OPTIONS]

Options:
  --binary    Only update the binary
  --image     Only update the Docker image
  --yes       Skip version check confirmation
  -h, --help  Print help
```

## How It Works

### 1. Filesystem Isolation

The Rust CLI:
- Mounts your current working directory to `/app` in the container
- Mounts any volumes you specify with `-v`
- Mounts your auth directory (default: `~/.config/rustyolo`) persistently

The agent can only access these mounted paths. Your entire home directory and system files are invisible.

### 2. Privilege Isolation

The Rust CLI:
- Gets your host UID and GID using `id -u` and `id -g`
- Passes them to the container via environment variables

The entrypoint script:
- Updates the container's `agent` user to match your UID/GID
- Changes ownership of `/app` and persistent directories to match
- Uses `gosu` to drop root privileges and run the agent as the `agent` user

This ensures:
- Files created by the agent have your ownership
- The agent cannot escalate privileges
- The agent runs with minimal permissions

### 3. Network Isolation

The entrypoint script (running as root):
- Sets the default OUTPUT policy to DROP (blocks all outbound traffic)
- Allows loopback traffic
- Allows established/related connections
- Allows DNS (UDP/TCP port 53)
- Resolves each trusted domain to IP addresses using `dig`
- Creates iptables rules to allow traffic to those specific IPs
- Drops all other outbound traffic

This means:
- The agent can only contact domains you explicitly whitelist
- Even if the agent is compromised, it cannot exfiltrate data to arbitrary servers
- DNS resolution still works, but connections are blocked unless whitelisted

## Security Considerations

### What This Protects Against

- **Data Exfiltration**: The network firewall prevents the agent from sending your code/data to untrusted servers
- **Filesystem Access**: The agent cannot read sensitive files like `~/.aws/credentials` unless you explicitly mount them
- **Privilege Escalation**: The agent runs as a non-root user and cannot gain elevated privileges

### What This Does NOT Protect Against

- **Malicious Code in Your Project**: If the agent writes malicious code to your project, it will be owned by you and executed when you run it
- **Resource Exhaustion**: The agent could theoretically consume all CPU/memory (consider adding Docker resource limits)
- **Side Channels**: The agent could potentially encode data in DNS queries or timing attacks (though this is highly unlikely)

### Best Practices

1. **Review Changes**: Even with permissions skipped, always review what the agent changed before committing
2. **Whitelist Minimally**: Only add domains you absolutely need to the `--allow-domains` list
3. **Use Read-Only Mounts**: Mount sensitive files like SSH keys as read-only (`:ro`)
4. **Persistent Auth**: Use `--auth-home` to avoid re-authenticating every session
5. **Limit Scope**: Run the tool from the specific project directory, not your home directory

## Troubleshooting

### "Failed to execute docker command"

Make sure Docker is installed and running:
```bash
docker ps
```

### "Could not resolve domain"

The domain may not have A records, or DNS isn't working. Check:
```bash
dig +short github.com
```

### "Permission denied" errors inside container

The UID/GID mapping may have failed. Check the logs:
```bash
docker logs <container-id>
```

### Network requests are blocked

Add the required domains to `--allow-domains`. Use your browser's network inspector to see what domains the service uses.

## Extending

### Adding New Agents

To add support for other AI agents:

1. Update the Dockerfile to install the agent:
```dockerfile
RUN npm install -g <package-name>
```

2. Update `src/main.rs` to add default danger flags:
```rust
if args.agent == "your-agent" {
    docker_cmd.arg("--your-danger-flag");
}
```

3. Rebuild the Docker image:
```bash
docker build -t llm-rustyolo:latest .
```

### Custom Network Rules

You can modify `entrypoint.sh` to add more sophisticated firewall rules, like:
- Blocking specific ports
- Allowing IP ranges instead of just domains
- Rate limiting

### Resource Limits

Add Docker resource limits in `src/main.rs`:
```rust
docker_cmd.arg("--memory").arg("2g");
docker_cmd.arg("--cpus").arg("2");
```

## License

[Your License Here]

## Contributing

Contributions welcome! Please open an issue or PR.

### Publishing Releases

To enable auto-update functionality, new versions should be published as GitHub releases with precompiled binaries:

1. **Update version** in `Cargo.toml`
2. **Build release binaries** for supported platforms (Linux, macOS, Windows)
3. **Create a GitHub release** with tag format `vX.Y.Z` (e.g., `v0.2.0`)
4. **Attach binaries** to the release with naming convention: `rustyolo-{target}.tar.gz`

Example targets:
- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

The `self_update` crate will automatically detect and download the appropriate binary for the user's platform.

## Acknowledgments

- Simon Willison for the original inspiration
- The deva.sh team for auth patterns
- Anthropic for Claude Code
