# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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
```

## Architecture

Entry point is `main.rs`, with the following startup flow:

1. `Config::get_env_path()` locates the `.env` file (checks `~/.ssh-roads/.env` first, then the local path if cwd is `.ssh-roads/`) and loads environment variables
2. `Config::load()` reads `servers.json` (checks `~/.ssh-roads/servers.json` first, then cwd)
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

### Connection Types

- `password`: Uses the system `expect` command to fill in the password automatically; falls back to interactive `ssh` if `expect` is not available
- `key`: `ssh -i <ssh_key>` private key authentication with `IdentitiesOnly=yes`
- `gcp`: Invokes `gcloud compute ssh`

### Config File Format

All string fields in `servers.json` support the `$VAR_NAME` format and are automatically substituted from loaded environment variables.
The `port` field is hidden in the menu when it is `22` or omitted.
GCP type can omit `pswd`; `key` type requires an `ssh_key` path; non-GCP servers can omit all `gcp_*` fields.

## Git Commit Guidelines

See `.github/git-commit-instructions.md` for details. Summary:

- Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format
- For large or complex changes, include at least one bullet point below the subject line describing the reason for each file changed
- Do **not** add a `Co-Authored-By` trailer

## Global Installation

```bash
# One-shot: compile & install + create ~/.ssh-roads/ symlink + create /usr/local/bin/roads shortcut
./setup
```
