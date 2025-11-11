# Homebrew Tap Setup Instructions

This directory contains the Homebrew formula template for `rustyolo`. Follow these instructions to set up your own Homebrew tap.

## Step 1: Create the Tap Repository

Create a new GitHub repository named `homebrew-rustyolo` under your GitHub organization/user (e.g., `brooksomics/homebrew-rustyolo`).

Homebrew expects tap repositories to follow the naming convention: `homebrew-<tap-name>`.

## Step 2: Initialize the Repository

```bash
# Clone the new repository
git clone https://github.com/brooksomics/homebrew-rustyolo.git
cd homebrew-rustyolo

# Create the Formula directory
mkdir -p Formula

# Copy the formula template
cp /path/to/llm-rustyolo/homebrew/rustyolo.rb Formula/rustyolo.rb

# Create a README
cat > README.md <<EOF
# RustyYOLO Homebrew Tap

Homebrew tap for [rustyolo](https://github.com/brooksomics/llm-rustyolo), a secure, firewalled wrapper for running AI agents in YOLO mode.

## Installation

\`\`\`bash
brew tap brooksomics/rustyolo
brew install rustyolo
\`\`\`

Or install directly:

\`\`\`bash
brew install brooksomics/rustyolo/rustyolo
\`\`\`

## Updating

\`\`\`bash
brew upgrade rustyolo
\`\`\`

## Usage

See the [main repository](https://github.com/brooksomics/llm-rustyolo) for usage instructions.
EOF

# Commit and push
git add .
git commit -m "Initial Homebrew formula for rustyolo"
git push origin main
```

## Step 3: Calculate SHA256 Checksums

After creating a new GitHub release with binary assets, you need to calculate the SHA256 checksums:

```bash
# Download the release assets
wget https://github.com/brooksomics/llm-rustyolo/releases/download/v0.1.1/rustyolo-x86_64-apple-darwin.tar.gz
wget https://github.com/brooksomics/llm-rustyolo/releases/download/v0.1.1/rustyolo-aarch64-apple-darwin.tar.gz
wget https://github.com/brooksomics/llm-rustyolo/releases/download/v0.1.1/rustyolo-x86_64-unknown-linux-gnu.tar.gz

# Calculate checksums
shasum -a 256 rustyolo-x86_64-apple-darwin.tar.gz
shasum -a 256 rustyolo-aarch64-apple-darwin.tar.gz
shasum -a 256 rustyolo-x86_64-unknown-linux-gnu.tar.gz
```

Update the `sha256` fields in `Formula/rustyolo.rb` with these values.

## Step 4: Test the Formula Locally

Before pushing updates, test the formula:

```bash
# Install from local formula file
brew install --build-from-source ./Formula/rustyolo.rb

# Or test with audit
brew audit --strict ./Formula/rustyolo.rb

# Test installation
rustyolo --version
```

## Step 5: Publish Updates

When releasing a new version:

1. **Create a new GitHub release** in `llm-rustyolo` with the new version tag (e.g., `v0.2.0`)
2. **Wait for GitHub Actions** to build and upload the release binaries
3. **Update the formula** in `homebrew-rustyolo`:
   ```bash
   cd homebrew-rustyolo

   # Update version
   sed -i '' 's/version "0.1.1"/version "0.2.0"/' Formula/rustyolo.rb

   # Update URLs
   sed -i '' 's/v0.1.1/v0.2.0/g' Formula/rustyolo.rb

   # Calculate new checksums (as shown in Step 3)
   # Update sha256 fields manually

   # Commit and push
   git add Formula/rustyolo.rb
   git commit -m "Update rustyolo to v0.2.0"
   git push origin main
   ```

4. **Users can now upgrade**:
   ```bash
   brew update
   brew upgrade rustyolo
   ```

## Automating Formula Updates (Optional)

You can automate formula updates using GitHub Actions in the `homebrew-rustyolo` repository. Create `.github/workflows/update-formula.yml`:

```yaml
name: Update Formula on Release

on:
  repository_dispatch:
    types: [new-release]

jobs:
  update-formula:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4

      - name: Update formula
        env:
          VERSION: ${{ github.event.client_payload.version }}
          INTEL_SHA: ${{ github.event.client_payload.intel_sha }}
          ARM_SHA: ${{ github.event.client_payload.arm_sha }}
          LINUX_SHA: ${{ github.event.client_payload.linux_sha }}
        run: |
          # Update version
          sed -i '' "s/version \"[^\"]*\"/version \"$VERSION\"/" Formula/rustyolo.rb

          # Update URLs
          sed -i '' "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$VERSION/g" Formula/rustyolo.rb

          # Update SHA256s
          sed -i '' "0,/sha256 \"[^\"]*\"/s//sha256 \"$INTEL_SHA\"/" Formula/rustyolo.rb
          sed -i '' "0,/sha256 \"[^\"]*\"/s//sha256 \"$ARM_SHA\"/" Formula/rustyolo.rb
          sed -i '' "0,/sha256 \"[^\"]*\"/s//sha256 \"$LINUX_SHA\"/" Formula/rustyolo.rb

      - name: Commit changes
        run: |
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add Formula/rustyolo.rb
          git commit -m "Update rustyolo to v${{ github.event.client_payload.version }}"
          git push
```

Then, trigger this workflow from your main repository's release workflow by adding a step:

```yaml
- name: Trigger Homebrew formula update
  uses: peter-evans/repository-dispatch@v3
  with:
    token: ${{ secrets.TAP_REPO_TOKEN }}
    repository: brooksomics/homebrew-rustyolo
    event-type: new-release
    client-payload: |
      {
        "version": "${{ github.event.release.tag_name }}",
        "intel_sha": "${{ steps.calculate-shas.outputs.intel }}",
        "arm_sha": "${{ steps.calculate-shas.outputs.arm }}",
        "linux_sha": "${{ steps.calculate-shas.outputs.linux }}"
      }
```

## Troubleshooting

### Formula fails to install

- Verify the download URLs are correct
- Check that SHA256 checksums match the actual files
- Test with `brew install --debug --verbose`

### Binary not found

- Ensure the tarball contains the `rustyolo` binary in the root
- Check extraction with: `tar -tzf rustyolo-*.tar.gz`

### Docker image not found

- Users need to either build the image locally or pull from a registry
- Update the `caveats` section with correct instructions

## Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Tap Documentation](https://docs.brew.sh/Taps)
- [Creating a Homebrew Tap](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
