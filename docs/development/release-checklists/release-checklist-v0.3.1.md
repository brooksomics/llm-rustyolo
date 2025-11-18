# Release Checklist for v0.3.1

Use this checklist to ensure all steps are completed for the v0.3.1 bug fix release.

## Pre-Release

- [x] Fix the Homebrew detection bug
- [x] Test the fix builds successfully
- [x] Update version in `Cargo.toml` to `0.3.1`
- [x] Update CHANGELOG.md with v0.3.1 entry
- [x] Create release notes files
- [x] Commit and push changes to PR branch
- [ ] Merge PR to main branch
- [ ] Pull latest main branch locally
- [ ] Verify version is `0.3.1` in Cargo.toml

## Build Binaries

### macOS Apple Silicon (aarch64-apple-darwin)
```bash
cargo build --release
tar -czf rustyolo-aarch64-apple-darwin.tar.gz -C target/release rustyolo
sha256sum rustyolo-aarch64-apple-darwin.tar.gz  # Save this hash
```

### macOS Intel (x86_64-apple-darwin) - Optional
```bash
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
tar -czf rustyolo-x86_64-apple-darwin.tar.gz -C target/x86_64-apple-darwin/release rustyolo
sha256sum rustyolo-x86_64-apple-darwin.tar.gz  # Save this hash
```

### Linux (x86_64-unknown-linux-gnu) - Optional
```bash
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
gh release create v0.3.1 \
  --title "v0.3.1 - Bug Fix: Homebrew Installation Detection" \
  --notes-file GITHUB_RELEASE_NOTES_v0.3.1.md \
  rustyolo-aarch64-apple-darwin.tar.gz \
  rustyolo-x86_64-apple-darwin.tar.gz \
  rustyolo-x86_64-unknown-linux-gnu.tar.gz
```

Or manually:
1. Go to https://github.com/brooksomics/llm-rustyolo/releases/new
2. Tag: `v0.3.1`
3. Title: `v0.3.1 - Bug Fix: Homebrew Installation Detection`
4. Copy contents from `GITHUB_RELEASE_NOTES_v0.3.1.md`
5. Upload binary tarballs
6. Publish release

- [ ] Create GitHub release with tag `v0.3.1`
- [ ] Upload all binary tarballs
- [ ] Verify release notes are correct
- [ ] Publish release

## Update Docker Image

**Note:** The Docker image doesn't need updating for this release since there are no Docker-related changes. However, you can still tag it with the version for consistency:

```bash
# Pull the existing latest image
docker pull ghcr.io/brooksomics/llm-rustyolo:latest

# Tag with version
docker tag ghcr.io/brooksomics/llm-rustyolo:latest ghcr.io/brooksomics/llm-rustyolo:0.3.1

# Push version tag (optional - latest is unchanged)
docker push ghcr.io/brooksomics/llm-rustyolo:0.3.1
```

- [ ] Optionally tag Docker image with `0.3.1`
- [ ] Optionally push `0.3.1` tag

## Update Homebrew Tap

If you have a Homebrew tap at `brooksomics/rustyolo`:

1. Update the formula with new version `0.3.1` and SHA256 hashes
2. Update download URLs to point to v0.3.1 release assets
3. Test the formula locally: `brew install --build-from-source rustyolo`
4. Push updated formula to tap repository

- [ ] Update Homebrew formula version to `0.3.1`
- [ ] Update SHA256 hashes for all platforms
- [ ] Update download URLs to v0.3.1
- [ ] Test formula installation
- [ ] Push to tap repository

## Post-Release Verification

```bash
# Test auto-update detection
rustyolo --version  # Should show 0.3.0 if using old binary
# Should see: "New version 0.3.1 available!"

# Test manual update
rustyolo update --binary --yes
rustyolo --version  # Should now show 0.3.1

# Test Homebrew update
brew upgrade rustyolo
rustyolo --version  # Should show 0.3.1

# Test the fix
rustyolo update  # Homebrew users should see correct message
```

- [ ] Verify version update detection works (0.3.0 -> 0.3.1)
- [ ] Test `rustyolo update` command for manual installations
- [ ] Test `brew upgrade rustyolo` for Homebrew users
- [ ] Verify Homebrew users see correct message with `rustyolo update`
- [ ] Verify the bug is fixed (no more "IoError: No such file or directory")

## Announce Release (Optional)

- [ ] Update README.md if needed
- [ ] Post about the bug fix release
- [ ] Notify users who reported the issue
- [ ] Update project homepage if applicable

## Cleanup

All release notes are already committed. Just ensure main branch has everything:

```bash
git checkout main
git pull origin main
git log --oneline -5  # Verify release commits are present
```

- [ ] Verify main branch has all release commits
- [ ] Delete local binary tarballs (optional)
- [ ] Archive PR branch (optional)

---

## Quick Command Reference

```bash
# After merging PR to main
git checkout main
git pull origin main

# Build and package (macOS Apple Silicon example)
cargo build --release
tar -czf rustyolo-aarch64-apple-darwin.tar.gz -C target/release rustyolo
shasum -a 256 rustyolo-aarch64-apple-darwin.tar.gz

# Create release
gh release create v0.3.1 \
  --title "v0.3.1 - Bug Fix: Homebrew Installation Detection" \
  --notes-file GITHUB_RELEASE_NOTES_v0.3.1.md \
  rustyolo-*.tar.gz

# Optional: Tag Docker image
docker pull ghcr.io/brooksomics/llm-rustyolo:latest
docker tag ghcr.io/brooksomics/llm-rustyolo:latest ghcr.io/brooksomics/llm-rustyolo:0.3.1
docker push ghcr.io/brooksomics/llm-rustyolo:0.3.1
```

---

## Important Notes

- This is a **patch release** (bug fix only, no new features)
- The Docker image doesn't need rebuilding (CLI-only change)
- The fix is critical for Homebrew users, so prioritize releasing quickly
- Update the Homebrew tap as soon as binaries are released
