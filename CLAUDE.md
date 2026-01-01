# Social CLI

Rust製のマルチSNS投稿CLIツール（個人使用・MVP）

## 概要

複数のSNSに同時投稿できるコマンドラインツール。

**現在の対応**:
- ✅ Bluesky (Phase 1 MVP)
- ⏳ X (Twitter) (Phase 2)
- ⏳ Threads (Phase 3)

**対象ユーザー**: 自分（個人使用）

---

## クイックスタート

```bash
# 1. 環境設定
cp .env.example .env
# .envを編集してBluesky認証情報を設定

# 2. ビルド
cargo build

# 3. 投稿
cargo run -- post -m "Hello from Rust!"
```

---

## 技術スタック

| カテゴリ | クレート | 用途 |
|---------|---------|------|
| CLI | clap | コマンドライン引数 |
| 非同期 | tokio | 非同期ランタイム |
| HTTP | reqwest | HTTP クライアント（atriumが内部使用） |
| Bluesky | atrium-api | Bluesky API クライアント |
| Bluesky | atrium-xrpc-client | AT Protocol XRPC通信 |
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
│   └── bluesky.rs           # Bluesky API実装
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
# 投稿（開発モード）
cargo run -- post -m "メッセージ"

# 投稿（リリースビルド）
cargo build --release
./target/release/social-cli post -m "メッセージ"

# ヘルプ
cargo run -- --help
```

**出力例**:
```
✓ Posted successfully!
View your post: https://bsky.app/profile/user.bsky.social/post/abc123xyz
```

---

## 環境設定（.env）

```bash
# Bluesky認証情報
BLUESKY_IDENTIFIER=user.bsky.social
BLUESKY_APP_PASSWORD=xxxx-xxxx-xxxx-xxxx
```

**App Password取得方法**:
1. Bluesky → Settings → App Passwords
2. "Add App Password" → 名前入力（例: "social-cli"）
3. 生成されたパスワードを`.env`に貼り付け

---

## 開発フェーズ

### Phase 1: MVP (現在)
- Bluesky投稿のみ
- .env認証
- シンプルなCLI

### Phase 2: 拡張
- X (Twitter) 対応
- 複数SNS同時投稿

### Phase 3: 高度な機能
- Threads対応
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

- `.env`ファイルが存在するか確認
- App Password（通常パスワードではない）を使用
- Bluesky設定でApp Passwordが削除されていないか確認
- `.env`ファイルのパーミッションを確認

```bash
chmod 600 .env
```

詳細は [docs/usage.md](docs/usage.md) を参照。

---

## ライセンス

MIT License
