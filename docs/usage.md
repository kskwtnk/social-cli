# 使用方法

**重要**: このドキュメントは、**完成後の使い方を示す設計書**です。現在（Phase 1 MVP）では一部の機能のみ実装予定で、多くの機能はPhase 2以降に実装されます。

## 実装状況

| 機能 | Phase 1 (MVP) | Phase 2以降 |
|------|--------------|------------|
| Bluesky投稿 (`post -m`) | ✅ 実装予定 | - |
| X投稿 | - | ⏳ 計画中 |
| Threads投稿 | - | ⏳ 計画中 |
| `setup` コマンド | - | ⏳ 計画中 |
| `status` コマンド | - | ⏳ 計画中 |
| 設定ファイル管理 | - | ⏳ 計画中 |

**MVP (Phase 1)の使い方**:
```bash
# .env ファイルを設定（手動）
cp .env.example .env
nano .env  # 認証情報を入力

# Blueskyに投稿
cargo run -- post -m "Hello, world!"
```

以下は、完成後の理想的な使い方を示しています。

---

## コマンドリファレンス（将来の完成形）

Social CLIは3つの主要コマンドで構成される予定です。

---

## `social-cli setup` (Phase 2以降)

**注意**: このコマンドはPhase 2以降で実装予定です。MVP (Phase 1)では `.env` ファイルを手動編集します。

初期設定を行うコマンドです。

### 基本的な使い方

```bash
social-cli setup
```

### 対話形式でのセットアップ

```
Social CLI Setup
================

Bluesky Setup
-------------
Bluesky Handle or Email: your-handle.bsky.social
App Password: ****************
✓ Bluesky configured successfully

✓ Configuration saved successfully!
```

### 特定のプラットフォームのみ設定

```bash
# Blueskyのみ
social-cli setup --platform bluesky

# Twitter/Xのみ（Phase 2）
social-cli setup --platform twitter
```

### セットアップの流れ

1. **ハンドル/メールアドレス入力**
   - Blueskyのハンドル（例: `user.bsky.social`）
   - またはメールアドレス

2. **App Password入力**
   - Bluesky設定で生成したApp Passwordを入力
   - 入力中は非表示（セキュリティのため）

3. **認証確認**
   - 入力した認証情報でBlueskyに接続テスト
   - 成功したら設定を保存

4. **保存先**
   - 設定: `~/.config/social-cli/config.toml`
   - パスワード: OSキーチェーン

### エラー時の対処

#### 認証エラー

```
✗ Authentication failed: Invalid credentials
```

**対処法**:
- App Passwordが正しいか確認
- ハンドルが正しいか確認
- App Passwordが削除されていないか確認

#### ネットワークエラー

```
✗ Network error: Connection timeout
```

**対処法**:
- インターネット接続を確認
- Blueskyがダウンしていないか確認

---

## `social-cli post`

SNSに投稿するコマンドです。

**MVP (Phase 1)での実装**:
- ✅ Blueskyへの投稿のみサポート
- ✅ `-m` オプションでメッセージ指定
- ❌ `-p` オプション（プラットフォーム選択）は未実装

### 基本的な使い方

```bash
# MVP (Phase 1): Blueskyに投稿
cargo run -- post -m "Hello, world!"

# Phase 2以降: 全SNSに投稿
social-cli post -m "Hello, world!"
```

### オプション

| オプション | 短縮形 | 説明 | デフォルト | 実装状況 |
|-----------|--------|------|-----------|---------|
| `--message` | `-m` | 投稿メッセージ（必須） | - | ✅ Phase 1 |
| `--platform` | `-p` | 投稿先プラットフォーム | `all` | ⏳ Phase 2 |

### 使用例

#### MVP (Phase 1): Blueskyに投稿

```bash
# 開発中
cargo run -- post -m "新機能をリリースしました！"

# ビルド後
./target/release/social-cli post -m "新機能をリリースしました！"
```

**実装予定の出力**:

```
Posted successfully!
```

#### Phase 2以降: 全SNSに投稿

```bash
social-cli post -m "新機能をリリースしました！"
```

出力例:

```
Posting to 3 platform(s)...

✓ Bluesky - Posted successfully
  URL: https://bsky.app/profile/user.bsky.social/post/abc123

✓ X - Posted successfully
  URL: https://twitter.com/username/status/123456789

✓ Threads - Posted successfully
  URL: https://threads.net/@username/post/xyz789

Summary: 3/3 posts succeeded
```

#### 特定のプラットフォームのみに投稿（Phase 2以降）

```bash
# Blueskyのみ
social-cli post -p bluesky -m "Bluesky専用の投稿"

# Twitter/Xのみ
social-cli post -p twitter -m "X専用の投稿"
```

