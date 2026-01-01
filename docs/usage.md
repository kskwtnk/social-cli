# 使用方法

## 実装状況

| 機能 | 状態 |
|------|------|
| Bluesky投稿 | ✅ 実装済み (Phase 1) |
| X (Twitter) 投稿 | ✅ 実装済み (Phase 2) |
| Threads投稿 | ✅ 実装済み (Phase 3) |
| 全SNS一斉投稿 | ✅ 実装済み (Phase 4) |
| 個別エラーハンドリング | ✅ 実装済み (Phase 4) |
| cargo postエイリアス | ✅ 実装済み (Phase 4) |
| 投稿URLの自動表示 | ✅ 実装済み |

---

## クイックスタート

```bash
# 1. 環境設定
cp .env.example .env
# .envを編集してBluesky/X/Threads認証情報を設定

# 2. ビルド
cargo build

# 3. 全SNSに一斉投稿
cargo post -m "Hello from Rust!"

# または、個別SNSに投稿
cargo post bluesky -m "Bluesky only"
cargo post x -m "X only"
cargo post threads -m "Threads only"
```

---

## コマンドリファレンス

### 全SNSに一斉投稿

サブコマンドを指定しない場合、全てのSNS（Bluesky、X、Threads）に同時投稿します。

```bash
# 開発モード
cargo post -m "メッセージ"

# リリースビルド
./target/release/social-cli -m "メッセージ"
```

**出力例**:
```
Posting to all platforms...

✓ Posted to Bluesky successfully!
  https://bsky.app/profile/user.bsky.social/post/abc123xyz
✓ Posted to X successfully!
  https://x.com/i/web/status/1234567890123456789
✓ Posted to Threads successfully!
  https://www.threads.com/@username/post/abc123xyz

Posting complete!
```

**特徴**:
- 1つのSNSが失敗しても他のSNSへの投稿は継続
- 各SNSの投稿URLが個別に表示される
- エラーが発生した場合も詳細が表示される

---

### 個別SNSに投稿

特定のSNSのみに投稿したい場合は、サブコマンドを指定します。

#### Blueskyのみに投稿

```bash
cargo post bluesky -m "Bluesky専用の投稿"
```

**出力例**:
```
✓ Posted to Bluesky successfully!
View your post: https://bsky.app/profile/user.bsky.social/post/abc123xyz
```

#### X (Twitter) のみに投稿

```bash
cargo post x -m "X専用の投稿"
```

**出力例**:
```
✓ Posted to X successfully!
View your tweet: https://x.com/i/web/status/1234567890123456789
```

#### Threadsのみに投稿

```bash
cargo post threads -m "Threads専用の投稿"
```

**出力例**:
```
✓ Posted to Threads successfully!
View your thread: https://www.threads.com/@username/post/abc123xyz
```

---

## オプション

| オプション | 短縮形 | 説明 | 必須 |
|-----------|--------|------|------|
| `--message` | `-m` | 投稿メッセージ | ✅ |

### サブコマンド

| サブコマンド | 説明 |
|-------------|------|
| (無し) | 全SNSに一斉投稿 |
| `bluesky` | Blueskyのみに投稿 |
| `x` | X (Twitter) のみに投稿 |
| `threads` | Threadsのみに投稿 |

---

## 使用例

### 1. 全SNSに同じメッセージを投稿

```bash
cargo post -m "新機能をリリースしました！"
```

### 2. 複数行メッセージ

```bash
cargo post -m "これは複数行の
メッセージです。
改行も含められます。"
```

### 3. リリースビルド使用

```bash
# ビルド
cargo build --release

# 全SNSに投稿
./target/release/social-cli -m "本番環境からの投稿"

# 個別SNSに投稿
./target/release/social-cli bluesky -m "Blueskyのみ"
```

### 4. シェルスクリプトでの自動投稿

```bash
#!/bin/bash
# daily-update.sh

MESSAGE="$(date '+%Y年%m月%d日')の定期投稿です"
cargo post -m "$MESSAGE"
```

### 5. ファイルから投稿

```bash
# message.txtの内容を投稿
cargo post -m "$(cat message.txt)"
```

---

## エラーハンドリング

### 全SNS投稿時のエラー

一部のSNSで失敗しても、他のSNSへの投稿は継続されます。

**出力例（一部失敗）**:
```
Posting to all platforms...

✓ Posted to Bluesky successfully!
  https://bsky.app/profile/user.bsky.social/post/abc123xyz
✗ Failed to post to X: Network error: Connection timeout
✓ Posted to Threads successfully!
  https://www.threads.com/@username/post/abc123xyz

Posting complete!
```

### 個別SNS投稿時のエラー

エラーが発生すると、詳細なエラーメッセージが表示されます。

**出力例（認証エラー）**:
```
Error: BLUESKY_IDENTIFIER not set in .env file
```

---

## 文字数制限

各プラットフォームの文字数制限:

| プラットフォーム | 最大文字数 |
|----------------|-----------|
| Bluesky | 300文字 |
| X (Twitter) | 280文字（無料）/ 4,000文字（Premium） |
| Threads | 500文字 |

**注意**: 制限を超える場合は各プラットフォームでエラーになります。

---

## トラブルシューティング

### 実行時エラー

**認証エラー**:
`Error: ... not set in .env file` や認証失敗に関するエラーが表示された場合、`.env`ファイルの認証情報が正しく設定されていない可能性があります。
詳細な設定方法とトラブルシューティングについては、[セットアップガイド](setup.md)のトラブルシューティングのセクションを参照してください。

**ネットワークエラー**:
`Error: Network error: Connection timeout` のようなエラーが表示された場合は、以下を確認してください。
- インターネット接続
- 各SNSプラットフォームの障害情報（ステータスページなど）
- ファイアウォール等のネットワーク設定

**文字数制限超過**:
各SNSの文字数制限を超えて投稿しようとするとエラーになります。メッセージの長さを確認してください。

### ビルドエラー

**`edition2024` is required エラー**:
このエラーは、Rustのバージョンが古い場合に発生します。以下のコマンドでRustを更新してください。
```bash
# Rustを最新に更新（1.85以降が必要）
rustup update

# Rustのバージョン確認
rustc --version
```

**依存関係のエラー**:
ビルド時に依存関係に関する問題が発生した場合は、以下のコマンドでクリーンアップと再ビルドを試してください。
```bash
# 依存関係を再取得
cargo clean
rm -f Cargo.lock # 注意: 依存バージョンが更新される可能性があります
cargo build
```

---

## ヘルプの表示

```bash
# 全体のヘルプ
cargo post --help

# サブコマンドのヘルプ
cargo post bluesky --help
```

---

## 次のステップ

- [setup.md](setup.md) - 環境構築の詳細ガイド
- [security.md](security.md) - セキュリティのベストプラクティス
- [architecture.md](architecture.md) - システム設計とモジュール構成
