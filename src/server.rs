use crate::config::{Config, ServerConfig};
use colored::*;
use std::error::Error;
use std::process::Command;
use unicode_width::UnicodeWidthStr;

pub struct ServerManager {
    config: Config,
}

impl ServerManager {
    pub fn new(config: Config) -> Self {
        ServerManager { config }
    }

    /// Display the server list
    pub fn show_menu(&self) {
        let msg = "Choose one server from below as the target:";
        println!("{}", msg.bold());
        println!();

        for server in &self.config.servers {
            // Key, fixed width as 6 characters
            let key_col = self.pad_str(&server.key, 6);

            // Name, full-width character handling, width as 46 characters
            let name_col = self.pad_str(&server.name, 46);

            // Resolve IP and port
            let real_ip = server.resolve(&server.ip);
            let port_num = server.resolved_port();

            // Address, IP + Port, width as 21 characters
            let address = if port_num == 22 { real_ip } else { format!("{}:{}", real_ip, port_num) };
            let addr_col = self.pad_str(&address, 21);

            // Automatically add parentheses around the comment
            let comment_display = if server.comment.is_empty() {
                "".to_string()
            } else {
                format!("({})", server.comment)
            };

            // Comment, last column, no manual padding required
            println!(
                "{} {} {} {}",
                key_col,
                name_col.yellow().bold(),
                addr_col.green().bold(),
                comment_display.blue().bold()
            );
        }

        println!();
    }

    /// Helper function: pad string with spaces to achieve the desired width
    fn pad_str(&self, s: &str, width: usize) -> String {
        let current_width = s.width(); // Calculate display width considering full-width chars
        if current_width >= width {
            s.to_string()
        } else {
            format!("{}{}", s, " ".repeat(width - current_width))
        }
    }

    /// Find server by key
    pub fn find_server(&self, key: &str) -> Option<&ServerConfig> {
        self.config.servers.iter().find(|s| s.key == key)
    }

    /// Connect to the server by route
    pub fn connect_server(&self, route: &str) -> Result<(), Box<dyn Error>> {
        match self.find_server(route) {
            Some(server) => {
                let real_name = server.resolve(&server.name);
                let real_ip = server.resolve(&server.ip);
                let port_num = server.resolved_port();
                let address = if port_num == 22 || port_num == 0 {
                    real_ip.clone()
                } else {
                    format!("{}:{}", real_ip, port_num)
                };

                let msg = "You chose";
                println!(
                    "{} {} {}",
                    msg.red().bold(),
                    real_name.yellow().bold(),
                    address.green().bold()
                );

                match server.conn_type.as_str() {
                    "password" => self.connect_password(server)?,
                    "gcp" => self.connect_gcp(server)?,
                    _ => return Err("Unknown connection type".into()),
                }
                Ok(())
            }
            None => Err(format!("Server not found: {}", route).into()),
        }
    }

    /// Connect to a server using password authentication
    fn connect_password(&self, server: &ServerConfig) -> Result<(), Box<dyn Error>> {
        let port = server.resolved_port();
        let pswd = server.resolve(server.pswd.as_deref().unwrap_or(""));

        // The former Expect script to automate SSH connection
        // Use $env(...) to read environment variables, avoiding argv parameter passing issues
        let expect_script = r#"
            set host $env(SSH_HOST)
            set user $env(SSH_USER)
            set pswd $env(SSH_PSWD)
            set port $env(SSH_PORT)

            spawn ssh $user@$host -p $port -o StrictHostKeyChecking=no
            expect {
                "*assword:" {
                    send -- "$pswd\n"
                }
                timeout {
                    exit 1
                }
                eof {
                    exit 1
                }
            }
            interact
        "#;

        let status = Command::new("expect")
            .arg("-c")
            .arg(expect_script)
            .env("SSH_HOST", server.resolve(&server.ip))
            .env("SSH_USER", server.resolve(&server.user))
            .env("SSH_PSWD", pswd)
            .env("SSH_PORT", port.to_string())
            .status();

        match status {
            Ok(s) if s.success() => Ok(()),
            Ok(_) => Err("SSH connection failed".into()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Fallback: Use interactive SSH if expect is not installed
                Command::new("ssh")
                    .arg("-p")
                    .arg(port.to_string())
                    .arg("-o")
                    .arg("StrictHostKeyChecking=no")
                    .arg(format!("{}@{}", server.resolve(&server.user), server.resolve(&server.ip)))
                    .status()
                    .map(|s| if s.success() { Ok(()) } else { Err("SSH failed".into()) })?
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Connect to a GCP VM
    fn connect_gcp(&self, server: &ServerConfig) -> Result<(), Box<dyn Error>> {
        // Use .as_deref().unwrap_or("") to safely handle Option
        let project = server.resolve(server.gcp_project.as_deref().unwrap_or(""));
        let zone = server.resolve(server.gcp_zone.as_deref().unwrap_or(""));
        let vm = server.resolve(server.gcp_vm_name.as_deref().unwrap_or(""));

        let status = Command::new("gcloud")
            .arg("compute")
            .arg("ssh")
            .arg("--project").arg(project)
            .arg("--zone").arg(zone)
            .arg(format!("{}@{}", server.resolve(&server.user), vm))
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err("GCP SSH connection failed".into())
        }
    }
}
