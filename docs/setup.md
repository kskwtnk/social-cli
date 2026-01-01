# セットアップガイド

## 開発環境構築

### 前提条件

- **Rust**: 1.70以上（推奨: 最新安定版）
- **OS**: macOS, Linux, Windows
- **その他**: インターネット接続、各SNSのアカウント

---

## Rustのインストール

### 新規インストール

```bash
# rustupのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 環境変数を反映
source $HOME/.cargo/env

# バージョン確認
rustc --version
cargo --version
```

### 既存環境のアップデート

```bash
rustup update stable
```

---

## プロジェクトのクローンとビルド

### 1. リポジトリのクローン

```bash
git clone <repository-url>
cd social-cli
```

### 2. 依存関係のインストール

```bash
cargo build
```

初回ビルドは依存関係のダウンロードで時間がかかります（5-10分程度）。

### 3. テスト実行

```bash
cargo test
```

### 4. ビルド（リリースモード）

```bash
cargo build --release
```

バイナリは `target/release/social-cli` に生成されます。

---

## OS別の追加セットアップ

### macOS

特別な設定は不要です。キーチェーンアクセスが自動的に処理されます。

### Linux

#### Secret Service のインストール

```bash
# Debian/Ubuntu
sudo apt-get install libsecret-1-dev

# Fedora
sudo dnf install libsecret-devel

# Arch Linux
sudo pacman -S libsecret
```

#### ビルド時に必要なパッケージ

```bash
# Debian/Ubuntu
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel

# Arch Linux
sudo pacman -S base-devel pkg-config openssl
```

### Windows

Windows Credential Managerが自動的に使用されます。追加セットアップは不要です。

---

## SNS APIの認証情報取得

### Bluesky (必須 - Phase 1)

#### App Passwordの生成

1. Blueskyアカウントにログイン
2. Settings → Privacy and Security → App Passwords
3. "Add App Password" をクリック
4. 名前を入力（例: "social-cli"）
5. 生成されたパスワードをコピー（**一度しか表示されません**）

#### 注意事項

- **通常のパスワードは使用しないでください**
- App Passwordは各アプリケーションごとに個別に生成
- 不要になったら削除可能

---

### X (Twitter) - Phase 2

#### API Tierの確認

X APIは有料化されています。以下のいずれかのプランが必要です：

- **Free**: 読み取り専用（投稿不可の可能性あり）
- **Basic**: $100/月 - 基本的な投稿機能
- **Pro**: $5,000/月 - 高度な機能

#### APIキーの取得（Basic以上）

