#!/bin/bash
set -euo pipefail

# --- 1. CONFIGURE FIREWALL (as root) ---
echo "[RustyYOLO Firewall] Setting up network restrictions..."
iptables -P OUTPUT DROP
iptables -A OUTPUT -o lo -j ACCEPT
iptables -A OUTPUT -m state --state RELATED,ESTABLISHED -j ACCEPT

# Configure DNS restrictions (defense against DNS exfiltration)
DNS_SERVERS=${DNS_SERVERS:-"8.8.8.8 8.8.4.4 1.1.1.1 1.0.0.1"}
if [ "$DNS_SERVERS" = "any" ]; then
  echo "[RustyYOLO Firewall] WARNING: DNS to any server allowed (exfiltration risk!)"
  iptables -A OUTPUT -p udp --dport 53 -j ACCEPT
  iptables -A OUTPUT -p tcp --dport 53 -j ACCEPT
else
  echo "[RustyYOLO Firewall] Restricting DNS to allowed servers: $DNS_SERVERS"
  for dns_server in $DNS_SERVERS; do
    # Validate IP format to prevent command injection
    if ! echo "$dns_server" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$'; then
      echo "[RustyYOLO Firewall] ERROR: Invalid DNS server IP: $dns_server (skipping)"
      continue
    fi
    echo "[RustyYOLO Firewall] ALLOWING DNS to: $dns_server"
    iptables -A OUTPUT -p udp -d "$dns_server" --dport 53 -j ACCEPT
    iptables -A OUTPUT -p tcp -d "$dns_server" --dport 53 -j ACCEPT
  done
fi

# Read from TRUSTED_DOMAINS env var passed by the Rust wrapper
TRUSTED_DOMAINS=${TRUSTED_DOMAINS:-"github.com api.github.com pypi.org files.pythonhosted.org"}
echo "[RustyYOLO Firewall] Resolving and allowing trusted domains: $TRUSTED_DOMAINS"
for domain in $TRUSTED_DOMAINS; do
  # Validate domain format to prevent command injection
  # Allow: letters, digits, dots, hyphens, underscores (valid domain characters)
  if ! echo "$domain" | grep -qE '^[a-zA-Z0-9._-]+$'; then
    echo "[RustyYOLO Firewall] ERROR: Invalid domain format: $domain (skipping)"
    continue
  fi
  ips=$(dig +short "$domain" | grep -E '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$' || true)
  if [ -n "$ips" ]; then
    for ip in $ips; do
      echo "[RustyYOLO Firewall] ALLOWING IP: $ip (for $domain)"
      iptables -A OUTPUT -d "$ip" -j ACCEPT
    done
  else
    echo "[RustyYOLO Firewall] WARNING: Could not resolve $domain"
  fi
done
echo "[RustyYOLO Firewall] Setup complete. All other outbound traffic is blocked."


# --- 2. FIX PERMISSIONS (as root) ---
# Get the UID/GID passed from the Rust wrapper
# Default to 9001 if not set (matches Dockerfile placeholder)
AGENT_UID=${AGENT_UID:-9001}
AGENT_GID=${AGENT_GID:-9001}

echo "[RustyYOLO Permissions] Syncing user 'agent' to UID=$AGENT_UID, GID=$AGENT_GID"
# This is the robust method from deva.sh
groupmod -o -g "$AGENT_GID" agent
usermod -o -u "$AGENT_UID" -g "$AGENT_GID" agent

# Fix permissions on mounted volumes
# Note: We exclude .git directories to avoid permission issues on macOS
echo "[RustyYOLO Permissions] Fixing ownership for project directory: /app"
find /app -not -path '*/.git/*' -not -name '.git' -exec chown "$AGENT_UID:$AGENT_GID" {} + 2>/dev/null || true

# Fix permissions on any persistent auth directories
PERSISTENT_DIRS=${PERSISTENT_DIRS:-"/home/agent/.config/rustyolo"}
if [ -n "$PERSISTENT_DIRS" ]; then
  echo "[RustyYOLO Permissions] Fixing ownership for persistent directories: $PERSISTENT_DIRS"
  for dir in $PERSISTENT_DIRS; do
    # Create the directory if it doesn't exist (as root) so we can mount to it
    mkdir -p "$dir"
    chown -R "$AGENT_UID:$AGENT_GID" "$dir"
  done
fi

# --- 3. RUN COMMAND (as non-root) ---
echo "[RustyYOLO Entrypoint] Dropping privileges and running command as 'agent' user: $@"
# Use gosu to drop privileges and execute the command
exec gosu agent "$@"
