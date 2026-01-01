# セキュリティ考慮事項

## 概要

Social CLIは認証情報を扱うため、セキュリティが最優先事項です。このドキュメントでは、実装および使用時のセキュリティベストプラクティスを説明します。

---

## 目次

- [認証情報管理](#認証情報管理)
- [設定ファイルのセキュリティ](#設定ファイルのセキュリティ)
- [ネットワークセキュリティ](#ネットワークセキュリティ)
- [ログとデバッグ](#ログとデバッグ)
- [脅威モデル](#脅威モデル)
- [セキュリティチェックリスト](#セキュリティチェックリスト)

---

## 認証情報管理

### OSキーチェーンの使用

Social CLIは、パスワードやトークンを**OSキーチェーン**に暗号化保存します。

#### サポートされるバックエンド

| OS | バックエンド | 説明 |
|----|------------|------|
| macOS | Keychain | システム標準のKeychain Access |
| Windows | Credential Manager | Windows Credential Manager |
| Linux | Secret Service | GNOME Keyring / KWallet 等 |

#### 実装

```rust
use keyring::Entry;

const SERVICE_NAME: &str = "social-cli";

pub fn save_password(key: &str, password: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.set_password(password)?;
    Ok(())
}

pub fn get_password(key: &str) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    Ok(entry.get_password()?)
}
```

### なぜキーチェーンを使うのか

#### ❌ 平文保存の問題

```toml
# config.toml (悪い例 - 絶対にしない)
[bluesky]
identifier = "user.bsky.social"
password = "my-secret-password"  # 危険！
```

**問題点**:
- ファイルシステムアクセスで簡単に読み取り可能
- バックアップやクラウド同期で漏洩リスク
- Git等に誤ってコミットされる可能性

#### ✅ キーチェーン保存

```toml
# config.toml (良い例)
[bluesky]
enabled = true
identifier = "user.bsky.social"
# password はキーチェーンに保存
```

**利点**:
- OS標準の暗号化
- マスターパスワードで保護（macOS等）
- アクセス制御
- 監査ログ

### App Passwordのベストプラクティス

#### 1. 一意のApp Passwordを使用

```
✅ 良い例:
- social-cli用: abcd-efgh-ijkl-mnop
- モバイルアプリ用: wxyz-1234-5678-90ab
- ブラウザ拡張用: qwer-tyui-opas-dfgh

❌ 悪い例:
- すべてのアプリで同じApp Passwordを使用
- メインパスワードを使用
```

#### 2. 定期的なローテーション

```bash
# 3-6ヶ月ごとに更新
# 1. 新しいApp Passwordを生成
# 2. social-cli setup で更新
# 3. 古いApp Passwordを削除
```

#### 3. 不要なApp Passwordの削除

```
使用しなくなったApp Passwordは即座に削除:
- Bluesky Settings → App Passwords → Delete
```

---

## 設定ファイルのセキュリティ

### ファイルパーミッション

設定ファイルは**所有者のみ読み書き可能**に設定します。

```rust
use std::os::unix::fs::PermissionsExt;

pub fn save_config_securely(path: &Path, content: &str) -> Result<()> {
    // ファイル書き込み
    std::fs::write(path, content)?;

    // パーミッション設定 (Unix系)
    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o600); // rw-------
        std::fs::set_permissions(path, perms)?;
    }

    Ok(())
}
```

#### パーミッション確認

```bash
# Unix/macOS/Linux
ls -la ~/.config/social-cli/config.toml
# -rw------- 1 user user 256 Jan 1 12:00 config.toml
#  ^^^^^^^^^
#  所有者のみ読み書き可能
```

### 設定ファイルの内容

#### ✅ 含めて良いもの

- プラットフォーム有効/無効フラグ
- ユーザーID、ハンドル、メールアドレス（公開情報）
- API Tier情報
- 一般的な設定オプション

#### ❌ 含めてはいけないもの

- パスワード
- App Password
- APIキー、APIシークレット
- アクセストークン
- リフレッシュトークン

### バックアップ時の注意

```bash
# 設定ファイルのバックアップは安全
# （認証情報が含まれないため）
cp ~/.config/social-cli/config.toml ~/backup/

# ただし、キーチェーンデータは別途バックアップ
# OS標準のバックアップ機能を使用
```

---

## ネットワークセキュリティ

### HTTPS必須

すべてのAPI通信は**HTTPS**で行います。

```rust
// HTTPSのみ許可
const BLUESKY_API_ENDPOINT: &str = "https://bsky.social"; // ✅
// const BLUESKY_API_ENDPOINT: &str = "http://bsky.social"; // ❌

// 自動的にHTTPSにアップグレード
let url = if url.starts_with("http://") {
    url.replace("http://", "https://")
} else {
    url
};
```

### TLS証明書の検証

```rust
use reqwest::Client;

// デフォルトでTLS証明書を検証
let client = Client::builder()
    .danger_accept_invalid_certs(false) // ✅ 証明書検証を有効化
    .build()?;
```

**注意**: 開発環境でも証明書検証をスキップしないでください。

### プロキシの考慮

```rust
// 環境変数からプロキシ設定を読み取る
let client = Client::builder()
    .use_rustls_tls() // Rust-native TLS（推奨）
    .build()?;

// HTTP_PROXY, HTTPS_PROXY 環境変数を自動的に使用
```

---

## ログとデバッグ

### 機密情報のマスキング

```rust
use tracing::{info, debug};

// ❌ 悪い例
debug!("Authenticating with password: {}", password);

// ✅ 良い例
debug!("Authenticating with password: [REDACTED]");

// ✅ 部分的に表示（開発時のみ）
#[cfg(debug_assertions)]
debug!("Password (first 4 chars): {}****", &password[..4]);
```

### ログレベルの適切な使用

```rust
// error: 認証失敗等の重要なエラー（機密情報を含めない）
error!("Authentication failed for user: {}", username);

// warn: 潜在的な問題
warn!("API rate limit approaching");

// info: 一般的な情報
info!("Posted to Bluesky successfully");

// debug: デバッグ情報（開発時のみ）
debug!("HTTP request: {:?}", sanitized_request);

// trace: 詳細なトレース（開発時のみ）
trace!("Session token expires at: {:?}", expiry);
```

### 本番環境での推奨設定

```bash
# エラーと警告のみ表示
export RUST_LOG=social_cli=warn

# またはログ無効化
export RUST_LOG=off
```

### デバッグ時の注意

```bash
# デバッグログを有効化
export RUST_LOG=debug
social-cli post -m "Test"

# ログを確認後、必ず無効化
unset RUST_LOG
```

---

## 脅威モデル

### 想定される脅威

#### 1. ローカルファイルシステムへのアクセス

**脅威**: 攻撃者がユーザーのファイルシステムにアクセス

**対策**:
- キーチェーンで認証情報を暗号化
- 設定ファイルのパーミッション制限（0o600）
- 一時ファイルの適切な削除

#### 2. ネットワーク盗聴

**脅威**: 中間者攻撃（MITM）でAPI通信を盗聴

**対策**:
- HTTPS必須
- TLS証明書検証
- ピンニング（将来実装検討）

#### 3. メモリダンプ

**脅威**: メモリダンプから認証情報を抽出

**対策**:
- パスワード使用後は速やかにゼロクリア
```rust
use zeroize::Zeroize;

let mut password = get_password()?;
// 使用
authenticate(&password).await?;
// ゼロクリア
password.zeroize();
```

#### 4. ログファイルからの漏洩

**脅威**: ログファイルに記録された認証情報

**対策**:
- ログに機密情報を含めない
- マスキング処理
- 本番環境でのログレベル制限

#### 5. 環境変数・コマンドライン引数

**脅威**: プロセス一覧から認証情報を読み取り

**対策**:
- コマンドライン引数でパスワード渡しを禁止
```bash
# ❌ 悪い例
social-cli setup --password my-secret-password

# ✅ 良い例（対話形式）
social-cli setup
App Password: ****
```

---

## セキュリティチェックリスト

### 開発時

- [ ] 認証情報はキーチェーンに保存
- [ ] 設定ファイルに機密情報を含めない
- [ ] パーミッションを0o600に設定
- [ ] HTTPS必須
- [ ] TLS証明書検証を有効化
- [ ] ログに機密情報を出力しない
- [ ] パスワードをゼロクリア
- [ ] エラーメッセージに機密情報を含めない

### デプロイ時

- [ ] ログレベルをwarning以上に設定
- [ ] デバッグビルドを無効化
- [ ] 依存関係の脆弱性スキャン
```bash
cargo audit
```
- [ ] リリースビルドでストリップ
```bash
cargo build --release
strip target/release/social-cli
```

### 使用時

- [ ] App Passwordを使用（メインパスワードは使わない）
- [ ] アプリごとに一意のApp Password
- [ ] 定期的なApp Passwordのローテーション
- [ ] 不要なApp Passwordの削除
- [ ] 信頼できないネットワークでの使用を避ける
- [ ] 共有コンピューターでの使用を避ける

---

## インシデント対応

### 認証情報の漏洩が疑われる場合

#### 即座に実行

1. **App Passwordの無効化**
```
Bluesky Settings → App Passwords → Delete
```

2. **新しいApp Passwordの生成**
```bash
# 新しいパスワードで再セットアップ
social-cli setup
```

3. **ログ確認**
```bash
# 不正なアクセスがないか確認
# Bluesky等のWebインターフェースで確認
```

#### 調査

- どのように漏洩したか特定
- 影響範囲の確認
- 他のアカウントへの影響確認

### 設定ファイルをGitにコミットした場合

```bash
# 1. リポジトリ履歴から削除
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch config.toml" \
  --prune-empty --tag-name-filter cat -- --all

# 2. 強制プッシュ（注意！）
git push origin --force --all

# 3. App Passwordを無効化し、再生成
```

**注意**: 公開リポジトリの場合、すでに漏洩している可能性があります。即座にApp Passwordを無効化してください。

---

## コンプライアンス

### GDPR（欧州一般データ保護規則）

- ユーザーデータの最小化
- データの暗号化（キーチェーン）
- データ削除の権利（アンインストール手順提供）

### その他の規制

- 各国のプライバシー法規制を遵守
- SNSプラットフォームの利用規約を遵守

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

---

## セキュアコーディングガイドライン

### 1. 入力検証

```rust
// ユーザー入力を検証
pub fn validate_handle(handle: &str) -> Result<()> {
    if handle.is_empty() {
        return Err(SocialCliError::ConfigError("Handle cannot be empty".into()));
    }

    if handle.len() > 253 {
        return Err(SocialCliError::ConfigError("Handle too long".into()));
    }

    // 不正な文字を確認
    if !handle.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
        return Err(SocialCliError::ConfigError("Invalid handle format".into()));
    }

    Ok(())
}
```

### 2. エラーメッセージ

```rust
// ❌ 悪い例（詳細すぎる）
return Err(format!("Authentication failed for user {} with password {}", user, pass));

// ✅ 良い例（最小限）
return Err("Authentication failed".into());
```

### 3. タイミング攻撃対策

```rust
use constant_time_eq::constant_time_eq;

// パスワード比較は定数時間で
fn verify_password(input: &str, stored: &str) -> bool {
    constant_time_eq(input.as_bytes(), stored.as_bytes())
}
```

### 4. SQLインジェクション対策（将来、DBを使う場合）

```rust
// プリペアドステートメント使用
let stmt = conn.prepare("SELECT * FROM posts WHERE id = ?")?;
stmt.query_row([post_id], |row| {
    // ...
})?;
```

---

## セキュリティ連絡先

セキュリティ上の問題を発見した場合:

1. **公開Issue作成は避ける**（脆弱性が悪用される可能性）
2. プロジェクトメンテナーに直接連絡
3. 詳細な情報を提供:
   - 脆弱性の説明
   - 再現手順
   - 影響範囲
   - 修正案（あれば）

---

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

セキュリティは継続的なプロセスです。以下を心がけてください:

1. **認証情報は絶対に平文保存しない**
2. **ログに機密情報を含めない**
3. **HTTPS必須**
4. **定期的な更新とパスワードローテーション**
5. **最小権限の原則**

次のステップ:

- [architecture.md](architecture.md) - システムアーキテクチャ
- [setup.md](setup.md) - セキュアなセットアップ手順
