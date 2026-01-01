# セットアップガイド

このガイドでは、Social CLIの環境構築から初回投稿までの手順を説明します。

---

## 前提条件

- **Rust**: 1.85以降（atrium-xrpc-client 0.5が必要）
- **OS**: macOS / Linux / Windows
- **その他**: インターネット接続、Bluesky/X/Threadsアカウント

---

## Rustのインストール

### 新規インストール

```bash
# rustupのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 環境変数を反映
source $HOME/.cargo/env

# バージョン確認（1.85以降であることを確認）
rustc --version
cargo --version
```

### 既存環境のアップデート

```bash
rustup update
rustc --version  # 1.85以降であることを確認
```

---

## プロジェクトのセットアップ

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

### 3. .envファイルの作成

```bash
# .env.exampleから.envを作成
cp .env.example .env

# パーミッションを設定（重要）
chmod 600 .env
```

### 4. 認証情報の設定

`.env`ファイルを編集して、各SNSの認証情報を設定します。

```bash
# エディタで.envを開く
nano .env
# または
code .env
```

---

## Bluesky認証情報の取得

### 1. App Passwordの生成

1. Blueskyアプリ/Webを開く
2. Settings → App Passwords に移動
3. "Add App Password" をクリック
4. 名前を入力（例: "social-cli"）
5. 生成されたパスワードをコピー（`xxxx-xxxx-xxxx-xxxx`形式）

### 2. .envに設定

```bash
BLUESKY_IDENTIFIER=your-handle.bsky.social
BLUESKY_APP_PASSWORD=xxxx-xxxx-xxxx-xxxx
```

**注意**:
- App Password は通常のパスワードではありません
- 生成時に一度だけ表示されるので、必ずコピーしてください
- App Password は削除すると無効になります

---

## X (Twitter) 認証情報の取得

### 1. Developer Portalでアプリを作成

