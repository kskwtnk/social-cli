# Social CLI

Rust製のマルチSNS投稿CLIツール。
Bluesky、X (Twitter)、Threadsに一斉投稿できます。

## クイックスタート

```bash
# 1. セットアップ
cp .env.example .env
# .envを編集して各SNSの認証情報を設定します。
# 詳細な手順はセットアップガイドを参照してください。

# 2. ビルド
cargo build

# 3. 投稿
cargo post -m "Hello from Rust!"  # 全SNSに一斉投稿
cargo post bluesky -m "Bluesky only"  # 個別SNSに投稿
```

## ドキュメント

詳細な情報については、以下のドキュメントを参照してください。

- **[セットアップガイド](docs/setup.md)**: 環境構築と認証情報の詳細な設定方法
- **[使用方法](docs/usage.md)**: コマンドリファレンス、使用例、トラブルシューティング
- **[アーキテクチャ](docs/architecture.md)**: システム設計、技術スタック、モジュール構成
- **[セキュリティ](docs/security.md)**: 認証情報の安全な管理方法

## ライセンス

MIT License
