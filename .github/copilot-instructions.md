# Copilot Instructions

## Build & Run

```bash
# 開發建置
cargo build

# 正式發布建置
cargo build --release

# 安裝到 ~/.cargo/bin
cargo install --path .

# 執行（開發中）
cargo run
cargo run -- <SERVER_KEY>

# 一鍵安裝 + 建立符號連結 + 建立 /usr/local/bin/roads 捷徑
./setup
```

此專案無自動化測試套件。

## Architecture

程式入口 `main.rs` 的啟動流程：

1. `Config::get_env_path()` 定位 `.env`（優先 `~/.ssh-roads/.env`，其次 cwd 為 `.ssh-roads/` 時的本機路徑）並載入環境變數
2. `Config::load()` 讀取 `servers.json`（優先 `~/.ssh-roads/servers.json`，其次 cwd）
3. `ServerManager::show_menu()` 顯示對齊排版的伺服器清單
4. 從 CLI 引數或 stdin 取得 server key
5. `ServerManager::connect_server()` 依 `conn_type` 分派至對應連線方法

### 模組職責

| 模組        | 職責                                                                                        |
| ----------- | ------------------------------------------------------------------------------------------- |
| `config.rs` | `ServerConfig`（單一伺服器設定）與 `Config`（清單），含 `$VAR` 環境變數解析及設定檔路徑查找 |
| `server.rs` | `ServerManager`：選單顯示（Unicode 全形字元寬度對齊）與三種連線實作                         |
| `cli.rs`    | stdin 讀取輸入                                                                              |
| `main.rs`   | 組合上述流程，使用 Clap 解析可選的 `route` 引數                                             |

### 連線類型（`conn_type`）

- `password`：透過系統 `expect` 指令自動填入密碼；`expect` 不存在時回退至互動式 `ssh`
- `key`：`ssh -i <ssh_key>` 私鑰認證，固定帶 `IdentitiesOnly=yes`
- `gcp`：調用 `gcloud compute ssh`

## Key Conventions

### 環境變數展開

`servers.json` 所有字串欄位均支援 `$VAR_NAME` 格式。展開邏輯在 `ServerConfig::resolve()`，若變數不存在會印出警告並保留原始字串。

### 設定檔可選欄位

- `port` 為 `22` 或省略時，選單不顯示埠號
- GCP 類型：可省略 `pswd`；`key` 類型：必須有 `ssh_key`
- 非 GCP 伺服器可省略所有 `gcp_*` 欄位（以 `#[serde(default)]` 標記）

### Unicode 對齊

選單使用 `unicode-width` crate 的 `UnicodeWidthStr::width()` 計算顯示寬度（全形字元 = 2），再補空白對齊，勿改用 Rust 內建的 `len()`。

### `password` 連線的 expect 腳本

密碼透過環境變數（`SSH_HOST`、`SSH_USER`、`SSH_PSWD`、`SSH_PORT`）傳入 expect 腳本，避免密碼出現在 argv 中。

## Git Commit 規範

詳見 `.github/git-commit-instructions.md`，摘要如下：

- 一律使用繁體中文（台灣），採台灣標準翻譯與慣用術語
- 遵循 [Conventional Commits](https://www.conventionalcommits.org/zh-hant/) 格式
- 變動複雜時，標題外須有至少一項 bullet point 說明各檔案異動原因
- Commit message **不加** `Co-Authored-By` 署名
