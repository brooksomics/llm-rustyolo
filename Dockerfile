# Start from Debian slim - no Node.js needed with native installer
FROM debian:bookworm-slim

# Install dependencies:
# - iptables: The Linux firewall (for network isolation)
# - dnsutils: Provides 'dig' (for resolving domains)
# - gosu: For safely dropping privileges
# - curl, ca-certificates: For downloading the native installer
# - git: Required by Claude Code for marketplace and git operations
RUN apt-get update && \
    apt-get install -y iptables dnsutils gosu curl ca-certificates git && \
    rm -rf /var/lib/apt/lists/*

# Install Claude Code via native installer
# The installer creates a symlink at ~/.local/bin/claude pointing to the actual
# binary in ~/.local/share/claude/versions/<version>. We copy the actual binary
# to /usr/local/bin for system-wide access.
RUN curl -fsSL https://claude.ai/install.sh | bash && \
    cp -L /root/.local/bin/claude /usr/local/bin/claude && \
    chmod +x /usr/local/bin/claude && \
    rm -rf /root/.local && \
    claude --version

# Create the non-root user that the agent will run as.
# We create it with a placeholder UID/GID that will be changed at runtime.
# Use a high UID to avoid conflicts with existing users in the base image.
# Create symlink at expected native install location to suppress startup warning.
# Note: We intentionally do NOT add ~/.local/bin to PATH to avoid PATH hijacking risks.
RUN useradd --uid 9001 --create-home --shell /bin/bash agent && \
    mkdir -p /home/agent/.local/bin && \
    ln -s /usr/local/bin/claude /home/agent/.local/bin/claude && \
    chown -R agent:agent /home/agent/.local

# Copy the entrypoint script that sets up the firewall
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Set the working directory
WORKDIR /app

# Run the entrypoint script
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Default command (will be appended to the entrypoint)
CMD ["claude", "--dangerously-skip-permissions"]
