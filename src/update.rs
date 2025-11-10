use serde::Deserialize;
use std::error::Error;
use std::process::Command;

const GITHUB_REPO: &str = "brooksomics/llm-rustyolo";
const GITHUB_API_URL: &str = "https://api.github.com/repos";

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

/// Update the binary using `self_update`
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

/// Update the Docker image by pulling the latest version
pub fn update_docker_image() -> Result<(), Box<dyn Error>> {
    let image = "llm-rustyolo:latest";

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
}
