# Gemini CLI Authentication Guide

This guide explains how to authenticate with Google Gemini CLI inside the rustyolo sandbox.

## Background

The Gemini CLI supports three authentication methods:
1. **Login with Google** (OAuth) - Browser-based
2. **Use Gemini API Key** - Recommended for sandboxed environments
3. **Vertex AI** - For Google Cloud Platform

Due to the sandboxed Docker environment, the standard Google OAuth flow (which requires opening a browser) needs special handling.

## Recommended: Use API Key

The simplest method for sandboxed environments:

```bash
# Start rustyolo with Gemini
rustyolo gemini

# Select: "2. Use Gemini API Key"
# Enter your API key when prompted
```

**Get an API key:** https://aistudio.google.com/app/apikey

The API key will be stored in `~/.config/rustyolo` (or your custom `--auth-home` directory) and persisted across sessions.

## Alternative: Google OAuth Login (Pre-Authentication Workflow)

If you prefer to use Google OAuth instead of an API key, you can authenticate once on your host machine and then mount the OAuth credentials into the rustyolo container.

### One-Time Setup

**Step 1: Install Gemini CLI on your host machine**

Choose one installation method:

```bash
# Option A: Homebrew (macOS/Linux)
brew install gemini-cli

# Option B: npm (any platform)
npm install -g @google/gemini-cli
```

**Step 2: Authenticate on your host**

Run Gemini CLI **outside** of rustyolo (where your browser works):

```bash
gemini
# Select: "1. Login with Google"
# Browser opens automatically
# Complete the OAuth flow
# Credentials are saved to ~/.gemini/oauth_creds.json
```

**Step 3: Mount the OAuth credentials with rustyolo**

```bash
rustyolo -v ~/.gemini:/home/agent/.gemini gemini
```

**Important:** Do NOT use `:ro` (read-only) on the mount - Gemini CLI needs write access to create temporary files for logging and chat state.

### Make it Easier with Config File

Add this to `.rustyolo.toml` in your project:

```toml
[default]
# Mount Gemini OAuth credentials (read-write required)
volumes = [
    "~/.gemini:/home/agent/.gemini"
]
```

Then just run:
```bash
rustyolo gemini  # OAuth credentials automatically available!
```

### How It Works

The rustyolo CLI automatically whitelists these Google domains when using Gemini:
- `generativelanguage.googleapis.com` (API endpoint)
- `accounts.google.com` (OAuth login)
- `oauth2.googleapis.com` (Token exchange)
- `www.googleapis.com` (General Google APIs)

The container uses your existing OAuth tokens from `~/.gemini/oauth_creds.json`, so no browser is needed inside the container.

## Persistent Authentication

Regardless of which method you use, authentication credentials are stored in:

**Default location:**
```
~/.config/rustyolo/
```

**Custom location:**
```bash
rustyolo --auth-home /path/to/auth gemini
```

The auth directory is mounted into the container at `/home/agent/.config/rustyolo`, so your credentials persist across sessions.

## Troubleshooting

### "Please restart Gemini CLI to continue"

This message appears when:
1. OAuth flow was initiated but not completed
2. The callback server is waiting for the OAuth redirect

**Solution:** Use the API key method instead, or follow the headless workaround above.

### "Network request failed"

This means a required domain isn't whitelisted. The rustyolo CLI should automatically whitelist all necessary Google domains, but if you see this error:

```bash
# Check the firewall logs
docker ps -a  # Get the container ID from your last run
docker logs <container-id> | grep AUDIT-BLOCK

# Add any missing domains manually
rustyolo --allow-domains "missing.domain.com" gemini
```

### API Key vs OAuth: Which Should I Use?

**Use API Key if:**
- ✅ You want simple, reliable authentication
- ✅ You're working in a sandboxed/headless environment
- ✅ You don't need to access Google Cloud Platform resources

**Use OAuth if:**
- ✅ You need access to Vertex AI or other GCP services
- ✅ You're comfortable with port forwarding or SSH tunneling
- ✅ You prefer using your Google account over managing API keys

## Related Issues

- [Gemini CLI Issue #1696](https://github.com/google-gemini/gemini-cli/issues/1696) - Authentication fails on headless environments
- [Gemini CLI Issue #4984](https://github.com/google-gemini/gemini-cli/issues/4984) - OAuth authentication fails due to IPv6 issues

## Future Improvements

The Gemini CLI team is considering adding:
- Device code flow (like GitHub CLI)
- `--no-browser` flag (like `gcloud auth login --no-browser`)
- Better headless authentication support

Once these features are available, they'll work seamlessly with rustyolo's sandboxed environment.