1. [X Developer Portal](https://developer.x.com/) にアクセス
2. "Create Project" → "Create App"
3. API Key、API Secret、Access Token、Access Token Secretを取得
4. 必要なパーミッション: **Read and Write**

---

### Threads - Phase 3

#### Meta for Developers

1. [Meta for Developers](https://developers.facebook.com/) でアカウント作成
2. アプリケーション登録
3. Threads APIへのアクセス申請
4. アクセストークン取得

**注意**: Threads APIは限定公開中。アクセスには審査が必要な場合があります。

---

## 初回セットアップ

### 1. バイナリのインストール（オプション）

```bash
# システム全体で使用する場合
cargo install --path .

# または、パスを通す
export PATH="$PATH:$(pwd)/target/release"
```

### 2. 設定ディレクトリの確認

設定ファイルは以下の場所に保存されます：

- **macOS/Linux**: `~/.config/social-cli/`
- **Windows**: `%APPDATA%\social-cli\`

ディレクトリは初回実行時に自動作成されます。

### 3. Blueskyのセットアップ

```bash
social-cli setup
```

対話形式で以下を入力：

```
Social CLI Setup
================

Bluesky Setup
-------------
Bluesky Handle or Email: your-handle.bsky.social
App Password: ****
✓ Bluesky configured successfully

✓ Configuration saved successfully!
```

### 4. 設定の確認

```bash
social-cli status
```

出力例：

```
Social CLI Configuration Status
================================

Bluesky:
  Status: ✓ Enabled
  Handle: your-handle.bsky.social

Twitter:
  Status: ✗ Not configured
```

---

## トラブルシューティング

### ビルドエラー

#### 1. リンカーエラー (Linux)

**エラー**: `error: linker 'cc' not found`

**解決策**:

```bash
# Debian/Ubuntu
sudo apt-get install build-essential

# Fedora
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

#### 2. OpenSSLエラー

**エラー**: `Could not find directory of OpenSSL installation`

**解決策**:

```bash
# Debian/Ubuntu
sudo apt-get install libssl-dev pkg-config

# Fedora
sudo dnf install openssl-devel pkg-config

# macOS
brew install openssl
```

#### 3. libsecretエラー (Linux)

**エラー**: `error: could not find system library 'libsecret-1'`

**解決策**:

```bash
# Debian/Ubuntu
sudo apt-get install libsecret-1-dev

# Fedora
sudo dnf install libsecret-devel
```

---

### 実行時エラー

#### 1. キーチェーンアクセスエラー (macOS)

**エラー**: `Keychain access denied`

**解決策**:

- macOSが初回アクセス許可を求めます
- "Allow" をクリックしてアクセスを許可
- "Always Allow" を選択すると次回以降聞かれません

#### 2. Secret Serviceエラー (Linux)

**エラー**: `Secret Service is not available`

**解決策**:

```bash
# GNOME Keyringの起動確認
ps aux | grep gnome-keyring

# 起動していない場合
gnome-keyring-daemon --start --components=secrets
```

または、代替としてKeePassXCなどのSecret Service互換アプリケーションを使用。

#### 3. 認証エラー (Bluesky)

**エラー**: `Authentication error: Invalid credentials`

**確認事項**:

- App Passwordを使用しているか（通常パスワードではダメ）
- ハンドルまたはメールアドレスが正しいか
- App Passwordが有効か（削除していないか）

**解決策**:

```bash
# 設定をやり直す
social-cli setup
```

---

## 開発者向けセットアップ

### 開発ツール

```bash
# フォーマッター
rustup component add rustfmt

# Linter
rustup component add clippy

# 実行
cargo fmt      # フォーマット
cargo clippy   # Lint
```

### 開発時の便利なコマンド

```bash
# ビルドと実行（デバッグモード）
cargo run -- setup
cargo run -- post -m "Test message"

# ウォッチモード（ファイル変更時に自動再ビルド）
cargo install cargo-watch
cargo watch -x 'run -- status'

# テスト（詳細出力）
cargo test -- --nocapture

# ベンチマーク
cargo bench
```

### VSCode設定（推奨）

`.vscode/settings.json`:

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

推奨拡張機能:

- rust-analyzer
- CodeLLDB (デバッグ用)
- Even Better TOML

---

## 環境変数

### デバッグログ

```bash
# ログレベル設定
export RUST_LOG=debug
social-cli post -m "Test"

# 特定モジュールのみ
export RUST_LOG=social_cli::platforms=trace
```

### カスタム設定パス（高度）

```bash
# 設定ファイルのパスを変更
export SOCIAL_CLI_CONFIG_DIR=/custom/path
```

---

## アンインストール

### 1. バイナリの削除

```bash
# cargo installでインストールした場合
cargo uninstall social-cli

# 手動削除
rm -f ~/.cargo/bin/social-cli
```

### 2. 設定ファイルの削除

```bash
# macOS/Linux
rm -rf ~/.config/social-cli

# Windows
rd /s /q "%APPDATA%\social-cli"
```

### 3. キーチェーンから認証情報を削除

#### macOS

1. "Keychain Access" アプリを開く
2. "social-cli" で検索
3. 該当エントリを削除

#### Linux

```bash
# Secret Serviceツールで削除
secret-tool clear service social-cli
```

#### Windows

1. "Credential Manager" を開く
2. "social-cli" を検索
3. 該当エントリを削除

---

## 次のステップ

セットアップが完了したら、以下を参照してください：

- [usage.md](usage.md) - 使用方法とコマンドリファレンス
- [api-integration.md](api-integration.md) - API統合の詳細
- [security.md](security.md) - セキュリティのベストプラクティス
