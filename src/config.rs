use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration file structure for .rustyolo.toml
///
/// This allows users to specify default settings at the project level,
/// avoiding the need to type long command-line arguments repeatedly.
///
/// CLI arguments always take precedence over config file settings.
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Default configuration section
    #[serde(default)]
    pub default: DefaultConfig,

    /// Resource limits configuration
    #[serde(default)]
    pub resources: ResourcesConfig,

    /// Security configuration
    #[serde(default)]
    pub security: SecurityConfig,
}

/// Default runtime configuration
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DefaultConfig {
    /// Space-separated list of domains to allow outbound traffic to
    pub allow_domains: Option<String>,

    /// Additional volumes to mount (array of strings)
    pub volumes: Option<Vec<String>>,

    /// Environment variables to pass (array of strings in KEY=VALUE format)
    pub env: Option<Vec<String>>,

    /// Persistent auth directory path
    pub auth_home: Option<PathBuf>,

    /// Docker image to use
    pub image: Option<String>,

    /// Agent to run (e.g., "claude")
    pub agent: Option<String>,
}

/// Resource limits configuration
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ResourcesConfig {
    /// Memory limit (e.g., "4g", "512m")
    pub memory: Option<String>,

    /// CPU limit (e.g., "4", "0.5")
    pub cpus: Option<String>,

    /// Maximum number of processes
    pub pids_limit: Option<String>,
}

/// Security configuration
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SecurityConfig {
    /// Path to custom seccomp profile, or "none" to disable
    pub seccomp_profile: Option<String>,

    /// Space-separated list of DNS servers to allow
    pub dns_servers: Option<String>,

    /// Audit logging level: "none", "basic", "verbose"
    pub audit_log: Option<String>,

    /// Custom message to inject into agent's system prompt
    pub inject_message: Option<String>,
}

impl Config {
    /// Load configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// * `Ok(Config)` - Successfully parsed configuration
    /// * `Err(String)` - Error message if parsing failed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let config = Config::load(".rustyolo.toml")?;
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read config file: {e}"))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {e}"))
    }

    /// Try to load configuration from the current directory
    ///
    /// Looks for `.rustyolo.toml` in the current directory.
    /// Returns `Ok(None)` if the file doesn't exist.
    /// Returns `Err` if the file exists but cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// match Config::try_load_from_current_dir() {
    ///     Ok(Some(config)) => println!("Loaded config"),
    ///     Ok(None) => println!("No config file found"),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn try_load_from_current_dir() -> Result<Option<Self>, String> {
        let config_path = PathBuf::from(".rustyolo.toml");

        if !config_path.exists() {
            return Ok(None);
        }

        Self::load(&config_path).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_config() {
        let toml_str = r#"
[default]
allow_domains = "github.com pypi.org"
volumes = ["~/.ssh:/home/agent/.ssh:ro", "~/.gitconfig:/home/agent/.gitconfig:ro"]
env = ["MY_VAR=value", "ANOTHER=var"]
auth_home = "~/.config/rustyolo"
image = "my-custom-image:latest"
agent = "claude"

[resources]
memory = "8g"
cpus = "6"
pids_limit = "512"

[security]
seccomp_profile = "./seccomp/custom.json"
dns_servers = "8.8.8.8 1.1.1.1"
audit_log = "verbose"
inject_message = "You are in a restricted environment"
"#;

        let config: Config = toml::from_str(toml_str).unwrap();

        // Test default section
        assert_eq!(
            config.default.allow_domains,
            Some("github.com pypi.org".to_string())
        );
        assert_eq!(config.default.volumes.as_ref().unwrap().len(), 2);
        assert_eq!(config.default.env.as_ref().unwrap().len(), 2);
        assert_eq!(
            config.default.auth_home,
            Some(PathBuf::from("~/.config/rustyolo"))
        );
        assert_eq!(
            config.default.image,
            Some("my-custom-image:latest".to_string())
        );
        assert_eq!(config.default.agent, Some("claude".to_string()));

        // Test resources section
        assert_eq!(config.resources.memory, Some("8g".to_string()));
        assert_eq!(config.resources.cpus, Some("6".to_string()));
        assert_eq!(config.resources.pids_limit, Some("512".to_string()));

        // Test security section
        assert_eq!(
            config.security.seccomp_profile,
            Some("./seccomp/custom.json".to_string())
        );
        assert_eq!(
            config.security.dns_servers,
            Some("8.8.8.8 1.1.1.1".to_string())
        );
        assert_eq!(config.security.audit_log, Some("verbose".to_string()));
        assert_eq!(
            config.security.inject_message,
            Some("You are in a restricted environment".to_string())
        );
    }

    #[test]
    fn test_parse_minimal_config() {
        let toml_str = r#"
[default]
allow_domains = "github.com"
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.default.allow_domains,
            Some("github.com".to_string())
        );
        assert!(config.default.volumes.is_none());
        assert!(config.resources.memory.is_none());
    }

    #[test]
    fn test_parse_empty_config() {
        let toml_str = "";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.default.allow_domains.is_none());
    }

    #[test]
    fn test_reject_unknown_fields() {
        let toml_str = r#"
[default]
unknown_field = "value"
"#;

        let result: Result<Config, _> = toml::from_str(toml_str);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unknown field `unknown_field`"));
    }
}
