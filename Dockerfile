# Start from the official Node.js image
FROM node:latest

# Install dependencies:
# - iptables: The Linux firewall (for network isolation)
# - dnsutils: Provides 'dig' (for resolving domains)
# - gosu: For safely dropping privileges
RUN apt-get update && \
    apt-get install -y iptables dnsutils gosu && \
    rm -rf /var/lib/apt/lists/*

# Install AI agents
# Claude Code is the primary agent
# Add other AI CLIs as they become available (e.g., aider, cursor-cli, etc.)
RUN npm install -g @anthropic-ai/claude-code

# Create the non-root user that the agent will run as.
# We create it with a placeholder UID/GID that will be changed at runtime.
# Use a high UID to avoid conflicts with existing users in the base image.
RUN useradd --uid 9001 --create-home --shell /bin/bash agent

# Copy the entrypoint script that sets up the firewall
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Set the working directory
WORKDIR /app

# Run the entrypoint script
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Default command (will be appended to the entrypoint)
CMD ["claude", "--dangerously-skip-permissions"]
