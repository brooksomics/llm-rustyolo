# Release Checklist for v0.3.0

Use this checklist to ensure all steps are completed for the release.

## Pre-Release

- [ ] Merge PR #15 to main
- [ ] Pull latest main branch locally
- [ ] Update version in `Cargo.toml` to `0.3.0`
- [ ] Commit version bump: `git commit -m "chore: bump version to 0.3.0"`
- [ ] Push to main: `git push origin main`

## Build Binaries

### macOS Apple Silicon (aarch64-apple-darwin)
```bash
cargo build --release
tar -czf rustyolo-aarch64-apple-darwin.tar.gz -C target/release rustyolo
sha256sum rustyolo-aarch64-apple-darwin.tar.gz  # Save this hash
```

### macOS Intel (x86_64-apple-darwin)
```bash
# If you have access to Intel Mac or cross-compilation setup
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
tar -czf rustyolo-x86_64-apple-darwin.tar.gz -C target/x86_64-apple-darwin/release rustyolo
sha256sum rustyolo-x86_64-apple-darwin.tar.gz  # Save this hash
```

### Linux (x86_64-unknown-linux-gnu)
```bash
# If you have access to Linux or cross-compilation setup
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
tar -czf rustyolo-x86_64-unknown-linux-gnu.tar.gz -C target/x86_64-unknown-linux-gnu/release rustyolo
sha256sum rustyolo-x86_64-unknown-linux-gnu.tar.gz  # Save this hash
```

- [ ] Build macOS Apple Silicon binary
- [ ] Build macOS Intel binary (if possible)
- [ ] Build Linux binary (if possible)
- [ ] Save SHA256 hashes for all binaries

## Create GitHub Release

```bash
gh release create v0.3.0 \
  --title "v0.3.0 - Sandbox Awareness & Seccomp Integration" \
  --notes-file GITHUB_RELEASE_NOTES.md \
  rustyolo-aarch64-apple-darwin.tar.gz \
  rustyolo-x86_64-apple-darwin.tar.gz \
  rustyolo-x86_64-unknown-linux-gnu.tar.gz
```

Or manually:
1. Go to https://github.com/brooksomics/llm-rustyolo/releases/new
2. Tag: `v0.3.0`
3. Title: `v0.3.0 - Sandbox Awareness & Seccomp Integration`
4. Copy contents from `GITHUB_RELEASE_NOTES.md`
5. Upload binary tarballs
6. Publish release

- [ ] Create GitHub release
- [ ] Upload all binary tarballs
- [ ] Verify release notes are correct
- [ ] Publish release

## Update Docker Image

```bash
# Build the image
docker build -t ghcr.io/brooksomics/llm-rustyolo:latest .

# Tag with version
docker tag ghcr.io/brooksomics/llm-rustyolo:latest ghcr.io/brooksomics/llm-rustyolo:0.3.0

# Push both tags
docker push ghcr.io/brooksomics/llm-rustyolo:latest
docker push ghcr.io/brooksomics/llm-rustyolo:0.3.0
```

- [ ] Build Docker image
- [ ] Tag with version number
- [ ] Push `latest` tag
- [ ] Push `0.3.0` tag
- [ ] Verify image is accessible: `docker pull ghcr.io/brooksomics/llm-rustyolo:latest`

## Update Homebrew Tap (if applicable)

If you have a Homebrew tap at `brooksomics/rustyolo`:

1. Update the formula with new version and SHA256 hashes
2. Test the formula locally: `brew install --build-from-source rustyolo`
3. Push updated formula to tap repository

- [ ] Update Homebrew formula version to `0.3.0`
- [ ] Update SHA256 hashes for all platforms
- [ ] Update download URLs
- [ ] Test formula installation
- [ ] Push to tap repository

## Post-Release Verification

```bash
# Test auto-update detection
rustyolo --version  # Should show 0.2.0 if using old binary
# Should see: "New version 0.3.0 available!"

# Test manual update
rustyolo update --binary --yes

# Test Docker pull
docker pull ghcr.io/brooksomics/llm-rustyolo:latest

# Test new features
rustyolo --help  # Verify --inject-message and --seccomp-profile are listed
rustyolo claude  # Should show default sandbox message
```

- [ ] Verify version update detection works
- [ ] Test `rustyolo update` command
- [ ] Test Docker image pull
- [ ] Test `--inject-message` flag
- [ ] Test `--seccomp-profile` flag
- [ ] Verify default seccomp profile works

## Announce Release (Optional)

- [ ] Update README.md if needed
- [ ] Tweet/post about the release
- [ ] Update project homepage if applicable
- [ ] Notify users in relevant communities

## Cleanup

- [ ] Archive release notes: `git add RELEASE_NOTES_v0.3.0.md CHANGELOG.md GITHUB_RELEASE_NOTES.md`
- [ ] Commit: `git commit -m "docs: add release notes for v0.3.0"`
- [ ] Push: `git push origin main`
- [ ] Delete local binary tarballs (optional)

---

## Quick Command Reference

```bash
# Version bump
sed -i '' 's/version = "0.2.0"/version = "0.3.0"/' Cargo.toml
git add Cargo.toml
git commit -m "chore: bump version to 0.3.0"
git push origin main

# Build and package
cargo build --release
tar -czf rustyolo-aarch64-apple-darwin.tar.gz -C target/release rustyolo

# Create release
gh release create v0.3.0 \
  --title "v0.3.0 - Sandbox Awareness & Seccomp Integration" \
  --notes-file GITHUB_RELEASE_NOTES.md \
  rustyolo-*.tar.gz

# Docker
docker build -t ghcr.io/brooksomics/llm-rustyolo:latest .
docker tag ghcr.io/brooksomics/llm-rustyolo:latest ghcr.io/brooksomics/llm-rustyolo:0.3.0
docker push ghcr.io/brooksomics/llm-rustyolo:latest
docker push ghcr.io/brooksomics/llm-rustyolo:0.3.0
```
