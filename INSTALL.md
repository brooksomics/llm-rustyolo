# Installation Guide

This guide provides detailed installation instructions for llm-rustyolo using either Homebrew (recommended for macOS/Linux) or manual build.

## Installation Options

### Option 1: Homebrew Installation (Recommended for macOS/Linux)

This is the easiest method for macOS and Linux users.

#### Prerequisites

1. **Install Homebrew** (if not already installed):
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

2. **Install Docker**:
   ```bash
   # macOS
   brew install --cask docker

   # Linux (or download from https://www.docker.com/products/docker-desktop)
   curl -fsSL https://get.docker.com -o get-docker.sh
   sudo sh get-docker.sh
   ```

#### Install rustyolo

```bash
# Add the rustyolo tap
brew tap brooksomics/rustyolo

# Install rustyolo
brew install rustyolo

# Verify installation
rustyolo --version
```

#### Get the Docker Image

```bash
# Pull the pre-built image from GitHub Container Registry
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

**Optional: Build Locally**

If you need to customize the image:

```bash
git clone https://github.com/brooksomics/llm-rustyolo.git
cd llm-rustyolo
docker build -t ghcr.io/brooksomics/llm-rustyolo:latest .
```

#### Updating

```bash
# Update Docker image (shows reminder about CLI)
rustyolo update

# Update just the Docker image
rustyolo update --image

# Or pull manually
docker pull ghcr.io/brooksomics/llm-rustyolo:latest

# Update the CLI binary
brew upgrade rustyolo
```

**Note:** The `rustyolo update` command updates the Docker image and reminds you to run `brew upgrade rustyolo` for the CLI binary.

---

### Option 2: Manual Build

Use this method if you want to customize the code or if Homebrew is not available.

#### Prerequisites

#### 1. Install Rust

If you don't have Rust installed, install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts, then reload your shell:

```bash
source ~/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

#### 2. Install Docker

#### macOS

Install Docker Desktop:
```bash
brew install --cask docker
```

Or download from: https://www.docker.com/products/docker-desktop

#### Linux

```bash
# Ubuntu/Debian
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add your user to the docker group
sudo usermod -aG docker $USER
newgrp docker
```

Verify installation:

```bash
docker --version
docker ps
```

#### Building llm-rustyolo

##### 1. Clone the Repository

```bash
git clone https://github.com/brooksomics/llm-rustyolo.git
cd llm-rustyolo
```

##### 2. Build the Rust CLI

```bash
# Build in release mode (optimized)
cargo build --release

# The binary will be at: target/release/rustyolo
```

##### 3. Install the Binary (Optional but Recommended)

```bash
# macOS/Linux
sudo cp target/release/rustyolo /usr/local/bin/

# Or copy to your local bin (no sudo needed)
mkdir -p ~/bin
cp target/release/rustyolo ~/bin/
# Make sure ~/bin is in your PATH
```

##### 4. Get the Docker Image

```bash
# Pull the pre-built image
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

**Optional: Build Locally**

If you need to customize the image:

```bash
docker build -t ghcr.io/brooksomics/llm-rustyolo:latest .
```

This will take a few minutes as it downloads the Node.js base image and installs Claude Code.

## Verification

### Test the Rust CLI

```bash
rustyolo --help
```

You should see the help message.

### Test the Docker Image

```bash
docker images | grep llm-rustyolo
```

You should see the `ghcr.io/brooksomics/llm-rustyolo` image.

### Run a Basic Test

```bash
# Create a test directory
mkdir -p /tmp/rustyolo-test
cd /tmp/rustyolo-test

# Run Claude help (no network needed)
rustyolo claude --help
```

If you see Claude's help message, everything is working!

## Common Issues

### "cargo: command not found"

You need to install Rust. See step 1 above.

After installing, make sure to reload your shell or run:
```bash
source ~/.cargo/env
```

### "docker: command not found"

You need to install Docker. See step 2 above.

### "permission denied" when running docker

On Linux, add your user to the docker group:
```bash
sudo usermod -aG docker $USER
newgrp docker
```

### Docker build fails with "npm install" errors

Try pulling the pre-built image instead:
```bash
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
```

If building locally, try:
```bash
docker build --no-cache -t ghcr.io/brooksomics/llm-rustyolo:latest .
```

## Next Steps

See [CLAUDE.md](./CLAUDE.md) for complete usage documentation.

Quick start:
```bash
cd ~/my-project

rustyolo \
  --allow-domains "github.com api.github.com" \
  -v ~/.ssh:/home/agent/.ssh:ro \
  --auth-home ~/.config/rustyolo \
  claude
```
