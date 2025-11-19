use clap::{Args, Parser, Subcommand};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

mod config;
mod update;

// Embed the default seccomp profile at compile time
const DEFAULT_SECCOMP_PROFILE: &str = include_str!("../seccomp/seccomp-default.json");

// Default resource limits
const DEFAULT_MEMORY: &str = "4g";
const DEFAULT_CPUS: &str = "4";
const DEFAULT_PIDS_LIMIT: &str = "256";

// Default DNS servers (Google and Cloudflare public DNS)
const DEFAULT_DNS_SERVERS: &str = "8.8.8.8 8.8.4.4 1.1.1.1 1.0.0.1";

// Anthropic API domains (automatically added for Claude agent)
const ANTHROPIC_DOMAINS: &str = "api.anthropic.com anthropic.com";

// Default Docker image
const DEFAULT_IMAGE: &str = "ghcr.io/brooksomics/llm-rustyolo:latest";

// Default agent
const DEFAULT_AGENT: &str = "claude";

// Default audit log level
const DEFAULT_AUDIT_LOG: &str = "none";

/// A secure, firewalled Docker wrapper for AI agents.
///
/// This tool builds a 'docker run' command to enforce four layers of security:
/// 1. Filesystem Isolation (via read-only volume mounts)
/// 2. Privilege Isolation (by running as a non-root user)
/// 3. Network Isolation (by building an iptables firewall inside the container)
/// 4. Syscall Isolation (via seccomp to block dangerous system calls)
#[derive(Parser, Debug)]
#[command(name = "rustyolo", version, about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    run_args: Option<RunArgs>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Update rustyolo components (binary and/or Docker image)
    Update {
        /// Only update the binary
        #[arg(long)]
        binary: bool,

        /// Only update the Docker image
        #[arg(long)]
        image: bool,

        /// Skip version check confirmation
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Args, Debug)]
struct RunArgs {
    /// The agent to run (e.g., 'claude', 'codex', 'gemini-cli').
    #[arg(default_value = DEFAULT_AGENT)]
    agent: String,

    /// Additional volumes to mount (e.g., `-v ~/.ssh:/home/agent/.ssh:ro`)
    #[arg(short = 'v', long = "volume")]
    volumes: Vec<String>,

    /// Environment variables to pass (e.g., `-e MY_VAR=value`)
    #[arg(short = 'e', long = "env")]
    envs: Vec<String>,

    /// Space-separated list of domains to allow outbound traffic to.
    /// All other traffic (except DNS) will be blocked.
    /// Example: --allow-domains "github.com pypi.org npmjs.com"
    /// Note: Anthropic domains are automatically added when using Claude.
    #[arg(long, env = "TRUSTED_DOMAINS")]
    allow_domains: Option<String>,

    /// Mount a persistent auth directory. Maps your local dir
    /// to '/home/agent/.config/rustyolo' in the container.
    /// Recommended: ~/.config/rustyolo
    #[arg(long = "auth-home")]
    auth_home: Option<PathBuf>,

    /// The Docker image to use.
    #[arg(long, default_value = DEFAULT_IMAGE)]
    image: String,

    /// Arguments to pass directly to the agent (e.g., --help or -p "prompt").
    #[arg(last = true)]
    additional: Vec<String>,

    /// Skip version check on startup
    #[arg(long)]
    skip_version_check: bool,

    /// Custom message to inject into the agent's system prompt.
    /// Use 'none' to disable the default sandbox message.
    /// Default: Informs the agent about sandbox limitations.
    #[arg(long = "inject-message")]
    inject_message: Option<String>,

    /// Path to a custom seccomp profile, or 'none' to disable seccomp.
    /// If not specified, uses the embedded conservative default profile.
    /// Example: --seccomp-profile ./seccomp/seccomp-restrictive.json
    #[arg(long = "seccomp-profile")]
    seccomp_profile: Option<String>,

    /// Maximum memory the container can use (default: 4g).
    /// Use 'unlimited' to disable memory limits.
    /// Examples: 2g, 512m, 4096m
    #[arg(long, default_value = DEFAULT_MEMORY)]
    memory: String,

    /// Number of CPUs the container can use (default: 4).
    /// Use 'unlimited' to disable CPU limits.
    /// Examples: 2, 4, 0.5
    #[arg(long, default_value = DEFAULT_CPUS)]
    cpus: String,

