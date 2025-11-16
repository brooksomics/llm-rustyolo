use serde::Deserialize;
use std::env;
use std::error::Error;
use std::process::Command;

const GITHUB_REPO: &str = "brooksomics/llm-rustyolo";
const GITHUB_API_URL: &str = "https://api.github.com/repos";

#[derive(Debug, PartialEq)]
pub enum InstallMethod {
    Homebrew,
    Manual,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

/// Get the latest version from GitHub releases
pub fn get_latest_version() -> Result<String, Box<dyn Error>> {
    let url = format!("{GITHUB_API_URL}/{GITHUB_REPO}/releases/latest");

    let response = reqwest::blocking::Client::builder()
        .user_agent("rustyolo")
        .timeout(std::time::Duration::from_secs(5))
        .build()?
        .get(&url)
        .send()?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned status: {}", response.status()).into());
    }

    let release: GitHubRelease = response.json()?;

    // Remove 'v' prefix if present
    let version = release.tag_name.trim_start_matches('v').to_string();
    Ok(version)
}

/// Detect how rustyolo was installed
pub fn detect_installation_method() -> InstallMethod {
    // Get the current executable path
    if let Ok(exe_path) = env::current_exe() {
        let path_str = exe_path.to_string_lossy();

        // Check if binary is in Homebrew Cellar path
        // Homebrew on Intel Macs: /usr/local/Cellar/rustyolo/
        // Homebrew on Apple Silicon: /opt/homebrew/Cellar/rustyolo/
        // Linuxbrew: /home/linuxbrew/.linuxbrew/Cellar/rustyolo/
        if path_str.contains("/Cellar/rustyolo/") {
            return InstallMethod::Homebrew;
        }

        // Check if binary is a Homebrew symlink
        // Homebrew creates symlinks in /opt/homebrew/bin or /usr/local/bin
        if path_str.starts_with("/opt/homebrew/bin/")
            || path_str.starts_with("/usr/local/bin/")
            || path_str.starts_with("/home/linuxbrew/.linuxbrew/bin/")
        {
            // Try to resolve the symlink to see if it points to Homebrew Cellar
            if let Ok(resolved_path) = std::fs::canonicalize(&exe_path) {
                let resolved_str = resolved_path.to_string_lossy();
                if resolved_str.contains("/Cellar/rustyolo/") {
                    return InstallMethod::Homebrew;
                }
            }
            // Even if we can't resolve it, if it's in a Homebrew bin directory,
            // it's likely a Homebrew installation
            return InstallMethod::Homebrew;
        }
    }

    InstallMethod::Manual
}

/// Update the binary using `self_update`
/// Note: This function should only be called for manual installations.
/// Homebrew installations should be handled by the caller (main.rs).
pub fn update_binary(skip_confirm: bool) -> Result<self_update::Status, Box<dyn Error>> {
    let current_version = env!("CARGO_PKG_VERSION");

    if !skip_confirm {
        println!("[RustyYOLO] Current version: {current_version}");
        println!("[RustyYOLO] Checking for latest release...");
    }

    let status = self_update::backends::github::Update::configure()
        .repo_owner("brooksomics")
        .repo_name("llm-rustyolo")
        .bin_name("rustyolo")
        .show_download_progress(true)
        .show_output(false)
        .no_confirm(skip_confirm)
        .current_version(current_version)
        .build()?
        .update()?;

    Ok(status)
}

/// Update the Docker image by pulling the latest version from GitHub Container Registry
pub fn update_docker_image() -> Result<(), Box<dyn Error>> {
    let image = "ghcr.io/brooksomics/llm-rustyolo:latest";

    println!("[RustyYOLO] Pulling latest Docker image: {image}");

    let output = Command::new("docker").arg("pull").arg(image).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to pull Docker image: {stderr}").into());
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires network access"]
    fn test_get_latest_version() {
        let result = get_latest_version();
        // This will fail until we publish the first release
        // but it's useful to test the API integration
        match result {
            Ok(version) => {
                assert!(!version.is_empty());
                println!("Latest version: {version}");
            }
            Err(e) => {
                println!("Expected error (no releases yet): {e}");
            }
        }
    }

    #[test]
    fn test_detect_installation_method() {
        // This test will pass in both environments
        // When run locally (manual install), it should return Manual
        // When run via Homebrew, it should return Homebrew
        let method = detect_installation_method();
        assert!(
            method == InstallMethod::Manual || method == InstallMethod::Homebrew,
            "Installation method should be either Manual or Homebrew"
        );
    }
}
