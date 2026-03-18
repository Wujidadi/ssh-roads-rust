use clap::Parser;
use colored::*;
use std::process;

mod cli;
mod config;
mod server;

use config::Config;
use server::ServerManager;

#[derive(Parser)]
#[command(name = "ssh-roads")]
#[command(about = "SSH connection manager", long_about = None)]
struct Args {
    /// Server key to connect to
    route: Option<String>,
}

fn main() {
    // Load environment variables - only from project directories
    if let Some(env_path) = Config::get_env_path() {
        dotenv::from_path(env_path).ok();
    }

    let args = Args::parse();

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("{}", format!("Error loading config: {}", e).red().bold());
        process::exit(1);
    });

    let manager = ServerManager::new(config);

    // Show menu
    manager.show_menu();

    // Get the route from argument or stdin
    let route = if let Some(r) = args.route {
        r
    } else {
        cli::read_input()
    };

    // Connect to server
    if let Err(e) = manager.connect_server(&route) {
        eprintln!("{} {}", "Error:".red().bold(), e.to_string().red());
        process::exit(1);
    }
}