#### 長いメッセージ

```bash
social-cli post -m "これは長いメッセージです。
複数行にわたる投稿も可能です。
改行も含められます。"
```

### 投稿結果の見方

#### 成功した場合

```
✓ Bluesky - Posted successfully
  URL: https://bsky.app/profile/user.bsky.social/post/abc123
```

- ✓: 成功マーク
- URL: 投稿へのリンク（ブラウザで開けます）

#### 失敗した場合

```
✗ Bluesky - Failed: Network error: Connection timeout
```

- ✗: 失敗マーク
- エラーメッセージが表示されます

#### 部分的に失敗した場合

```
Posting to 2 platform(s)...

✓ Bluesky - Posted successfully
  URL: https://bsky.app/profile/user.bsky.social/post/abc123

✗ Twitter - Failed: Authentication error: Invalid token

Summary: 1/2 posts succeeded
```

- 成功したプラットフォームと失敗したプラットフォームが個別に表示
- サマリーで全体の結果を確認

### 文字数制限

各プラットフォームの文字数制限:

| プラットフォーム | 最大文字数 |
|----------------|-----------|
| Bluesky | 300文字 |
| Twitter/X | 280文字（無料）/ 4,000文字（Premium） |
| Threads | 500文字 |

**注意**: 制限を超える場合はエラーになります。

### エラー時の対処

#### 未設定エラー

```
✗ Error: No platforms configured. Run 'social-cli setup' first.
```

**対処法**: `social-cli setup`で設定を行う

#### 認証エラー

```
✗ Bluesky - Failed: Authentication error: Session expired
```

**対処法**: `social-cli setup`で再度認証

#### ネットワークエラー

```
✗ Bluesky - Failed: Network error: Connection refused
```

**対処法**:
- インターネット接続を確認
- しばらく待ってから再試行

---

## `social-cli status` (Phase 2以降)

**注意**: このコマンドはPhase 2以降で実装予定です。MVP (Phase 1)では未実装です。

現在の設定状態を確認するコマンドです。

### 基本的な使い方

```bash
social-cli status
```

### 出力例

```
Social CLI Configuration Status
================================

Bluesky:
  Status: ✓ Enabled
  Handle: user.bsky.social

Twitter:
  Status: ✗ Not configured

Threads:
  Status: ✗ Not configured
```

### 情報の見方

#### 設定済みのプラットフォーム

```
Bluesky:
  Status: ✓ Enabled
  Handle: user.bsky.social
```

- ✓ Enabled: 設定済みで利用可能
- Handle: 登録されているハンドル

#### 未設定のプラットフォーム

```
Twitter:
  Status: ✗ Not configured
```

- ✗ Not configured: 未設定

#### 無効化されているプラットフォーム

```
Twitter:
  Status: ✗ Disabled
  API Tier: Basic
```

- ✗ Disabled: 設定されているが無効化されている

### 使用場面

- セットアップ後の確認
- トラブルシューティング時の設定確認
- 複数アカウント管理時の確認（Phase 3以降）

---

## 実践的な使用例

### 1. 初回セットアップから投稿まで

```bash
# 1. セットアップ
$ social-cli setup
Bluesky Handle or Email: user.bsky.social
App Password: ****************
✓ Configuration saved successfully!

# 2. 設定確認
$ social-cli status
Social CLI Configuration Status
================================
Bluesky:
  Status: ✓ Enabled
  Handle: user.bsky.social

# 3. 投稿
$ social-cli post -m "Hello from social-cli!"
Posting to 1 platform(s)...
✓ Bluesky - Posted successfully
  URL: https://bsky.app/profile/user.bsky.social/post/xyz789
Summary: 1/1 posts succeeded
```

### 2. シェルスクリプトでの自動投稿

```bash
#!/bin/bash
# daily-update.sh

MESSAGE="$(date '+%Y年%m月%d日')の定期投稿です"
social-cli post -m "$MESSAGE"
```

### 3. エイリアス設定

`~/.bashrc` または `~/.zshrc`:

```bash
# 短縮エイリアス
alias sp='social-cli post -m'
alias ss='social-cli status'

# 使用例
sp "投稿内容"
```

### 4. ファイルから投稿

```bash
# message.txtの内容を投稿
social-cli post -m "$(cat message.txt)"
```

### 5. パイプラインでの使用

```bash
# 他のコマンドの出力を投稿
echo "ビルドが完了しました" | xargs -I {} social-cli post -m "{}"

# git logから最新コミットを投稿
git log -1 --pretty=format:"%s" | xargs -I {} social-cli post -m "最新コミット: {}"
```

