use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// A secure, firewalled Docker wrapper for AI agents.
///
/// This tool builds a 'docker run' command to enforce the "Lethal Trifecta" of security:
/// 1. Filesystem Isolation (via read-only volume mounts)
/// 2. Privilege Isolation (by running as a non-root user)
/// 3. Network Isolation (by building an iptables firewall inside the container)
#[derive(Parser, Debug)]
#[command(name = "rustyolo", version, about, long_about = None)]
struct Args {
    /// The agent to run (e.g., 'claude', 'codex', 'gemini-cli').
    #[arg(index = 1, default_value = "claude")]
    agent: String,

    /// Additional volumes to mount (e.g., -v ~/.ssh:/home/agent/.ssh:ro)
    #[arg(short = 'v', long = "volume")]
    volumes: Vec<String>,

    /// Environment variables to pass (e.g., -e MY_VAR=value)
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
    #[arg(long, default_value = "llm-rustyolo:latest")]
    image: String,

    /// Arguments to pass directly to the agent (e.g., --help or -p "prompt").
    #[arg(last = true)]
    agent_args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut docker_cmd = Command::new("docker");
    docker_cmd.arg("run").arg("-it").arg("--rm");

    // --- 3. Network Isolation ---
    docker_cmd.arg("--cap-add=NET_ADMIN");

    // Build the trusted domains list
    let mut trusted_domains = args.allow_domains.unwrap_or_default();

    // If using Claude, ensure Anthropic API domains are included
    if args.agent == "claude" {
        let anthropic_domains = "api.anthropic.com anthropic.com";
        if trusted_domains.is_empty() {
            trusted_domains = anthropic_domains.to_string();
        } else if !trusted_domains.contains("anthropic.com") {
            trusted_domains = format!("{} {}", trusted_domains, anthropic_domains);
        }
    }

    // Pass the domains to the container if any are set
    if !trusted_domains.is_empty() {
        docker_cmd.arg("-e").arg(format!("TRUSTED_DOMAINS={}", trusted_domains));
    }

    // --- 2. Privilege Isolation ---
    let uid = Command::new("id").arg("-u").output().expect("Failed to get UID");
    let gid = Command::new("id").arg("-g").output().expect("Failed to get GID");

    docker_cmd
        .arg("-e")
        .arg(format!("AGENT_UID={}", String::from_utf8_lossy(&uid.stdout).trim()));
    docker_cmd
        .arg("-e")
        .arg(format!("AGENT_GID={}", String::from_utf8_lossy(&gid.stdout).trim()));

    // --- 1. Filesystem Isolation ---
    let pwd = env::current_dir().expect("Failed to get current directory");
    docker_cmd.arg("-v").arg(format!("{}:/app", pwd.display()));
    docker_cmd.arg("-w").arg("/app");

    // Add user-specified volumes
    for vol in args.volumes {
        println!("[RustyYOLO] Mounting volume: {}", vol);
        docker_cmd.arg("-v").arg(vol);
    }

    // Add user-specified env vars
    for env_var in args.envs {
        docker_cmd.arg("-e").arg(env_var);
    }

    // Mount persistent auth/history directories
    let default_auth_home =
        dirs::config_dir().unwrap_or(PathBuf::from("~/.config")).join("rustyolo");
    let auth_home_path = args.auth_home.unwrap_or(default_auth_home);

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
        .arg(format!("{}:{}", auth_path.display(), container_auth_path));
    docker_cmd.arg("-e").arg(format!("PERSISTENT_DIRS={}", container_auth_path));

    // Add the image
    docker_cmd.arg(&args.image);

    // Add the agent command
    docker_cmd.arg(&args.agent); // Always add agent name
    if args.agent_args.is_empty() {
        // If no args are given, assume default "YOLO" mode
        if args.agent == "claude" {
            docker_cmd.arg("--dangerously-skip-permissions");
        }
        // Add default "danger" flags for other agents here as they become available
        // e.g., aider, cursor, etc.
    } else {
        // Pass user's explicit args (e.g., "claude --help")
        docker_cmd.args(args.agent_args);
    }

    // --- Run the Command ---
    println!("[RustyYOLO] Starting secure container...");
    println!("[RustyYOLO] Full command: {:?}", docker_cmd);

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
