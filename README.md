# llm-rustyolo: A Secure, Firewalled Agent Runner

<p align="center">
  <img src="assets/mascot.png" alt="RustyYOLO Mascot" width="400">
</p>

This project provides a robust, secure wrapper for running AI agents like Claude Code in "YOLO mode" (`--dangerously-skip-permissions`) by solving the entire [lethal trifecta](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/):

🔒 **Filesystem Isolation**: The agent only sees your project directory and explicitly mounted volumes (like read-only `~/.ssh`). It cannot see your host filesystem.

👤 **Privilege Isolation**: The agent runs as a powerless, non-root `agent` user inside the container, with file permissions matched to your host user.

🔥 **Network Isolation**: A dynamic iptables firewall is built at startup, blocking all outbound network traffic except for DNS and a list of trusted domains you provide.

This tool is heavily inspired by [deva.sh](https://github.com/thevibeworks/deva) and Simon Willison's ["Living dangerously with Claude"](https://simonwillison.net/2025/Oct/22/living-dangerously-with-claude/).

## Architecture

This project has two parts:

1. **A Rust CLI (`rustyolo`)**: This is the wrapper you run on your host machine. It parses your arguments (volumes, network rules, auth paths) and programmatically constructs a secure `docker run` command.

2. **A Docker Image (`llm-rustyolo`)**: This image contains the agents (Claude Code, etc.) and an `entrypoint.sh` script. The script uses the arguments from the Rust CLI to build the firewall, fix file permissions, and then run the agent as a non-root user.

This approach combines the flexible auth and volume mounting from deva.sh with the strict network firewall we developed.

## Quick Setup

### Prerequisites
- **Homebrew** (for macOS/Linux users) - Install from https://brew.sh
- **Docker** (Docker Desktop on macOS, or docker.io on Linux)

### Installation

#### Option 1: Homebrew (Recommended for macOS/Linux)

```bash
# Install via Homebrew tap
brew tap brooksomics/rustyolo
brew install rustyolo

# Pull the Docker image
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

Or build locally if you need to customize:
```bash
git clone https://github.com/brooksomics/llm-rustyolo.git
cd llm-rustyolo
docker build -t ghcr.io/brooksomics/llm-rustyolo:latest .
```

#### Option 2: Manual Build (For customization or other platforms)

```bash
# 1. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Build the Rust CLI
cargo build --release
sudo cp target/release/rustyolo /usr/local/bin/

# 3. Pull the Docker image
docker pull ghcr.io/brooksomics/llm-rustyolo:latest

# Or build locally if you need to customize
docker build -t ghcr.io/brooksomics/llm-rustyolo:latest .
```

For detailed installation instructions, see [INSTALL.md](./INSTALL.md).

## Usage

Once set up, you can go to any project directory and run your agent.

### Example: Running Claude with Network Access

This is the most common use case. It allows the agent to access github.com (for git pull) and pypi.org (for pip install) but nothing else.

```bash
cd ~/my-new-project

rustyolo \
  --allow-domains "github.com api.github.com pypi.org files.pythonhosted.org" \
  -v ~/.ssh:/home/agent/.ssh:ro \
  -v ~/.gitconfig:/home/agent/.gitconfig:ro \
  --auth-home ~/.config/rustyolo \
  claude
```

### Example: Running with No Network

This runs Claude with zero internet access.

```bash
rustyolo \
  -v ~/.ssh:/home/agent/.ssh:ro \
  --auth-home ~/.config/rustyolo \
  claude
```

### Example: Running a Custom Command

You can pass any command and arguments after the agent name. `rustyolo` is smart enough to see you provided args and won't add its default "danger" flag.

```bash
rustyolo claude --help
```

## Keeping Up-to-Date

### Homebrew Installation

If you installed via Homebrew, you have multiple update options:

```bash
# Update Docker image only (shows reminder about CLI)
rustyolo update

# Update just the Docker image
rustyolo update --image

# Update the CLI binary (must use Homebrew)
brew upgrade rustyolo
```

**Note:** The `rustyolo update` command only updates the Docker image for Homebrew installations, as Homebrew manages the CLI binary separately. You'll see a reminder to run `brew upgrade rustyolo` for the CLI.

### Manual Installation

If you built from source, use the built-in update commands:

```bash
# Update the binary
rustyolo update --binary

# Update the Docker image
rustyolo update --image

# Update both
rustyolo update
```

The tool automatically checks for updates on startup and notifies you when a new version is available.

## All CLI Options

```
A secure, firewalled Docker wrapper for AI agents.

Usage: rustyolo [OPTIONS] [AGENT] [AGENT_ARGS]...
       rustyolo update [OPTIONS]

Subcommands:
  update    Update rustyolo components (binary and/or Docker image)

Arguments:
  [AGENT]
          The agent to run (e.g., 'claude')
          [default: claude]

  [AGENT_ARGS]...
          Arguments to pass directly to the agent (e.g., --help or -p "prompt")

Options:
  -v, --volume <VOLUMES>
          Additional volumes to mount (e.g., -v ~/.ssh:/home/agent/.ssh:ro)

  -e, --env <ENVS>
          Environment variables to pass (e.g., -e MY_VAR=value)

  --allow-domains <ALLOW_DOMAINS>
          Space-separated list of domains to allow outbound traffic to.
          All other traffic (except DNS) will be blocked.
          Example: --allow-domains "github.com pypi.org npmjs.com"
          Note: Anthropic domains are automatically added when using Claude.
          [env: TRUSTED_DOMAINS=]

  --auth-home <AUTH_HOME>
          Mount a persistent auth directory. Maps your local dir
          to '/home/agent/.config/rustyolo' in the container.
          Recommended: ~/.config/rustyolo

  --image <IMAGE>
          The Docker image to use
          [default: llm-rustyolo:latest]

  --skip-version-check
          Skip automatic version check on startup

  -h, --help
          Print help

  -V, --version
          Print version
```

## Documentation

- [INSTALL.md](./INSTALL.md) - Detailed installation instructions
- [CLAUDE.md](./CLAUDE.md) - Complete documentation on how it works, security considerations, and advanced usage
- [SECURITY.md](./SECURITY.md) - Secret scanning and security protection setup

## Security

This repository implements multiple layers of secret detection to prevent accidentally committing sensitive information:

- **Pre-commit Hooks** - Gitleaks, detect-secrets, and more run before each commit
- **GitHub Actions** - Automated secret scanning on every push and PR
- **git-secrets** - Additional local protection with custom patterns

See [SECURITY.md](./SECURITY.md) for complete setup instructions and best practices.

## License

MIT License

## Contributing

Contributions welcome! Please open an issue or PR.