1. [X Developer Portal](https://developer.x.com/) にアクセス
2. "Create Project" → プロジェクト名を入力
3. "Create App" → アプリ名を入力

### 2. User authentication settingsを設定

1. アプリの設定ページで "Set up" (User authentication settings) をクリック
2. App permissions: **"Read and Write"** を選択
3. Type of App: "Web App, Automated App or Bot" を選択
4. Callback URL: `http://127.0.0.1` （必須だが使用しない）
5. Website URL: 任意のURL
6. "Save" をクリック

### 3. API KeysとTokensを生成

1. "Keys and tokens" タブに移動
2. **Consumer Keys** セクション:
   - API Key (`X_CONSUMER_KEY`)
   - API Key Secret (`X_CONSUMER_SECRET`)
   をコピー
3. **Authentication Tokens** セクション:
   - "Generate" をクリック
   - Access Token (`X_ACCESS_TOKEN`)
   - Access Token Secret (`X_ACCESS_TOKEN_SECRET`)
   をコピー

**重要**: User authentication settingsを設定**後に**Access TokenとAccess Token Secretを再生成してください。

### 4. .envに設定

```bash
X_CONSUMER_KEY=your_consumer_key_here
X_CONSUMER_SECRET=your_consumer_secret_here
X_ACCESS_TOKEN=your_access_token_here
X_ACCESS_TOKEN_SECRET=your_access_token_secret_here
```

---

## Threads認証情報の取得

### 1. Meta Developer Portalでアプリを作成

1. [Meta for Developers](https://developers.facebook.com/) にアクセス
2. 「アプリを作成」をクリック
3. アプリタイプ選択: 「その他」または「ビジネス」を選択
4. アプリ名とメールアドレスを入力
5. ビジネスポートフォリオ選択: 「現時点ではビジネスポートフォリオをリンクしない」を選択

### 2. Threads API製品を追加

1. 左メニューの「製品を追加」から「Threads」を探す
2. 「設定」をクリック

### 3. Access Tokenを生成

1. Threads製品の設定画面でAccess Tokenを生成
2. 必要な権限: `threads_basic`, `threads_content_publish`
3. 生成されたAccess Tokenをコピー

### 4. User IDを取得

User IDは数値のID（ユーザー名ではない）が必要です。

```bash
# YOUR_ACCESS_TOKENは手順3でコピーしたトークン
curl "https://graph.threads.net/v1.0/me?fields=id,username&access_token=YOUR_ACCESS_TOKEN"
```

レスポンス例:
```json
{"id":"33034957766151404","username":"kskwtnk"}
```

**`id`フィールドの値（数値）をコピー**してください。

### 5. .envに設定

```bash
THREADS_USER_ID=33034957766151404  # 数値ID（ユーザー名ではない）
THREADS_ACCESS_TOKEN=your_access_token_here
```

---

## 動作確認

### 1. 個別SNSでテスト

まず、各SNSに個別に投稿してテストします。

```bash
# Blueskyに投稿
cargo post bluesky -m "Test from social-cli!"

# Xに投稿
cargo post x -m "Test from social-cli!"

# Threadsに投稿
cargo post threads -m "Test from social-cli!"
```

### 2. 全SNS一斉投稿でテスト

すべてのSNSの設定が完了したら、一斉投稿をテストします。

```bash
cargo post -m "Hello from all platforms!"
```

**期待される出力**:
```
Posting to all platforms...

✓ Posted to Bluesky successfully!
  https://bsky.app/profile/your-handle.bsky.social/post/abc123
✓ Posted to X successfully!
  https://x.com/i/web/status/1234567890123456789
✓ Posted to Threads successfully!
  https://www.threads.com/@your_username/post/abc123

Posting complete!
```

---

## トラブルシューティング

### Rustのバージョンエラー

**エラー**: `edition2024 is required`

**解決策**:
```bash
rustup update
rustc --version  # 1.85以降であることを確認
cargo clean
cargo build
```

### Bluesky認証エラー

**エラー**: `BLUESKY_IDENTIFIER not set in .env file`

**解決策**:
- `.env`ファイルが存在するか確認
- `BLUESKY_IDENTIFIER`と`BLUESKY_APP_PASSWORD`が設定されているか確認
- App Password（通常パスワードではない）を使用しているか確認
- Bluesky設定でApp Passwordが削除されていないか確認

### X認証エラー

**エラー**: `X_CONSUMER_KEY not set in .env file`

**解決策**:
- 4つのキーすべてが設定されているか確認:
  - `X_CONSUMER_KEY`
  - `X_CONSUMER_SECRET`
  - `X_ACCESS_TOKEN`
  - `X_ACCESS_TOKEN_SECRET`
- X Developer Portalで User authentication settings が設定されているか確認
- App permissionsが「Read and Write」になっているか確認
- Access TokenとAccess Token Secretを**User authentication settings設定後に再生成**したか確認

### Threads認証エラー

**エラー**: `THREADS_USER_ID not set in .env file`

**解決策**:
- `THREADS_USER_ID`が数値ID（ユーザー名ではない）で設定されているか確認
- `THREADS_ACCESS_TOKEN`が設定されているか確認
- User IDを再取得:
  ```bash
  curl "https://graph.threads.net/v1.0/me?fields=id&access_token=YOUR_TOKEN"
  ```

### .envファイルのパーミッションエラー

**問題**: セキュリティ上、`.env`ファイルは所有者のみが読み書きできるべきです。

**解決策**:
```bash
chmod 600 .env
```

---

## リリースビルド

本番環境や実際の使用では、リリースビルドを使用することを推奨します。

```bash
# リリースビルド
cargo build --release

# バイナリの場所
ls -lh target/release/social-cli

# 使用方法
./target/release/social-cli -m "Production post"
```

**リリースビルドの利点**:
- 実行速度が速い
- バイナリサイズが小さい
- 最適化されたコード

---

## 次のステップ

- [usage.md](usage.md) - 詳細な使用方法とコマンドリファレンス
- [security.md](security.md) - セキュリティのベストプラクティス
- [architecture.md](architecture.md) - システム設計とアーキテクチャ
