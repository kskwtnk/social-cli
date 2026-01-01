# Social CLI

Rust製のマルチSNS投稿CLIツール（個人使用・MVP）

## 概要

複数のSNSに同時投稿できるコマンドラインツール。

**現在の対応**:
- ✅ Bluesky (Phase 1 完了)
- ✅ X (Twitter) (Phase 2 完了)
- ⏳ Threads (Phase 3)

**対象ユーザー**: 自分（個人使用）

---

## クイックスタート

```bash
# 1. 環境設定
cp .env.example .env
# .envを編集してBluesky/X認証情報を設定

# 2. ビルド
cargo build

# 3. Blueskyに投稿
cargo run -- bluesky -m "Hello from Rust!"

# または、Xに投稿
cargo run -- x -m "Hello from Rust!"
```

---

## 技術スタック

| カテゴリ | クレート | 用途 |
|---------|---------|------|
| CLI | clap | コマンドライン引数 |
| 非同期 | tokio | 非同期ランタイム |
| HTTP | reqwest | HTTP クライアント |
| Bluesky | atrium-api | Bluesky API クライアント |
| Bluesky | atrium-xrpc-client | AT Protocol XRPC通信 |
| X (Twitter) | reqwest-oauth1 | OAuth 1.0a署名 |
| 環境変数 | dotenvy | .env読み込み |
| エラー | anyhow | シンプルなエラー処理 |
| シリアライズ | serde / serde_json | JSON変換 |

**Rust Edition**: 2021（安定性優先）
**必要なRustバージョン**: 1.85以降（atrium-xrpc-client 0.5が必要）

---

## プロジェクト構造（MVP）

```
social-cli/
├── .env                     # 認証情報（git無視）
├── .env.example             # テンプレート
├── .gitignore
├── Cargo.toml
├── CLAUDE.md                # このファイル
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── bluesky.rs           # Bluesky API実装
│   └── x.rs                 # X (Twitter) API実装
└── docs/                    # 詳細ドキュメント
    ├── architecture.md
    ├── setup.md
    ├── usage.md
    ├── api-integration.md
    └── security.md
```

---

## コマンド

```bash
# Blueskyに投稿（開発モード）
cargo run -- bluesky -m "メッセージ"

# Xに投稿（開発モード）
cargo run -- x -m "メッセージ"

# リリースビルド
cargo build --release
./target/release/social-cli bluesky -m "メッセージ"
./target/release/social-cli x -m "メッセージ"

# ヘルプ
cargo run -- --help
```

**出力例（Bluesky）**:
```
✓ Posted to Bluesky successfully!
View your post: https://bsky.app/profile/user.bsky.social/post/abc123xyz
```

**出力例（X）**:
```
✓ Posted to X successfully!
View your tweet: https://x.com/i/web/status/1234567890123456789
```

---

## 環境設定（.env）

```bash
# Bluesky認証情報
BLUESKY_IDENTIFIER=user.bsky.social
BLUESKY_APP_PASSWORD=xxxx-xxxx-xxxx-xxxx

# X (Twitter) 認証情報
X_CONSUMER_KEY=your_consumer_key_here
X_CONSUMER_SECRET=your_consumer_secret_here
X_ACCESS_TOKEN=your_access_token_here
X_ACCESS_TOKEN_SECRET=your_access_token_secret_here
```

**Bluesky App Password取得方法**:
1. Bluesky → Settings → App Passwords
2. "Add App Password" → 名前入力（例: "social-cli"）
3. 生成されたパスワードを`.env`に貼り付け

**X API認証情報取得方法**:
1. [X Developer Portal](https://developer.x.com/) にアクセス
2. プロジェクトとアプリを作成
3. App settings → User authentication settings を設定
4. Keys and tokens → Consumer Keys と Authentication Tokens を生成
5. 4つのキーを`.env`に貼り付け

---

## 開発フェーズ

### Phase 1: Bluesky MVP ✅ 完了
- Bluesky投稿機能
- .env認証
- シンプルなCLI

### Phase 2: X対応 ✅ 完了
- X (Twitter) 投稿機能
- OAuth 1.0a認証
- 個別SNS選択コマンド

### Phase 3: 高度な機能（今後）
- Threads対応
- 複数SNS同時投稿
- 設定ファイル管理
- キーチェーン統合

---

## ドキュメント

詳細な情報は以下を参照：

- **[アーキテクチャ設計](docs/architecture.md)** - システム設計、モジュール構成
- **[セットアップガイド](docs/setup.md)** - 環境構築、トラブルシューティング
- **[使用方法](docs/usage.md)** - コマンドリファレンス、使用例
- **[API統合ガイド](docs/api-integration.md)** - Bluesky/Twitter/Threads API詳細
- **[セキュリティ](docs/security.md)** - 認証情報管理、ベストプラクティス

---

## トラブルシューティング

### ビルドエラー

**`edition2024` is required エラーが出る場合**:

```bash
# Rustを最新に更新（1.85以降が必要）
rustup update

# Rustのバージョン確認
rustc --version
```

**依存関係のエラーが出る場合**:

```bash
# 依存関係を再取得
cargo clean
rm -f Cargo.lock
cargo build
```

### 認証エラー

**Bluesky**:
- `.env`ファイルが存在するか確認
- App Password（通常パスワードではない）を使用
- Bluesky設定でApp Passwordが削除されていないか確認

**X (Twitter)**:
- X Developer Portalで User authentication settings が設定されているか確認
- App permissionsが「Read and Write」になっているか確認
- Access TokenとAccess Token Secretを **User authentication settings設定後に再生成**したか確認
- Free tierの上限（月500投稿）に達していないか確認

**.envファイルのパーミッション**:
```bash
chmod 600 .env
```

詳細は [docs/usage.md](docs/usage.md) を参照。

---

## ライセンス

MIT License
