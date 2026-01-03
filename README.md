# social-cli

Rust製のマルチSNS投稿CLIツール。
Bluesky、X (Twitter)、Threadsに一斉投稿できます。

## クイックスタート

```bash
# 1. セットアップ
cp .env.example .env
# .envを編集して各SNSの認証情報を設定します。

# 2. ビルド
cargo build

# 3. 投稿
cargo post -m "Hello from Rust!"
# 全SNSに一斉投稿

cargo post bluesky -m "Bluesky only"
# 個別SNSに投稿
```
