use clap::Parser;
use colored::*;
use std::process;

mod config;
mod server;
mod cli;

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
    // Load environment variables
    let env_path = Config::get_resource_path(".env");
    if env_path.exists() {
        dotenv::from_path(env_path).ok();
    } else {
        dotenv::dotenv().ok();
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
