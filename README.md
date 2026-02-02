# SSH Roads

`ssh-roads` is a lightweight SSH connection manager written in Rust. It aims to replace cumbersome Bash scripts by providing a more stable, fast, and neatly formatted interface for server connections.

## 🌟 Features

- **Automated Login**: Integrated `expect` engine supports automatic password entry, eliminating the need for manual copy-pasting. It automatically falls back to a standard SSH connection if `expect` is not available in the environment.
- **GCP Integration**: Native support for Google Cloud Platform (GCP) instance connections, simplifying `gcloud compute ssh` commands.
- **Perfect Layout Alignment**: Developed specifically for terminal displays, it supports Unicode width calculation to ensure perfect alignment even when server names contain full-width characters (e.g., Chinese characters).
- **Vibrant Colors**: Uses colors to distinguish keys, server names, IP addresses, and comments for a better visual experience.
- **Dual-Mode Input**: Supports direct connection via command-line arguments or an interactive menu to choose from after execution.

## 📋 System Requirements

- **Rust Environment**: Rust 1.85.0 (Edition 2024) or newer.
- **Expect**: Recommended to install the `expect` command to enable automated password filling (pre-installed by default on macOS).
- **Google Cloud SDK**: Required to install and configure the `gcloud` command if you intend to connect to GCP instances.

## 🚀 Installation & Global Configuration

1. **Compile and Install**:
   Run the following command in the project root to install the executable to `~/.cargo/bin`:
   ```bash
   cargo install --path .
   ```

2. **Automated Environment Configuration**:
   Run the following commands to create the directory and establish symbolic links for the configuration files:
   ```bash
   mkdir -p ~/.ssh-roads
   ln -s "$PWD/servers.json" ~/.ssh-roads/servers.json
   ln -s "$PWD/.env" ~/.ssh-roads/.env
   ```

3. **Create Global Command Link (Optional)**:
   If you wish to use the `roads` command directly from any directory, please run:
   ```bash
   sudo ln -s "$(which ssh-roads)" /usr/local/bin/roads
   ```

Alternatively, use the `setup` script within the project to complete all the above steps with one click:
```bash
./setup
```

## ⚙️ Configuration Instructions

The project prioritizes reading configurations from the executable's directory. If they do not exist, it will read from `~/.ssh-roads/`.

### `servers.json` Format

The fields have been significantly simplified; only mandatory fields need to be present:
```json
{
  "servers": [
    {
      "key": "A",
      "name": "My Server",
      "ip": "$MY_SERVER_IP",
      "port": "$SSH_PORT_DEFAULT",
      "conn_type": "password",
      "user": "$MY_USER",
      "pswd": "$MY_PSWD",
      "comment": "Remark text"
    },
    {
      "key": "GCP",
      "name": "GCP Host",
      "ip": "$GCP_IP",
      "conn_type": "gcp",
      "user": "ubuntu",
      "gcp_project": "$GCP_PROJECT",
      "gcp_zone": "asia-east1-c",
      "gcp_vm_name": "instance-1",
      "comment": "Cloud host"
    }
  ]
}
```
- **Variable Support**: All fields support the `$VARIABLE` format; the program automatically reads corresponding values from `.env`.
- **Automatic Brackets**: The `comment` field does not require manual brackets; the program will automatically add `()` when displaying the menu.
- **Flexible Fields**:
  - For non-GCP servers, `gcp_` related fields can be omitted directly.
  - For GCP servers, `pswd` can be omitted, and the system will invoke `gcloud` for authentication.
  - If `port` is `22` or not provided, the menu display will be simplified.

## 💡 Usage

### 1. Interactive Mode

Execute the program directly to see the server list with a neatly aligned interface, then enter the `key` to connect:
```bash
roads
```

### 2. Quick Connection

If you already know the server's `key`, you can pass it as an argument to skip the menu:
```bash
roads A
```

## 🛠 Tech Stack

- [**Clap**](https://github.com/clap-rs/clap): A powerful command-line argument parser.
- [**Serde**](https://serde.rs/): A high-performance JSON serialization/deserialization tool.
- [**Unicode-Width**](https://github.com/unicode-rs/unicode-width): Accurately calculates display width for full-width/half-width characters.
- [**Colored**](https://github.com/mackwic/colored): Terminal color rendering.
- [**Dotenv**](https://github.com/dotenv-rs/dotenv): Supports reading environment variables.

## 📄 License

This project is for personal use only or may be modified in accordance with the MIT license specifications.