    /// Maximum number of processes the container can spawn (default: 256).
    /// Use 'unlimited' to disable PID limits.
    #[arg(long, default_value = DEFAULT_PIDS_LIMIT)]
    pids_limit: String,

    /// Space-separated list of DNS servers to allow (default: Google and Cloudflare public DNS).
    /// Use 'any' to allow DNS to any server (NOT RECOMMENDED - enables exfiltration).
    /// Default: "8.8.8.8 8.8.4.4 1.1.1.1 1.0.0.1"
    /// Example: --dns-servers "8.8.8.8 1.1.1.1"
    #[arg(long, default_value = DEFAULT_DNS_SERVERS)]
    dns_servers: String,

    /// Enable audit logging of security events (default: none).
    /// - none: No audit logging (default)
    /// - basic: Log blocked network connections and syscalls
    /// - verbose: Also log allowed connections and resource usage
    ///
    ///   Logs are accessible via 'docker logs <container-id>'
    #[arg(long, default_value = DEFAULT_AUDIT_LOG)]
    audit_log: String,

    /// Print the Docker command without executing it (dry run mode)
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Update { binary, image, yes }) => {
            handle_update(binary, image, yes);
        }
        None => {
            // Run mode - check for updates first unless skipped
            let mut run_args = cli.run_args.unwrap_or_else(|| RunArgs {
                agent: DEFAULT_AGENT.to_string(),
                volumes: Vec::new(),
                envs: Vec::new(),
                allow_domains: None,
                auth_home: None,
                image: DEFAULT_IMAGE.to_string(),
                additional: Vec::new(),
                skip_version_check: false,
                inject_message: None,
                seccomp_profile: None,
                memory: DEFAULT_MEMORY.to_string(),
                cpus: DEFAULT_CPUS.to_string(),
                pids_limit: DEFAULT_PIDS_LIMIT.to_string(),
                dns_servers: DEFAULT_DNS_SERVERS.to_string(),
                audit_log: DEFAULT_AUDIT_LOG.to_string(),
                dry_run: false,
            });

            // Try to load configuration file from current directory
            if let Ok(Some(config)) = config::Config::try_load_from_current_dir() {
                println!("[RustyYOLO] Loaded configuration from .rustyolo.toml");
                merge_config_with_args(&mut run_args, config);
            }

            if !run_args.skip_version_check {
                check_for_updates();
            }
            run_agent(run_args);
        }
    }
}

