# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
# 開發建置
cargo build

# 正式發布建置
cargo build --release

# 安裝到 ~/.cargo/bin
cargo install --path .

# 直接執行（開發中）
cargo run
cargo run -- <SERVER_KEY>
```

## Architecture

程式入口為 `main.rs`，流程如下：

1. `Config::get_env_path()` 定位 `.env` 檔案（先找 `~/.ssh-roads/.env`，再判斷當前目錄是否為 `.ssh-roads/`）並載入環境變數
2. `Config::load()` 讀取 `servers.json`（先找 `~/.ssh-roads/servers.json`，再找當前目錄）
3. `ServerManager::show_menu()` 顯示對齊排版的伺服器列表
4. 透過命令列引數或 stdin 取得使用者輸入的 server key
5. `ServerManager::connect_server()` 依 `conn_type` 分派至對應連線方法

### 模組職責

| 模組        | 職責                                                                                                  |
| ----------- | ----------------------------------------------------------------------------------------------------- |
| `config.rs` | `ServerConfig`（單一伺服器設定）與 `Config`（伺服器列表），含 `$VAR` 環境變數解析邏輯及設定檔路徑查找 |
| `server.rs` | `ServerManager`：選單顯示（Unicode 全形字元寬度對齊）與三種連線實作                                   |
| `cli.rs`    | stdin 讀取輸入                                                                                        |
| `main.rs`   | 組合上述流程，使用 Clap 解析可選的 `route` 引數                                                       |

### 連線類型

- `password`：使用系統 `expect` 指令自動填入密碼；若 `expect` 不存在則回退到互動式 `ssh`
- `key`：`ssh -i <ssh_key>` 私鑰認證，使用 `IdentitiesOnly=yes`
- `gcp`：調用 `gcloud compute ssh`

### 設定檔格式

`servers.json` 中所有字串欄位均支援 `$VAR_NAME` 格式，程式會從已載入的環境變數中自動替換。`port` 欄位若為 `22`
或省略則選單不顯示埠號。GCP 類型可省略 `pswd`；`key` 類型需提供 `ssh_key` 路徑；非 GCP 伺服器可省略所有 `gcp_` 欄位。

## Git Commit 規範

詳見 `.github/git-commit-instructions.md`，摘要如下：

- Commit message 一律使用繁體中文（台灣），不夾雜日韓語或其他非中文詞彙
- 使用 [Conventional Commits](https://www.conventionalcommits.org/zh-hant/) 標準格式
- 變動較多或複雜時，標題之外須列出至少一項 bullet point，說明各檔案的異動原因
- Commit message 最後**不加** Co-Authored-By 署名

## Global Installation

```bash
# 一鍵完成：編譯安裝 + 建立 ~/.ssh-roads/ 符號連結 + 建立 /usr/local/bin/roads 捷徑
./setup
```
