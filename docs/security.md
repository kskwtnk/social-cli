# セキュリティ考慮事項

## 概要

Social CLIは認証情報を扱うため、セキュリティが重要です。このドキュメントでは、`.env`ファイルを使用した認証情報管理のベストプラクティスを説明します。

---

## 目次

- [.envファイルの安全な管理](#envファイルの安全な管理)
- [インシデント対応](#インシデント対応)
- [依存関係のセキュリティ](#依存関係のセキュリティ)
- [参考リソース](#参考リソース)

---

## .envファイルの安全な管理

### 1. ファイル作成

```bash
# .env.example をコピー
cp .env.example .env

# パーミッションを設定（所有者のみ読み書き可）
chmod 600 .env
```

### 2. .gitignore への追加（必須）

`.gitignore` に以下が含まれていることを確認：

```gitignore
# 環境変数（認証情報を含む）
.env
```

### 3. .env ファイルの内容

```bash
# Bluesky認証情報
BLUESKY_IDENTIFIER=your-handle.bsky.social
BLUESKY_APP_PASSWORD=xxxx-xxxx-xxxx-xxxx

# X (Twitter) 認証情報
X_CONSUMER_KEY=your_consumer_key_here
X_CONSUMER_SECRET=your_consumer_secret_here
X_ACCESS_TOKEN=your_access_token_here
X_ACCESS_TOKEN_SECRET=your_access_token_secret_here

# Threads 認証情報
THREADS_USER_ID=your_numeric_user_id_here
THREADS_ACCESS_TOKEN=your_access_token_here
```

**重要**:
- **Bluesky**: 通常のパスワードではなく、App Passwordを使用（Settings → App Passwordsから生成）
- **X**: Developer Portalで4つのOAuth 1.0a認証情報を生成
- **Threads**: 数値User ID（ユーザー名ではない）とAccess Tokenを使用
- このファイルを絶対にGitにコミットしない
- すべての認証情報は機密情報として扱う

### 4. セキュリティチェックリスト

- [ ] `.env` ファイルが `.gitignore` に含まれている
- [ ] `.env` のパーミッションが `600` (rw-------)
- [ ] App Password を使用（通常パスワードは使わない）
- [ ] `.env` ファイルを共有・公開していない
- [ ] バックアップ時に `.env` を除外している

### 5. .env のリスクと対策

| リスク | 対策 |
|--------|------|
| ファイルシステムアクセスで読み取り可能 | パーミッション `600` に設定 |
| Git誤コミット | `.gitignore` に追加、pre-commit hook |
| バックアップで漏洩 | バックアップ対象から除外 |
| 共有PCでの使用 | 個人用マシンでのみ使用 |

### 6. App Passwordのベストプラクティス

**一意のApp Passwordを使用**:
- social-cli専用のApp Passwordを生成
- 他のアプリと同じパスワードを使い回さない
- メインパスワードは絶対に使用しない

**定期的なローテーション**:
- 3-6ヶ月ごとにApp Passwordを更新
- Bluesky Settings → App Passwords で古いパスワードを削除し、新しいパスワードを生成
- `.env`ファイルを更新

**不要なApp Passwordの削除**:
- 使用しなくなったApp Passwordは即座に削除
- Bluesky Settings → App Passwords → Delete

---

## インシデント対応

### 認証情報の漏洩が疑われる場合

**即座に実行**:

1. **すべての認証情報を無効化**
   - **Bluesky**: Settings → App Passwords → 該当パスワードを削除
   - **X**: Developer Portal → Keys and tokens → Regenerate
   - **Threads**: Meta for Developers → Access Tokenを再生成

2. **新しい認証情報を生成**
   - 各SNSで新しい認証情報を生成
   - `.env`ファイルを更新

3. **ログ確認**
   - 不正なアクセスがないか各SNSのWebインターフェースで確認
   - 不審な投稿やアクティビティをチェック

**調査**:
- どのように漏洩したか特定
- 影響範囲の確認
- 他のアカウントへの影響確認

### .envファイルをGitにコミットした場合

```bash
# 1. リポジトリ履歴から.envを削除
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch .env" \
  --prune-empty --tag-name-filter cat -- --all

# 2. リモートに強制プッシュ（注意！）
git push origin --force --all

# 3. すべての認証情報を即座に無効化・再生成
# Bluesky: Settings → App Passwords → 該当パスワードを削除
# X: Developer Portal → Keys and tokens → Regenerate
# Threads: Meta for Developers → Access Tokenを再生成

# 4. 新しい認証情報を生成して.envを更新
```

**重要**: 公開リポジトリの場合、すでに漏洩している可能性が高いです。即座にすべての認証情報を無効化してください。

---

## 依存関係のセキュリティ

### 定期的な監査

```bash
# cargo-auditのインストール
cargo install cargo-audit

# 脆弱性スキャン
cargo audit

# 自動修正（可能な場合）
cargo audit fix
```

### 依存関係の最小化

- 必要なクレートのみを使用
- 信頼できるメンテナーのクレートを選択
- 定期的な更新

```bash
# 依存関係の更新
cargo update

# 古いバージョンの確認
cargo outdated
```


## 参考リソース

### セキュリティガイドライン

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OAuth 2.0 Security Best Current Practice](https://datatracker.ietf.org/doc/html/draft-ietf-oauth-security-topics)

### 関連ツール

- `cargo-audit` - 脆弱性スキャン
- `cargo-deny` - ライセンスとセキュリティチェック
- `cargo-geiger` - unsafe コードの検出

---

## まとめ

セキュリティのベストプラクティス:

1. **`.env`ファイルを絶対にGitにコミットしない**
2. **App Passwordを使用（メインパスワードは使わない）**
3. **ファイルパーミッションを`600`に設定**
4. **定期的なApp Passwordのローテーション**
5. **不要なApp Passwordは即座に削除**

## 次のステップ

- [architecture.md](architecture.md) - システムアーキテクチャ
- [setup.md](setup.md) - セキュアなセットアップ手順
- [usage.md](usage.md) - 使用方法とコマンドリファレンス