fn handle_update(binary_only: bool, image_only: bool, yes: bool) {
    let install_method = update::detect_installation_method();
    let update_binary = binary_only || !image_only;
    let update_image = image_only || !binary_only;

    if update_binary {
        // For Homebrew installations, skip binary update gracefully
        if install_method == update::InstallMethod::Homebrew {
            if binary_only {
                // User explicitly requested --binary, show error
                eprintln!("[RustyYOLO] ❌ rustyolo was installed via Homebrew.");
                eprintln!("[RustyYOLO] To update the CLI binary, run:");
                eprintln!("[RustyYOLO]   brew upgrade rustyolo");
                eprintln!();
                eprintln!("[RustyYOLO] To update the Docker image, run:");
                eprintln!("[RustyYOLO]   rustyolo update --image");
                std::process::exit(1);
            } else {
                // User ran 'rustyolo update', skip binary with a reminder
                println!("[RustyYOLO] ℹ️  Skipping binary update (managed by Homebrew).");
                println!("[RustyYOLO] To update the CLI binary, run: brew upgrade rustyolo");
                println!();
            }
        } else {
            // Manual installation - proceed with binary update
            println!("[RustyYOLO] Updating binary...");
            match update::update_binary(yes) {
                Ok(status) => {
                    if status.updated() {
                        println!(
                            "[RustyYOLO] Binary updated successfully to version {}",
                            status.version()
                        );
                        println!("[RustyYOLO] Please restart rustyolo to use the new version.");
                    } else {
                        println!(
                            "[RustyYOLO] Binary is already up to date (version {}).",
                            status.version()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("[RustyYOLO] Failed to update binary: {e}");
                    std::process::exit(1);
                }
            }
        }
    }

    if update_image {
        println!("[RustyYOLO] Updating Docker image...");
        match update::update_docker_image() {
            Ok(()) => {
                println!("[RustyYOLO] Docker image updated successfully.");
            }
            Err(e) => {
                eprintln!("[RustyYOLO] Failed to update Docker image: {e}");
                std::process::exit(1);
            }
        }
    }
}

/// Merges configuration file settings with command-line arguments.
///
/// CLI arguments always take precedence over config file settings.
/// This function only applies config values if the CLI didn't provide them.
///
/// # Arguments
///
/// * `args` - Mutable reference to parsed CLI arguments
/// * `config` - Loaded configuration from .rustyolo.toml
fn merge_config_with_args(args: &mut RunArgs, config: config::Config) {
    // Merge default section
    if args.allow_domains.is_none() {
        args.allow_domains = config.default.allow_domains;
    }

    // Merge volumes - only if CLI didn't provide any
    if args.volumes.is_empty() {
        if let Some(config_volumes) = config.default.volumes {
            args.volumes = config_volumes;
        }
    }

    // Merge environment variables - only if CLI didn't provide any
    if args.envs.is_empty() {
        if let Some(config_envs) = config.default.env {
            args.envs = config_envs;
        }
    }

    if args.auth_home.is_none() {
        args.auth_home = config.default.auth_home;
    }

    // Only override image if it's still the default
    if args.image == DEFAULT_IMAGE {
        if let Some(config_image) = config.default.image {
            args.image = config_image;
        }
    }

    // Only override agent if it's still the default
    if args.agent == DEFAULT_AGENT {
        if let Some(config_agent) = config.default.agent {
            args.agent = config_agent;
        }
    }

    // Merge resources section - only if still using defaults
    if args.memory == DEFAULT_MEMORY {
        if let Some(config_memory) = config.resources.memory {
            args.memory = config_memory;
        }
    }

    if args.cpus == DEFAULT_CPUS {
        if let Some(config_cpus) = config.resources.cpus {
            args.cpus = config_cpus;
        }
    }

    if args.pids_limit == DEFAULT_PIDS_LIMIT {
        if let Some(config_pids_limit) = config.resources.pids_limit {
            args.pids_limit = config_pids_limit;
        }
    }

    // Merge security section
    if args.seccomp_profile.is_none() {
        args.seccomp_profile = config.security.seccomp_profile;
    }

    if args.dns_servers == DEFAULT_DNS_SERVERS {
        if let Some(config_dns_servers) = config.security.dns_servers {
            args.dns_servers = config_dns_servers;
        }
    }

    if args.audit_log == DEFAULT_AUDIT_LOG {
        if let Some(config_audit_log) = config.security.audit_log {
            args.audit_log = config_audit_log;
        }
    }

    if args.inject_message.is_none() {
        args.inject_message = config.security.inject_message;
    }
}

fn check_for_updates() {
    if let Ok(latest_version) = update::get_latest_version() {
        let current_version = env!("CARGO_PKG_VERSION");
        if latest_version != current_version {
            println!("[RustyYOLO] ⚠️  New version {latest_version} available! (current: {current_version})");
            println!("[RustyYOLO]    Run 'rustyolo update' to upgrade.");
            println!();
        }
    }
    // Silently ignore errors in version checking to not disrupt normal usage
}

/// Sets up seccomp (secure computing mode) syscall filtering for the Docker container.
///
/// Seccomp is a Linux kernel feature that restricts which system calls a process can make.
/// This provides defense-in-depth security by blocking dangerous syscalls like ptrace,
/// kernel module loading, and mount operations at the kernel level.
///
/// # Arguments
///
/// * `docker_cmd` - Mutable reference to the Docker command being constructed
/// * `seccomp_profile` - Optional seccomp profile specification:
///   - `None` - Use the embedded default conservative profile (recommended)
///   - `Some("none")` - Disable seccomp entirely (not recommended, for debugging only)
///   - `Some("/path/to/profile.json")` - Use a custom seccomp profile
///
/// # Returns
///
/// * `Some(PathBuf)` - Path to the temporary file containing the embedded profile (keeps it alive)
/// * `None` - If using a custom profile or seccomp is disabled
///
/// # Security
///
/// The default profile blocks ~40 dangerous syscalls including:
/// - ptrace (process debugging)
/// - mount/umount (filesystem manipulation)
/// - `init_module`/`delete_module` (kernel module loading)
/// - reboot (system reboot)
/// - bpf (eBPF program loading)
/// - keyctl (kernel keyring manipulation)
///
/// # Panics
///
/// Exits the process if a custom profile path is specified but the file doesn't exist.
///
/// # Examples
///
/// ```no_run
/// use std::process::Command;
/// let mut cmd = Command::new("docker");
///
/// // Use default profile
/// let _temp = setup_seccomp(&mut cmd, None);
///
/// // Disable seccomp (not recommended)
/// setup_seccomp(&mut cmd, Some("none"));
/// ```
fn setup_seccomp(docker_cmd: &mut Command, seccomp_profile: Option<&str>) -> Option<PathBuf> {
    match seccomp_profile {
        Some("none") => {
            // User explicitly disabled seccomp
            println!("[RustyYOLO] ⚠️  Seccomp disabled - syscall filtering is OFF");
            docker_cmd.arg("--security-opt").arg("seccomp=unconfined");
            None
        }
        Some(custom_path) => {
            // User provided a custom profile path
            let profile_path = PathBuf::from(custom_path);
            if !profile_path.exists() {
                eprintln!("[RustyYOLO] ❌ Seccomp profile not found: {custom_path}");
                std::process::exit(1);
            }
            println!("[RustyYOLO] Using custom seccomp profile: {custom_path}");
            docker_cmd
                .arg("--security-opt")
                .arg(format!("seccomp={}", profile_path.display()));
            None
        }
        None => {
            // Use the embedded default profile
            println!("[RustyYOLO] Using embedded default seccomp profile");

            // Write the embedded profile to a temporary file
            let temp_dir = env::temp_dir();
            let temp_profile_path = temp_dir.join("rustyolo-seccomp-default.json");

            fs::write(&temp_profile_path, DEFAULT_SECCOMP_PROFILE)
                .expect("Failed to write seccomp profile to temp file");

            docker_cmd
                .arg("--security-opt")
                .arg(format!("seccomp={}", temp_profile_path.display()));

            // Return the temp file so it doesn't get deleted until the function ends
            Some(temp_profile_path)
        }
    }
}

/// Validates user-supplied volumes for dangerous mounts that could enable container escape.
///
/// This function performs security checks on volume mount specifications to prevent:
/// - Docker socket mounting (complete container escape)
/// - Mounting critical system directories (/proc, /sys, /dev, /boot, /etc)
///
/// # Arguments
///
/// * `volumes` - Slice of volume mount specifications (e.g., "/host/path:/container/path:ro")
///
/// # Returns
///
/// * `Some(String)` - Error message describing the security risk if a dangerous volume is detected
/// * `None` - All volumes are safe to mount
///
/// # Security
///
/// This is a critical security function that prevents privilege escalation and container escape.
/// It blocks mounts that would allow an agent to:
/// - Spawn new containers via Docker socket
/// - Inspect or manipulate host processes via /proc
/// - Access raw devices via /dev
/// - Modify system configuration via /etc, /sys
/// - Access boot files via /boot
///
/// # Examples
///
/// ```no_run
/// // Safe volumes pass validation
/// let safe = vec!["~/.ssh:/home/agent/.ssh:ro".to_string()];
/// assert!(validate_volumes(&safe).is_none());
///
/// // Dangerous volumes are rejected
/// let dangerous = vec!["/var/run/docker.sock:/var/run/docker.sock".to_string()];
/// assert!(validate_volumes(&dangerous).is_some());
/// ```
fn validate_volumes(volumes: &[String]) -> Option<String> {
    for volume in volumes {
        let vol_lower = volume.to_lowercase();

        // Check for Docker socket - enables complete container escape
        if vol_lower.contains("docker.sock") {
            return Some(format!(
                "Mounting the Docker socket is forbidden (security risk: container escape).\n\
                 Attempted mount: {volume}\n\
                 This would allow the agent to spawn new containers and bypass all security restrictions."
            ));
        }

        // Check for other dangerous system mounts
        let dangerous_paths = [
            ("/proc", "process information"),
            ("/sys", "system configuration"),
            ("/dev", "devices"),
            ("/boot", "boot files"),
            ("/etc", "system configuration"),
        ];

        for (path, description) in &dangerous_paths {
            // Match exact mount of these system directories (not subdirectories in user projects)
            // e.g., block "-v /proc:/proc" but allow "-v /home/user/myproc:/myproc"
            if vol_lower.starts_with(&format!("{path}:")) {
                return Some(format!(
                    "Mounting {path} is forbidden (security risk: {description}).\n\
                     Attempted mount: {volume}\n\
                     If you need specific files, mount them individually rather than the entire directory."
                ));
            }
        }
    }
    None
}

/// Applies resource limits to the Docker command to prevent `DoS` attacks and resource exhaustion.
///
/// This function configures Docker's resource constraints to prevent a compromised agent from:
/// - Consuming all available memory (memory bombs)
/// - Spawning infinite processes (fork bombs)
/// - Monopolizing CPU resources (cryptomining, compute-intensive attacks)
///
/// # Arguments
///
/// * `docker_cmd` - Mutable reference to the Docker command being constructed
/// * `memory` - Memory limit (e.g., "4g", "512m") or "unlimited" to disable
/// * `cpus` - CPU limit (e.g., "4", "0.5") or "unlimited" to disable
/// * `pids_limit` - Maximum number of processes (e.g., "256") or "unlimited" to disable
///
/// # Security
///
/// Default limits (4GB RAM, 4 CPUs, 256 PIDs) are sufficient for normal AI agent operations
/// while preventing resource-based attacks. Disabling limits is not recommended unless
/// you trust the agent completely and understand the risks.
///
/// # Examples
///
/// ```no_run
/// use std::process::Command;
/// let mut cmd = Command::new("docker");
/// apply_resource_limits(&mut cmd, "4g", "4", "256");
/// ```
fn apply_resource_limits(docker_cmd: &mut Command, memory: &str, cpus: &str, pids_limit: &str) {
    if memory.to_lowercase() == "unlimited" {
        println!("[RustyYOLO] ⚠️  Memory limit disabled");
    } else {
        docker_cmd.arg("--memory").arg(memory);
        println!("[RustyYOLO] Memory limit: {memory}");
    }

    if cpus.to_lowercase() == "unlimited" {
        println!("[RustyYOLO] ⚠️  CPU limit disabled");
    } else {
        docker_cmd.arg("--cpus").arg(cpus);
        println!("[RustyYOLO] CPU limit: {cpus}");
    }

    if pids_limit.to_lowercase() == "unlimited" {
        println!("[RustyYOLO] ⚠️  PIDs limit disabled");
    } else {
        docker_cmd.arg("--pids-limit").arg(pids_limit);
        println!("[RustyYOLO] PIDs limit: {pids_limit}");
    }
}

/// Configures DNS server restrictions to prevent DNS tunneling and data exfiltration attacks.
///
/// This function restricts which DNS servers the container can query, preventing attacks where:
/// - Data is exfiltrated via DNS queries to attacker-controlled servers
/// - Commands are received via DNS TXT records (DNS tunneling)
/// - Information is leaked through DNS query patterns
///
/// # Arguments
///
/// * `docker_cmd` - Mutable reference to the Docker command being constructed
/// * `dns_servers` - Space-separated list of allowed DNS server IPs, or "any" to disable restrictions
///
/// # Security
///
/// Default DNS servers (Google 8.8.8.8/8.8.4.4 and Cloudflare 1.1.1.1/1.0.0.1) are public,
/// well-known servers that are unlikely to be controlled by attackers. Restricting DNS
/// prevents using arbitrary servers for data exfiltration.
///
/// Setting `dns_servers` to "any" disables this protection and is **not recommended**.
///
/// # Examples
///
/// ```no_run
/// use std::process::Command;
/// let mut cmd = Command::new("docker");
/// configure_dns_restrictions(&mut cmd, "8.8.8.8 1.1.1.1");
/// ```
fn configure_dns_restrictions(docker_cmd: &mut Command, dns_servers: &str) {
    if dns_servers.to_lowercase() == "any" {
        println!("[RustyYOLO] ⚠️  DNS restrictions disabled - exfiltration risk!");
        docker_cmd.arg("-e").arg("DNS_SERVERS=any");
    } else {
        println!("[RustyYOLO] Allowed DNS servers: {dns_servers}");
        docker_cmd.arg("-e").arg(format!("DNS_SERVERS={dns_servers}"));

        // Configure Docker to use these DNS servers
        // This ensures the container actually queries these servers instead of Docker's default
        for dns_server in dns_servers.split_whitespace() {
            docker_cmd.arg("--dns").arg(dns_server);
        }
    }
}

/// Configures audit logging level for security events in the container.
///
/// This function enables logging of security-relevant events for forensics and monitoring:
/// - Blocked network connections (helps diagnose connectivity issues)
/// - Allowed network connections (helps understand agent behavior)
/// - Syscall denials (helps debug seccomp issues)
/// - Resource usage patterns (helps detect anomalies)
///
/// # Arguments
///
/// * `docker_cmd` - Mutable reference to the Docker command being constructed
/// * `audit_log` - Audit logging level:
///   - "none" - No audit logging (default, minimal output)
///   - "basic" - Log blocked events only (security violations)
///   - "verbose" - Log all security events (allowed + blocked)
///
/// # Usage
///
/// Logs are accessible via `docker logs <container-id>` after the container exits.
///
/// # Examples
///
/// ```no_run
/// use std::process::Command;
/// let mut cmd = Command::new("docker");
/// configure_audit_logging(&mut cmd, "basic");
/// ```
fn configure_audit_logging(docker_cmd: &mut Command, audit_log: &str) {
    let audit_level = audit_log.to_lowercase();
    match audit_level.as_str() {
        "none" => {
            // No logging - default behavior
        }
        "basic" => {
            println!("[RustyYOLO] Audit logging: basic (blocked events only)");
            docker_cmd.arg("-e").arg("AUDIT_LOG=basic");
        }
        "verbose" => {
            println!("[RustyYOLO] Audit logging: verbose (all security events)");
            docker_cmd.arg("-e").arg("AUDIT_LOG=verbose");
        }
        _ => {
            eprintln!("[RustyYOLO] ⚠️  Invalid audit-log value: '{audit_level}'. Using 'none'.");
        }
    }
}

/// Setup filesystem isolation by mounting volumes and setting working directory.
fn setup_filesystem_isolation(
    docker_cmd: &mut Command,
    volumes: Vec<String>,
    envs: Vec<String>,
    auth_home: Option<PathBuf>,
) {
    // --- 1. Filesystem Isolation ---
    let pwd = env::current_dir().expect("Failed to get current directory");
    docker_cmd.arg("-v").arg(format!("{}:/app", pwd.display()));
    docker_cmd.arg("-w").arg("/app");

    // Add user-specified volumes
    for vol in volumes {
        println!("[RustyYOLO] Mounting volume: {vol}");
        docker_cmd.arg("-v").arg(vol);
    }

    // Add user-specified env vars
    for env_var in envs {
        docker_cmd.arg("-e").arg(env_var);
    }

    // Mount persistent auth/history directories
    let default_auth_home =
        dirs::config_dir().unwrap_or(PathBuf::from("~/.config")).join("rustyolo");
    let auth_home_path = auth_home.unwrap_or(default_auth_home);

    // Ensure the directory exists on the host
    if !auth_home_path.exists() {
        std::fs::create_dir_all(&auth_home_path).expect("Failed to create auth-home directory");
    }

    let auth_path = auth_home_path
        .canonicalize()
        .expect("Failed to get absolute path for --auth-home");

    let container_auth_path = "/home/agent/.config/rustyolo";
    println!(
        "[RustyYOLO] Mounting auth home: {} -> {}",
        auth_path.display(),
        container_auth_path
    );
    docker_cmd
        .arg("-v")
        .arg(format!("{}:{container_auth_path}", auth_path.display()));
    docker_cmd.arg("-e").arg(format!("PERSISTENT_DIRS={container_auth_path}"));
}

fn run_agent(args: RunArgs) {
    // Validate volumes before constructing the Docker command
    if let Some(error_msg) = validate_volumes(&args.volumes) {
        eprintln!("[RustyYOLO] ❌ Dangerous volume mount detected!");
        eprintln!("[RustyYOLO] {error_msg}");
        std::process::exit(1);
    }

    let mut docker_cmd = Command::new("docker");
    docker_cmd.arg("run").arg("-it").arg("--rm");

    // --- 4. Syscall Isolation (Seccomp) ---
    let _seccomp_temp_file = setup_seccomp(&mut docker_cmd, args.seccomp_profile.as_deref());

    // --- 3. Network Isolation ---
    // Drop all capabilities and only add NET_ADMIN (needed for iptables)
    docker_cmd.arg("--cap-drop=ALL");
    docker_cmd.arg("--cap-add=NET_ADMIN");

    // Prevent privilege escalation via setuid/setgid binaries
    docker_cmd.arg("--security-opt").arg("no-new-privileges");

    // Disable IPv6 to prevent firewall bypass (iptables only configures IPv4)
    docker_cmd.arg("--sysctl").arg("net.ipv6.conf.all.disable_ipv6=1");

    // --- Resource Limits (Defense against DoS/crypto mining) ---
    apply_resource_limits(&mut docker_cmd, &args.memory, &args.cpus, &args.pids_limit);

    // --- DNS Restrictions (Defense against DNS exfiltration) ---
    configure_dns_restrictions(&mut docker_cmd, &args.dns_servers);

    // --- Audit Logging ---
    configure_audit_logging(&mut docker_cmd, &args.audit_log);

    // Build the trusted domains list
    let mut trusted_domains = args.allow_domains.unwrap_or_default();

    // If using Claude, ensure Anthropic API domains are included
    if args.agent == "claude" {
        if trusted_domains.is_empty() {
            trusted_domains = ANTHROPIC_DOMAINS.to_string();
        } else if !trusted_domains.contains("anthropic.com") {
            trusted_domains = format!("{trusted_domains} {ANTHROPIC_DOMAINS}");
        }
    }

    // Pass the domains to the container if any are set
    if !trusted_domains.is_empty() {
        docker_cmd.arg("-e").arg(format!("TRUSTED_DOMAINS={trusted_domains}"));
    }

    // --- 2. Privilege Isolation ---
    let uid = Command::new("id").arg("-u").output().expect("Failed to get UID");
    let gid = Command::new("id").arg("-g").output().expect("Failed to get GID");

    let uid_str = String::from_utf8_lossy(&uid.stdout).trim().to_string();
    let gid_str = String::from_utf8_lossy(&gid.stdout).trim().to_string();

    docker_cmd.arg("-e").arg(format!("AGENT_UID={uid_str}"));
    docker_cmd.arg("-e").arg(format!("AGENT_GID={gid_str}"));

    // --- 1. Filesystem Isolation ---
    setup_filesystem_isolation(&mut docker_cmd, args.volumes, args.envs, args.auth_home);

    // Add the image
    docker_cmd.arg(&args.image);

    // Add the agent command
    docker_cmd.arg(&args.agent); // Always add agent name

    // Prepare system prompt injection
    let default_sandbox_message = "You are operating within a sandboxed Docker environment with restricted access. \
        The sandbox enforces four layers of security: (1) Filesystem isolation - you can only access the mounted \
        project directory and explicitly mounted volumes; (2) Privilege isolation - you are running as a non-root \
        user with limited permissions; (3) Network isolation - outbound traffic is blocked except for DNS and \
        explicitly whitelisted domains; (4) Syscall isolation - dangerous system calls are blocked via seccomp \
        (e.g., kernel module loading, process debugging, system reboots). If you need additional permissions, \
        filesystem access, or network access to complete a task, please ask the operator to adjust the sandbox \
        configuration.";

    let inject_message = match &args.inject_message {
        Some(msg) if msg.to_lowercase() == "none" => None, // User explicitly disabled
        Some(msg) => Some(msg.as_str()),                   // User provided custom message
        None => Some(default_sandbox_message),             // Use default
    };

    if args.additional.is_empty() {
        // If no args are given, assume default "YOLO" mode
        if args.agent == "claude" {
            docker_cmd.arg("--dangerously-skip-permissions");

            // Inject system prompt for Claude
            if let Some(message) = inject_message {
                docker_cmd.arg("--append-system-prompt");
                docker_cmd.arg(message);
            }
        }
        // Add default "danger" flags for other agents here as they become available
        // e.g., aider, cursor, etc.
    } else {
        // Pass user's explicit args (e.g., "claude --help")
        docker_cmd.args(args.additional);

        // Still inject system prompt even with custom args (if agent is claude)
        if args.agent == "claude" {
            if let Some(message) = inject_message {
                docker_cmd.arg("--append-system-prompt");
                docker_cmd.arg(message);
            }
        }
    }

    // --- Run the Command ---
    println!("[RustyYOLO] Starting secure container...");
    println!("[RustyYOLO] Full command: {docker_cmd:?}");

    // Handle dry-run mode
    if args.dry_run {
        println!("[RustyYOLO] Dry run mode - not executing command");
        println!("[RustyYOLO] Command would be:");
        // Print a more readable command format
        let cmd_parts: Vec<String> =
            docker_cmd.get_args().map(|s| s.to_string_lossy().to_string()).collect();
        println!("docker {}", cmd_parts.join(" "));
        return;
    }

    let mut child = docker_cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute docker command.");

    let status = child.wait().expect("Failed to wait on docker command.");
    if !status.success() {
        eprintln!("[RustyYOLO] Container exited with an error.");
        std::process::exit(status.code().unwrap_or(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for validate_volumes function
    #[test]
    fn test_validate_volumes_safe_mounts() {
        // Safe volume mounts should pass
        let safe_volumes = vec![
            "~/.ssh:/home/agent/.ssh:ro".to_string(),
            "~/.gitconfig:/home/agent/.gitconfig:ro".to_string(),
            "/home/user/project:/app".to_string(),
            "/tmp/data:/data:ro".to_string(),
        ];
        assert!(validate_volumes(&safe_volumes).is_none());
    }

    #[test]
    fn test_validate_volumes_docker_socket() {
        // Docker socket mounts should be blocked
        let dangerous = vec!["/var/run/docker.sock:/var/run/docker.sock".to_string()];
        let result = validate_volumes(&dangerous);
        assert!(result.is_some());
        assert!(result.unwrap().contains("Docker socket"));
    }

    #[test]
    fn test_validate_volumes_docker_socket_uppercase() {
        // Case-insensitive check for docker.sock
        let dangerous = vec!["/var/run/DOCKER.SOCK:/var/run/docker.sock".to_string()];
        let result = validate_volumes(&dangerous);
        assert!(result.is_some());
    }

    #[test]
    fn test_validate_volumes_proc_mount() {
        // /proc mounts should be blocked
        let dangerous = vec!["/proc:/proc".to_string()];
        let result = validate_volumes(&dangerous);
        assert!(result.is_some());
        assert!(result.unwrap().contains("/proc"));
    }

    #[test]
    fn test_validate_volumes_sys_mount() {
        // /sys mounts should be blocked
        let dangerous = vec!["/sys:/sys:ro".to_string()];
        let result = validate_volumes(&dangerous);
        assert!(result.is_some());
        assert!(result.unwrap().contains("/sys"));
    }

    #[test]
    fn test_validate_volumes_dev_mount() {
        // /dev mounts should be blocked
        let dangerous = vec!["/dev:/dev".to_string()];
        let result = validate_volumes(&dangerous);
        assert!(result.is_some());
        assert!(result.unwrap().contains("/dev"));
    }

    #[test]
    fn test_validate_volumes_boot_mount() {
        // /boot mounts should be blocked
        let dangerous = vec!["/boot:/boot".to_string()];
        let result = validate_volumes(&dangerous);
        assert!(result.is_some());
        assert!(result.unwrap().contains("/boot"));
    }

    #[test]
    fn test_validate_volumes_etc_mount() {
        // /etc mounts should be blocked
        let dangerous = vec!["/etc:/etc:ro".to_string()];
        let result = validate_volumes(&dangerous);
        assert!(result.is_some());
        assert!(result.unwrap().contains("/etc"));
    }

    #[test]
    fn test_validate_volumes_proc_subdirectory_allowed() {
        // User projects with "proc" in the name should be allowed
        let safe = vec!["/home/user/myproc:/myproc".to_string()];
        assert!(validate_volumes(&safe).is_none());
    }

    #[test]
    fn test_validate_volumes_mixed_safe_and_dangerous() {
        // If any volume is dangerous, should fail
        let mixed = vec!["~/.ssh:/home/agent/.ssh:ro".to_string(), "/proc:/proc".to_string()];
        let result = validate_volumes(&mixed);
        assert!(result.is_some());
    }

    #[test]
    fn test_validate_volumes_empty_list() {
        // Empty volume list should pass
        let empty: Vec<String> = vec![];
        assert!(validate_volumes(&empty).is_none());
    }

    // Tests for setup_seccomp function
    #[test]
    fn test_setup_seccomp_none() {
        // When seccomp is explicitly disabled
        let mut cmd = Command::new("docker");
        let result = setup_seccomp(&mut cmd, Some("none"));
        assert!(result.is_none());
        // The command should have --security-opt seccomp=unconfined
        let args: Vec<String> = cmd.get_args().map(|s| s.to_string_lossy().to_string()).collect();
        assert!(args.contains(&"--security-opt".to_string()));
        assert!(args.contains(&"seccomp=unconfined".to_string()));
    }

    #[test]
    fn test_setup_seccomp_default() {
        // When using the default embedded profile
        let mut cmd = Command::new("docker");
        let result = setup_seccomp(&mut cmd, None);
        assert!(result.is_some());
        // The command should have --security-opt seccomp=<path>
        let args: Vec<String> = cmd.get_args().map(|s| s.to_string_lossy().to_string()).collect();
        assert!(args.contains(&"--security-opt".to_string()));
        // Should have a seccomp profile path
        assert!(args.iter().any(|arg| arg.starts_with("seccomp=")));
    }
}
