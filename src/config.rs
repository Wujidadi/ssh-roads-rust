use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub key: String,
    pub name: String,
    pub ip: String,
    pub port: String,
    pub conn_type: String,
    pub user: String,
    pub comment: String,

    // Optional fields
    #[serde(default)]
    pub pswd: Option<String>,
    #[serde(default)]
    pub gcp_project: Option<String>,
    #[serde(default)]
    pub gcp_zone: Option<String>,
    #[serde(default)]
    pub gcp_vm_name: Option<String>,
}

// Helper method to resolve environment variables
impl ServerConfig {
    pub fn resolve(&self, value: &str) -> String {
        if value.starts_with('$') {
            let var_name = &value[1..];
            env::var(var_name).unwrap_or_else(|_| {
                eprintln!("Warning: Environment variable {} not found", var_name);
                value.to_string() // Return the original string if not found
            })
        } else {
            value.to_string()
        }
    }

    // Helper method to get resolved port as u16
    pub fn resolved_port(&self) -> u16 {
        self.resolve(&self.port).parse::<u16>().unwrap_or(22)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub servers: Vec<ServerConfig>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Self::get_resource_path("servers.json");
        let content = fs::read_to_string(&path).map_err(|e| {
            format!("Failed to read config file {:?}: {}", path, e)
        })?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    // Helper method to get the resource path
    pub fn get_resource_path(filename: &str) -> PathBuf {
        // Try the user's home directory .ssh-roads folder (Primary for global installation)
        if let Ok(home) = env::var("HOME") {
            let global = Path::new(&home).join(".ssh-roads").join(filename);
            if global.exists() {
                return global;
            }
        }

        // Try the current working directory (Secondary, for development)
        let local = PathBuf::from(filename);
        if local.exists() {
            return local;
        }

        local // Return the local path if not found, to allow later error reporting
    }
}