---

## 高度な使い方

### 環境変数での設定

#### デバッグモード

```bash
# 詳細なログ出力
RUST_LOG=debug social-cli post -m "Test"
```

#### カスタム設定パス

```bash
# 異なる設定ファイルを使用
SOCIAL_CLI_CONFIG_DIR=/path/to/config social-cli status
```

### JSON出力（将来実装予定）

```bash
# 機械可読形式で出力
social-cli post -m "Test" --output json
```

### ドライラン（将来実装予定）

```bash
# 実際には投稿せず、動作確認のみ
social-cli post -m "Test" --dry-run
```

---

## トラブルシューティング

### よくある問題と解決策

#### 1. "command not found: social-cli"

**原因**: パスが通っていない

**解決策**:

```bash
# cargo installした場合
export PATH="$HOME/.cargo/bin:$PATH"

# またはフルパスで実行
~/Repositories/social-cli/target/release/social-cli
```

#### 2. "No platforms configured"

**原因**: セットアップが完了していない

**解決策**:

```bash
social-cli setup
```

#### 3. "Authentication error: Invalid credentials"

**原因**:
- App Passwordが間違っている
- App Passwordが削除された
- セッションが期限切れ

**解決策**:

```bash
# 再セットアップ
social-cli setup
```

#### 4. "Post failed: Message too long"

**原因**: 文字数制限を超えている

**解決策**:

```bash
# メッセージを短くする
# または、プラットフォームごとに分ける（Phase 2以降）
social-cli post -p bluesky -m "短いメッセージ（300文字以内）"
```

#### 5. キーチェーンアクセスエラー (macOS)

**原因**: キーチェーンへのアクセス許可がない

**解決策**:

- ダイアログで "Always Allow" を選択
- または、Keychain Accessアプリで手動で許可

#### 6. Secret Serviceエラー (Linux)

**原因**: GNOME Keyringが起動していない

**解決策**:

```bash
# GNOME Keyringを起動
gnome-keyring-daemon --start --components=secrets

# または、環境変数を設定
eval $(gnome-keyring-daemon --start)
export SSH_AUTH_SOCK
```

### デバッグ手順

```bash
# 1. 設定確認
social-cli status

# 2. 詳細ログで実行
RUST_LOG=debug social-cli post -m "Test"

# 3. 設定ファイルの確認
cat ~/.config/social-cli/config.toml

# 4. 再セットアップ
social-cli setup
```

---

## ベストプラクティス

### セキュリティ

1. **App Passwordを共有しない**
   - 各デバイス/アプリごとに個別のApp Passwordを生成
   - 不要になったら削除

2. **設定ファイルのバックアップ**
   - キーチェーンの情報は含まれないため安全
   ```bash
   cp ~/.config/social-cli/config.toml ~/backup/
   ```

3. **定期的なApp Passwordの更新**
   - セキュリティのため、3-6ヶ月ごとに更新

### 効率的な使い方

1. **エイリアスの活用**
   ```bash
   alias post='social-cli post -m'
   ```

2. **テンプレートの使用**
   ```bash
   # template.txt
   今日の進捗:
   - {task1}
   - {task2}

   # 使用時
   cat template.txt | sed "s/{task1}/実装完了/g" | \
     sed "s/{task2}/テスト中/g" | \
     xargs -I {} social-cli post -m "{}"
   ```

3. **シェルスクリプトでの自動化**
   - 定期投稿
   - ビルド成功/失敗の通知
   - デプロイ完了通知

---

## FAQ

### Q: 投稿を削除できますか？

A: Phase 1では未対応です。削除は各プラットフォームのWebインターフェースから行ってください。

### Q: 画像を投稿できますか？

A: Phase 1では未対応です。Phase 3以降で実装予定です。

### Q: 複数アカウントを管理できますか？

A: Phase 1では1アカウントのみ。Phase 3で複数アカウント対応予定です。

### Q: 投稿の予約はできますか？

A: 未対応です。cronやタスクスケジューラーと組み合わせて実現可能です：

```bash
# crontabで毎日9時に投稿
0 9 * * * /usr/local/bin/social-cli post -m "おはようございます"
```

### Q: Windows PowerShellで使えますか？

A: はい、使えます：

```powershell
social-cli post -m "Test from PowerShell"
```

### Q: 投稿履歴を確認できますか？

A: Phase 1では未対応です。Phase 3で実装予定です。

---

## 次のステップ

- [api-integration.md](api-integration.md) - API統合の詳細
- [security.md](security.md) - セキュリティのベストプラクティス
- [setup.md](setup.md) - セットアップガイド
