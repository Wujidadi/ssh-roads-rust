# Copilot Instructions

## Build & Run

```bash
# Development build
cargo build

# Release build
cargo build --release

# Install to ~/.cargo/bin
cargo install --path .

# Run directly (during development)
cargo run
cargo run -- <SERVER_KEY>

# One-shot: compile & install + create ~/.ssh-roads/ symlink + create /usr/local/bin/roads shortcut
./setup
```

This project has no automated test suite.

## Architecture

Entry point is `main.rs`, with the following startup flow:

1. `Config::get_env_path()` locates the `.env` file (prefers `~/.ssh-roads/.env`; falls back to the local path when cwd is `.ssh-roads/`) and loads environment variables
2. `Config::load()` reads `servers.json` (prefers `~/.ssh-roads/servers.json`, then cwd)
3. `ServerManager::show_menu()` displays the aligned server list
4. Server key is obtained from CLI arguments or stdin
5. `ServerManager::connect_server()` dispatches to the corresponding connection method based on `conn_type`

### Module Responsibilities

| Module      | Responsibility                                                                                                                                 |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `config.rs` | `ServerConfig` (single server config) and `Config` (server list), including `$VAR` environment variable resolution and config file path lookup |
| `server.rs` | `ServerManager`: menu display (Unicode full-width character alignment) and three connection implementations                                    |
| `cli.rs`    | Reads input from stdin                                                                                                                         |
| `main.rs`   | Composes the above flow; uses Clap to parse the optional `route` argument                                                                      |

### Connection Types (`conn_type`)

- `password`: Uses the system `expect` command to fill in the password automatically; falls back to interactive `ssh` if `expect` is not available
- `key`: `ssh -i <ssh_key>` private key authentication with `IdentitiesOnly=yes`
- `gcp`: Invokes `gcloud compute ssh`

## Key Conventions

### Environment Variable Expansion

All string fields in `servers.json` support the `$VAR_NAME` format.
Expansion logic lives in `ServerConfig::resolve()`: if a variable is not found, a warning is printed and the original string is kept.

### Optional Config Fields

- When `port` is `22` or omitted, the port is not shown in the menu
- GCP type: `pswd` may be omitted; `key` type: `ssh_key` is required
- Non-GCP servers may omit all `gcp_*` fields (marked with `#[serde(default)]`)

### Unicode Alignment

The menu uses `UnicodeWidthStr::width()` from the `unicode-width` crate to measure display width (full-width characters = 2) and pads with spaces accordingly.
Do not use Rust's built-in `len()` for this purpose.

### `password` Connection — expect Script

The password is passed into the expect script via environment variables (`SSH_HOST`, `SSH_USER`, `SSH_PSWD`, `SSH_PORT`) to avoid exposing it in the process argument list.

## Git Commit Guidelines

See `.github/git-commit-instructions.md` for details. Summary:

- Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format
- For complex changes, include at least one bullet point below the subject line explaining the reason for each file changed
- Do **not** add a `Co-Authored-By` trailer
