# セットアップガイド

**注意**: このドキュメントは将来の拡張を見据えた内容も含まれています。MVP（Phase 1）では macOS 環境での `.env` ベースの簡易セットアップのみを想定しています。

## 開発環境構築（MVP - macOS）

### 前提条件

- **Rust**: 1.70以上（推奨: 最新安定版）
- **OS**: macOS（MVP Phase 1の対象）
- **その他**: インターネット接続、Blueskyアカウント

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

## macOS環境設定（MVP）

MVPではmacOSを対象としており、特別な追加セットアップは不要です。

### .env ファイルの準備

```bash
# プロジェクトディレクトリで実行
cd /path/to/social-cli

# .env.example から .env を作成
cp .env.example .env

# パーミッションを設定（重要）
chmod 600 .env

# .env を編集してBluesky認証情報を設定
nano .env  # または vi, vscode など
```

---

## Phase 2以降: 他OS対応（参考）

**注意**: 以下は将来の拡張用の参考情報です。MVP Phase 1では不要です。

<details>
<summary>Linux環境での追加セットアップ（Phase 2以降）</summary>

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

</details>

<details>
<summary>Windows環境での追加セットアップ（Phase 2以降）</summary>

Windows Credential Managerが自動的に使用されます。追加セットアップは不要です。

</details>

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

#### API Tierの確認（2026年1月時点）

X APIは2023年に料金体系が変更されました：

- **Free**: $0/月 - 月間100投稿まで可能（個人MVP用途には十分）
- **Basic**: $100-200/月 - 月間3,000投稿
- **Pro**: $5,000/月 - 月間300,000投稿、高度な機能

#### APIキーの取得

1. [X Developer Portal](https://developer.x.com/) にアクセス
2. "Create Project" → "Create App"
3. API Key、API Secret、Access Token、Access Token Secretを取得
4. 必要なパーミッション: **Read and Write**

**注意**: Free Tierでもテスト・個人使用には十分ですが、レート制限が厳しいため実用的な自動投稿には不向きです。

---

### Threads - Phase 3

#### Meta for Developers

1. [Meta for Developers](https://developers.facebook.com/) でアカウント作成
2. アプリケーション登録
3. Threadsビジネスアカウントとして認証
4. `threads_basic` および `threads_content_publish` パーミッション取得
5. アクセストークン取得

**制限事項**:
- ビジネスアカウント認証が必須（個人アカウントでは不可）
- 24時間で最大250投稿
- ハッシュタグは1つのみ、最大500文字
- 画像・動画は最大10枚/本まで

---

## 初回セットアップ（MVP）

### 1. .env ファイルの作成

```bash
# プロジェクトディレクトリに移動
cd /path/to/social-cli

# .env.example をコピー
cp .env.example .env

# パーミッション設定（重要）
chmod 600 .env
```

### 2. .env ファイルの編集

```bash
# エディタで .env を開く
nano .env  # または vim, code など
```

以下のように編集：

```bash
# Bluesky認証情報
BLUESKY_IDENTIFIER=your-handle.bsky.social
BLUESKY_APP_PASSWORD=xxxx-xxxx-xxxx-xxxx
```

**重要**:
- `BLUESKY_IDENTIFIER`: Blueskyのハンドル（例: `user.bsky.social`）またはメールアドレス
- `BLUESKY_APP_PASSWORD`: Bluesky Settings → App Passwords で生成したApp Password

### 3. ビルドと実行

```bash
# ビルド
cargo build --release

# 投稿テスト
cargo run --release -- post -m "Hello from social-cli!"
```

### 4. 動作確認

正常に投稿されると以下のような出力が表示されます：

```
Posted successfully!
```

Blueskyアプリまたはウェブで投稿を確認できます。

---

## Phase 2以降: 設定管理機能（参考）

**注意**: 以下は将来の拡張用の参考情報です。MVP Phase 1では実装されていません。

<details>
<summary>setupコマンド・statusコマンド（Phase 2以降）</summary>

### setupコマンド

```bash
social-cli setup
```

対話形式で認証情報を設定：

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

### statusコマンド

```bash
social-cli status
```

現在の設定状態を確認：

```
Social CLI Configuration Status
================================

Bluesky:
  Status: ✓ Enabled
  Handle: your-handle.bsky.social

Twitter:
  Status: ✗ Not configured
```

</details>

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
