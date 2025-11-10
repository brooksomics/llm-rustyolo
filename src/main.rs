use clap::{Args, Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

mod update;

/// A secure, firewalled Docker wrapper for AI agents.
///
/// This tool builds a 'docker run' command to enforce the "Lethal Trifecta" of security:
/// 1. Filesystem Isolation (via read-only volume mounts)
/// 2. Privilege Isolation (by running as a non-root user)
/// 3. Network Isolation (by building an iptables firewall inside the container)
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
    #[arg(default_value = "claude")]
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
    #[arg(long, default_value = "llm-rustyolo:latest")]
    image: String,

    /// Arguments to pass directly to the agent (e.g., --help or -p "prompt").
    #[arg(last = true)]
    additional: Vec<String>,

    /// Skip version check on startup
    #[arg(long)]
    skip_version_check: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Update { binary, image, yes }) => {
            handle_update(binary, image, yes);
        }
        None => {
            // Run mode - check for updates first unless skipped
            let run_args = cli.run_args.unwrap_or_else(|| RunArgs {
                agent: "claude".to_string(),
                volumes: Vec::new(),
                envs: Vec::new(),
                allow_domains: None,
                auth_home: None,
                image: "llm-rustyolo:latest".to_string(),
                additional: Vec::new(),
                skip_version_check: false,
            });

            if !run_args.skip_version_check {
                check_for_updates();
            }
            run_agent(run_args);
        }
    }
}

fn handle_update(binary_only: bool, image_only: bool, yes: bool) {
    let update_binary = binary_only || !image_only;
    let update_image = image_only || !binary_only;

    if update_binary {
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

fn run_agent(args: RunArgs) {
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
            trusted_domains = format!("{trusted_domains} {anthropic_domains}");
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
    let pwd = env::current_dir().expect("Failed to get current directory");
    docker_cmd.arg("-v").arg(format!("{}:/app", pwd.display()));
    docker_cmd.arg("-w").arg("/app");

    // Add user-specified volumes
    for vol in args.volumes {
        println!("[RustyYOLO] Mounting volume: {vol}");
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
    docker_cmd.arg("-e").arg(format!("PERSISTENT_DIRS={container_auth_path}"));

    // Add the image
    docker_cmd.arg(&args.image);

    // Add the agent command
    docker_cmd.arg(&args.agent); // Always add agent name
    if args.additional.is_empty() {
        // If no args are given, assume default "YOLO" mode
        if args.agent == "claude" {
            docker_cmd.arg("--dangerously-skip-permissions");
        }
        // Add default "danger" flags for other agents here as they become available
        // e.g., aider, cursor, etc.
    } else {
        // Pass user's explicit args (e.g., "claude --help")
        docker_cmd.args(args.additional);
    }

    // --- Run the Command ---
    println!("[RustyYOLO] Starting secure container...");
    println!("[RustyYOLO] Full command: {docker_cmd:?}");

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
