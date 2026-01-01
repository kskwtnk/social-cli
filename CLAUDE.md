# Social CLI - Multi-Platform Social Media Posting Tool

## プロジェクト概要

RustベースのCLIツールで、複数のソーシャルメディアプラットフォーム（Bluesky、X (Twitter)、Threads）に同時投稿できるツールです。

### 目的

- 単一のコマンドで複数のSNSに同時投稿
- 認証情報の安全な管理（OSキーチェーン使用）
- シンプルで直感的なCLIインターフェース

### 対応プラットフォーム（予定）

- ✅ Bluesky (Phase 1: MVP)
- ⏳ X (Twitter) (Phase 2)
- ⏳ Threads (Phase 3)

---

## 技術スタック

### 言語・ランタイム
- **Rust** (Edition 2021)
- **非同期**: tokio

### 主要クレート

| カテゴリ | クレート | 用途 |
|---------|---------|------|
| CLI | clap | コマンドライン引数パース |
| 非同期 | tokio, async-trait | 非同期ランタイム |
| HTTP | reqwest | HTTP クライアント |
| Bluesky | atrium-api, atrium-xrpc-client | Bluesky API クライアント |
| 設定 | serde, toml, directories | 設定ファイル管理 |
| セキュリティ | keyring, rpassword | 認証情報の安全な保存 |
| エラー処理 | thiserror, anyhow | カスタムエラー型 |
| UI | colored | カラー出力 |

---

## プロジェクト構造

```
social-cli/
├── CLAUDE.md                    # このファイル
├── Cargo.toml                   # プロジェクト定義
├── .gitignore
├── docs/                        # 詳細ドキュメント
│   ├── architecture.md          # アーキテクチャ設計
│   ├── setup.md                 # セットアップガイド
│   ├── usage.md                 # 使用方法
│   ├── api-integration.md       # API統合詳細
│   └── security.md              # セキュリティ考慮事項
├── src/
│   ├── main.rs                  # エントリーポイント
│   ├── lib.rs                   # ライブラリルート
│   ├── error.rs                 # エラー型定義
│   ├── config.rs                # 設定管理
│   ├── commands/                # CLIコマンド
│   │   ├── mod.rs
│   │   ├── setup.rs             # 初期設定
│   │   ├── post.rs              # 投稿機能
│   │   └── status.rs            # 設定確認
│   ├── platforms/               # SNSプラットフォーム統合
│   │   ├── mod.rs
│   │   ├── traits.rs            # 共通trait
│   │   ├── bluesky.rs           # Bluesky実装
│   │   └── twitter.rs           # Twitter実装（Phase 2）
│   └── utils/                   # ユーティリティ
│       ├── mod.rs
│       └── keyring.rs           # 認証情報保存
└── tests/
    └── integration_tests.rs
```

---

## クイックリファレンス

### 重要ファイル

| ファイル | 役割 |
|---------|------|
| [src/error.rs](src/error.rs) | カスタムエラー型（`SocialCliError`）- 全体の基盤 |
| [src/platforms/traits.rs](src/platforms/traits.rs) | `SocialPlatform` trait - プラットフォーム抽象化 |
| [src/config.rs](src/config.rs) | 設定ファイル管理 |
| [src/utils/keyring.rs](src/utils/keyring.rs) | 認証情報の安全な保存 |
| [src/commands/post.rs](src/commands/post.rs) | 投稿ロジック |
| [src/platforms/bluesky.rs](src/platforms/bluesky.rs) | Bluesky API統合 |

### コマンドリファレンス

```bash
# 初期設定
social-cli setup

# 全SNSに投稿
social-cli post -m "Hello, world!"

# 設定確認
social-cli status
```

---

## アーキテクチャ概要

### コアデザイン原則

