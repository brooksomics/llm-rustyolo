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

## Alternative: Google OAuth Login (Headless Workaround)

If you prefer to use Google OAuth instead of an API key, the Gemini CLI has a known limitation with headless environments ([Issue #1696](https://github.com/google-gemini/gemini-cli/issues/1696)). Here's a workaround using debug mode:

### Step 1: Run with Debug Flag

```bash
rustyolo gemini --debug
```

Select "1. Login with Google" and watch the debug output for an authentication URL like:

```
https://accounts.google.com/o/oauth2/v2/auth?client_id=...
```

### Step 2: Complete OAuth in Your Host Browser

1. Copy the full OAuth URL from the debug output
2. Open it in your **host machine's browser** (not in the container)
3. Complete the Google login and authorization
4. The browser will redirect to `http://localhost:PORT/callback?code=...`

### Step 3: OAuth Callback Handling

The challenge is that the OAuth callback expects a local server running in the container, but your browser is on the host.

**Option A: Port Forwarding (If you have SSH access)**
```bash
# On your host, forward the OAuth callback port (usually 8085 or similar)
ssh -L 8085:localhost:8085 user@container-host

# Then run rustyolo gemini with the forwarded port accessible
```

**Option B: Manual Token Exchange (Advanced)**
This requires intercepting the OAuth code and manually exchanging it for tokens. Not recommended unless you're familiar with OAuth flows.

### Why This Is Complex

The sandboxed Docker container:
- Cannot open a browser (headless environment)
- Cannot easily access the host's browser for OAuth callbacks
- Has strict network isolation that blocks most Google domains by default

The rustyolo CLI automatically whitelists these Google domains when using Gemini:
- `generativelanguage.googleapis.com` (API endpoint)
- `accounts.google.com` (OAuth login)
- `oauth2.googleapis.com` (Token exchange)
- `www.googleapis.com` (General Google APIs)

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