1. **プラットフォーム抽象化**: `SocialPlatform` traitで統一インターフェース
2. **セキュリティ第一**: 認証情報はOSキーチェーンに暗号化保存
3. **エラー透過性**: 詳細なエラーメッセージで問題特定を容易に
4. **並列投稿**: 複数SNSへの同時投稿で高速化

### SocialPlatform Trait

すべてのSNSプラットフォームは以下のtraitを実装します：

```rust
#[async_trait]
pub trait SocialPlatform: Send + Sync {
    fn name(&self) -> &'static str;
    async fn verify_credentials(&self) -> Result<bool>;
    async fn post_text(&self, message: &str) -> Result<PostResponse>;
    fn max_message_length(&self) -> usize;
}
```

### エラーハンドリング

カスタムエラー型（`SocialCliError`）で一貫したエラー処理：

- `ConfigError`: 設定ファイル関連
- `AuthError`: 認証エラー
- `ApiError`: API呼び出しエラー
- `NetworkError`: ネットワークエラー

詳細は [docs/architecture.md](docs/architecture.md) を参照。

---

## 設定とセキュリティ

### 設定ファイルの場所

- **Unix/macOS**: `~/.config/social-cli/config.toml`
- **Windows**: `%APPDATA%\social-cli\config.toml`

### 認証情報の保存

パスワード・トークンはOSキーチェーンに保存：

- **macOS**: Keychain
- **Windows**: Credential Manager
- **Linux**: Secret Service (libsecret)

設定ファイルには**認証情報を含めません**。

詳細は [docs/security.md](docs/security.md) を参照。

---

## 開発フェーズ

### Phase 0: ドキュメント作成 ✅
- プロジェクト設計とドキュメント整備

### Phase 1: Bluesky MVP（現在）
- Bluesky投稿機能の実装
- 設定管理とキーチェーン統合
- 基本的なCLIコマンド

### Phase 2: X (Twitter) 統合
- X API統合（有料プラン前提）
- マルチプラットフォーム投稿のエラーハンドリング強化

### Phase 3: 拡張機能
- Threads API統合
- 画像添付サポート
- 投稿履歴管理

---

## AI開発時の重要な制約・方針

### コーディング規約

1. **エラー処理**: すべてのエラーは`Result<T, SocialCliError>`で処理
2. **非同期**: I/O操作は必ず`async/await`を使用
3. **設定の不変性**: 設定読み込み後は変更しない（イミュータブル）
4. **セキュリティ**: 認証情報をログ出力しない、平文保存しない

### テスト方針

- ユニットテスト: 各モジュールに`#[cfg(test)] mod tests`
- 統合テスト: `tests/`ディレクトリ
- モックAPI: 本番APIを叩かないテスト環境

### セキュリティチェックリスト

- [ ] 認証情報はキーチェーンに保存
- [ ] 設定ファイルのパーミッション = `0o600`
- [ ] トークン・パスワードをログに出力しない
- [ ] HTTPSのみ使用（HTTP自動アップグレード）

---

## トラブルシューティング

### よくある問題

1. **キーチェーンアクセスエラー**
   - macOSの場合、初回はキーチェーンへのアクセス許可が必要
   - Linux の場合、`libsecret-1-dev`のインストールが必要

2. **Bluesky認証エラー**
   - App Passwordを使用（通常パスワードではない）
   - Bluesky設定から生成: Settings → App Passwords

3. **ビルドエラー**
   - Rust最新安定版を使用（`rustup update stable`）
   - 依存関係のバージョン競合確認

詳細は [docs/usage.md](docs/usage.md) を参照。

---

## 参考リソース

### 公式ドキュメント

- [Bluesky AT Protocol](https://docs.bsky.app/)
- [X API Documentation](https://developer.x.com/en/docs)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

### 関連クレート

- [clap](https://docs.rs/clap/)
- [tokio](https://docs.rs/tokio/)
- [atrium-api](https://docs.rs/atrium-api/)
- [keyring](https://docs.rs/keyring/)

---

## ライセンス

MIT License
