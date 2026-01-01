# API統合ガイド

このドキュメントでは、各SNSプラットフォームのAPI統合について詳しく説明します。

---

## 目次

- [Bluesky API](#bluesky-api)
- [X (Twitter) API](#x-twitter-api)
- [Threads API](#threads-api)
- [新規プラットフォーム追加ガイド](#新規プラットフォーム追加ガイド)

---

## Bluesky API

### 概要

BlueskyはAT Protocol（Authenticated Transfer Protocol）を使用した分散型ソーシャルネットワークです。

### 公式ドキュメント

- [AT Protocol Documentation](https://docs.bsky.app/)
- [API Reference](https://docs.bsky.app/docs/api/)

### 使用ライブラリ

| クレート | バージョン | 用途 |
|---------|-----------|------|
| `atrium-api` | 0.22+ | AT Protocol API定義 |
| `atrium-xrpc-client` | 0.10+ | XRPCクライアント |

### 認証方法

Blueskyは**App Password**方式を採用しています。

#### App Passwordとは

- Blueskyアカウントごとに生成する専用パスワード
- アプリケーションごとに個別に生成・管理
- メインパスワードではなく、限定的な権限を持つ
- いつでも削除・無効化可能

#### App Password生成手順

1. Blueskyアプリまたはウェブにログイン
2. Settings → Privacy and Security
3. App Passwords → Add App Password
4. 名前を入力（例: "social-cli"）
5. 生成されたパスワードをコピー（**一度しか表示されません**）

### 実装詳細

#### セッション作成

```rust
use atrium_api::client::AtpServiceClient;
use atrium_api::com::atproto::server::create_session::{Input, Output};
use atrium_xrpc_client::reqwest::ReqwestClient;

pub async fn create_session(
    identifier: &str,
    password: &str,
) -> Result<Output> {
    let client = AtpServiceClient::new(
        ReqwestClient::new("https://bsky.social")
    );

    let input = Input {
        identifier: identifier.into(),
        password: password.to_string(),
    };

    let output = client
        .service
        .com
        .atproto
        .server
        .create_session(input)
        .await?;

    Ok(output)
}
```

#### 投稿作成

```rust
use atrium_api::app::bsky::feed::post::{Record, RecordData};
use atrium_api::com::atproto::repo::create_record::{Input, Output};
use atrium_api::types::string::{AtIdentifier, Datetime};

pub async fn create_post(
    client: &AtpServiceClient<ReqwestClient>,
    did: &str,
    text: &str,
) -> Result<Output> {
    let record = Record {
        created_at: Datetime::now(),
        embed: None,
        entities: None,
        facets: None,
        labels: None,
        langs: None,
        reply: None,
        tags: None,
        text: text.to_string(),
    };

    let input = Input {
        collection: "app.bsky.feed.post".parse()?,
        record: record.into(),
        repo: did.parse()?,
        rkey: None,
        swap_commit: None,
        validate: Some(true),
    };

    let output = client
        .service
        .com
        .atproto
        .repo
        .create_record(input)
        .await?;

    Ok(output)
}
```

#### プロフィール取得（認証確認）

```rust
use atrium_api::app::bsky::actor::get_profile::{Parameters, Output};

pub async fn get_profile(
    client: &AtpServiceClient<ReqwestClient>,
    actor: &str,
) -> Result<Output> {
    let params = Parameters {
        actor: actor.parse()?,
    };

    let output = client
        .service
        .app
        .bsky
        .actor
        .get_profile(params)
        .await?;

    Ok(output)
}
```

### レート制限

Bluesky APIのレート制限（2025年時点）:

| エンドポイント | 制限 |
|--------------|------|
| createSession | 30 req/5min |
| createRecord (投稿) | 300 req/5min |
| getProfile | 3000 req/5min |

**推奨事項**:
- セッションは可能な限り再利用
- バーストアクセスを避ける
- エラーレスポンスでリトライ遅延を実装

### エラーハンドリング

```rust
match client.create_record(input).await {
    Ok(output) => Ok(output),
    Err(e) => {
        if e.to_string().contains("InvalidSession") {
            // セッション再作成
            create_session(identifier, password).await?;
            // リトライ
        } else if e.to_string().contains("RateLimitExceeded") {
            // 待機してリトライ
            tokio::time::sleep(Duration::from_secs(60)).await;
        } else {
            Err(e.into())
        }
    }
}
```

### URLの構築

投稿URLの形式:

```
https://bsky.app/profile/{handle}/post/{rkey}
```

例:

```rust
fn build_post_url(handle: &str, rkey: &str) -> String {
    format!("https://bsky.app/profile/{}/post/{}", handle, rkey)
}
```

---

## X (Twitter) API

### 概要

X APIは有料化されており、投稿機能を使用するにはBasicプラン以上が必要です。

### 公式ドキュメント

- [X API Documentation](https://developer.x.com/en/docs)
- [API Reference](https://developer.x.com/en/docs/api-reference-index)

### API Tier

| Tier | 月額 | 投稿機能 | 月間投稿数 |
|------|------|---------|-----------|
| Free | $0 | **不可** | 0 |
| Basic | $100 | 可 | 3,000 |
| Pro | $5,000 | 可 | 300,000 |

**重要**: 2023年以降、Free Tierでは投稿ができません。

### 使用ライブラリ（候補）

| クレート | 状態 | 備考 |
|---------|------|------|
| `egg-mode` | メンテナンス低下 | 歴史的なライブラリ、v1 API中心 |
| `twitter-v2` | 開発中 | v2 API対応、推奨 |
| カスタム実装 | - | `reqwest` + `oauth1` で自作 |

### 認証方法

#### OAuth 1.0a（推奨）

```rust
use oauth1::Token;

pub struct TwitterClient {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
}

impl TwitterClient {
    fn create_auth_header(&self, method: &str, url: &str) -> String {
        let token = Token::from_parts(
            &self.consumer_key,
            &self.consumer_secret,
            &self.access_token,
            &self.access_token_secret,
        );

        oauth1::authorize(method, url, &token, oauth1::HmacSha1, None, None)
    }
}
```

#### OAuth 2.0（ユーザーコンテキスト）

```rust
// 実装例（簡略版）
pub async fn create_tweet_oauth2(
    access_token: &str,
    text: &str,
) -> Result<TweetResponse> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "text": text
    });

    let response = client
        .post("https://api.twitter.com/2/tweets")
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?;

    let tweet: TweetResponse = response.json().await?;
    Ok(tweet)
}
```

### 投稿API

#### POST /2/tweets

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateTweetRequest {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTweetResponse {
    pub data: TweetData,
}

#[derive(Debug, Deserialize)]
pub struct TweetData {
    pub id: String,
    pub text: String,
}

pub async fn create_tweet(
    client: &TwitterClient,
    text: &str,
) -> Result<CreateTweetResponse> {
    let url = "https://api.twitter.com/2/tweets";
    let auth_header = client.create_auth_header("POST", url);

    let body = CreateTweetRequest {
        text: text.to_string(),
    };

    let response = reqwest::Client::new()
        .post(url)
        .header("Authorization", auth_header)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(SocialCliError::ApiError {
            platform: "Twitter".to_string(),
            message: error_text,
        });
    }

    let tweet_response: CreateTweetResponse = response.json().await?;
    Ok(tweet_response)
}
```

### レート制限

| Tier | エンドポイント | 制限 |
|------|--------------|------|
| Basic | POST /2/tweets | 300 req/15min |
| Pro | POST /2/tweets | 3,000 req/15min |

### エラーハンドリング

```rust
match create_tweet(&client, text).await {
    Ok(response) => Ok(response),
    Err(e) => {
        if e.to_string().contains("429") {
            // Rate limit
            Err(SocialCliError::ApiError {
                platform: "Twitter".to_string(),
                message: "Rate limit exceeded".to_string(),
            })
        } else if e.to_string().contains("401") {
            // 認証エラー
            Err(SocialCliError::AuthError(
                "Invalid credentials".to_string()
            ))
        } else {
            Err(e)
        }
    }
}
```

### URLの構築

投稿URLの形式:

```
https://twitter.com/{username}/status/{tweet_id}
```

**注意**: username取得のため追加API呼び出しが必要な場合があります。

---

## Threads API

### 概要

Threads APIはMeta Graph APIを通じて提供されます。現在は限定公開中です。

### 公式ドキュメント

- [Threads API Documentation](https://developers.facebook.com/docs/threads)

### アクセス要件

1. Meta for Developersアカウント
2. アプリケーション登録
3. Threads API アクセス申請（審査あり）
4. Threads Professionalアカウント（ビジネスアカウント）

### 認証方法

#### アクセストークン

```rust
pub struct ThreadsClient {
    user_id: String,
    access_token: String,
}
```

アクセストークンの取得はOAuth 2.0フローを使用:

1. ユーザーを認証URL に誘導
2. リダイレクトでcode取得
3. codeをaccess_tokenに交換

### 投稿API

#### POST /{user-id}/threads

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateThreadsPostRequest {
    pub media_type: String, // "TEXT"
    pub text: String,
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateThreadsPostResponse {
    pub id: String,
}

pub async fn create_threads_post(
    client: &ThreadsClient,
    text: &str,
) -> Result<String> {
    // 1. メディアコンテナ作成
    let container_url = format!(
        "https://graph.threads.net/v1.0/{}/threads",
        client.user_id
    );

    let body = CreateThreadsPostRequest {
        media_type: "TEXT".to_string(),
        text: text.to_string(),
        access_token: client.access_token.clone(),
    };

    let response = reqwest::Client::new()
        .post(&container_url)
        .form(&body)
        .send()
        .await?;

    let container: CreateThreadsPostResponse = response.json().await?;

    // 2. コンテナを公開
    let publish_url = format!(
        "https://graph.threads.net/v1.0/{}/threads_publish",
        client.user_id
    );

    let publish_body = serde_json::json!({
        "creation_id": container.id,
        "access_token": client.access_token,
    });

    let publish_response = reqwest::Client::new()
        .post(&publish_url)
        .form(&publish_body)
        .send()
        .await?;

    let result: CreateThreadsPostResponse = publish_response.json().await?;
    Ok(result.id)
}
```

### レート制限

Meta Graph APIのレート制限に従います（詳細は公式ドキュメント参照）。

### 文字数制限

- 最大500文字

---

## 新規プラットフォーム追加ガイド

新しいSNSプラットフォームを追加する手順です。

### ステップ1: Platform構造体の定義

`src/platforms/newplatform.rs`:

```rust
use crate::error::Result;
use crate::platforms::traits::{SocialPlatform, PostResponse};
use async_trait::async_trait;

pub struct NewPlatformClient {
    // 認証情報など
    api_key: String,
    api_secret: String,
}

impl NewPlatformClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self { api_key, api_secret }
    }

    // 内部ヘルパーメソッド
    async fn authenticate(&self) -> Result<()> {
        // 認証ロジック
        Ok(())
    }
}
```

### ステップ2: SocialPlatform Traitの実装

```rust
#[async_trait]
impl SocialPlatform for NewPlatformClient {
    fn name(&self) -> &'static str {
        "NewPlatform"
    }

    async fn verify_credentials(&self) -> Result<bool> {
        // 認証確認ロジック
        Ok(true)
    }

    async fn post_text(&self, message: &str) -> Result<PostResponse> {
        // 投稿ロジック
        Ok(PostResponse {
            platform: "NewPlatform".to_string(),
            post_id: "123".to_string(),
            url: Some("https://newplatform.com/post/123".to_string()),
            timestamp: chrono::Utc::now(),
        })
    }

    fn max_message_length(&self) -> usize {
        500 // プラットフォームの制限
    }
}
```

### ステップ3: Configに追加

`src/config.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformConfig {
    pub bluesky: Option<BlueskyConfig>,
    pub twitter: Option<TwitterConfig>,
    pub newplatform: Option<NewPlatformConfig>, // 追加
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewPlatformConfig {
    pub enabled: bool,
    pub api_key: String,
    // api_secret はキーチェーンに保存
}
```

### ステップ4: Keyring関数の追加

`src/utils/keyring.rs`:

```rust
pub fn save_newplatform_secret(api_key: &str, api_secret: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, &format!("newplatform:{}", api_key))?;
    entry.set_password(api_secret)?;
    Ok(())
}

pub fn get_newplatform_secret(api_key: &str) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, &format!("newplatform:{}", api_key))?;
    Ok(entry.get_password()?)
}
```

### ステップ5: Setupコマンドに追加

`src/commands/setup.rs`:

```rust
async fn setup_newplatform(config: &mut Config) -> Result<()> {
    println!("NewPlatform Setup");
    println!("-----------------");

    print!("API Key: ");
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim();

    let api_secret = rpassword::prompt_password("API Secret: ")?;

    // 認証テスト
    let client = NewPlatformClient::new(api_key.to_string(), api_secret.clone());
    client.authenticate().await?;

    if client.verify_credentials().await? {
        keyring::save_newplatform_secret(api_key, &api_secret)?;
        config.platforms.newplatform = Some(NewPlatformConfig {
            enabled: true,
            api_key: api_key.to_string(),
        });
        println!("✓ NewPlatform configured successfully");
    } else {
        return Err(SocialCliError::AuthError("Invalid credentials".into()));
    }

    Ok(())
}
```

### ステップ6: Postコマンドに追加

`src/commands/post.rs`:

```rust
if should_post_to("newplatform", &args.platform) {
    if let Some(ref np_config) = config.platforms.newplatform {
        if np_config.enabled {
            let api_secret = keyring::get_newplatform_secret(&np_config.api_key)?;
            let client = NewPlatformClient::new(
                np_config.api_key.clone(),
                api_secret
            );
            client.authenticate().await?;
            clients.push(Box::new(client));
        }
    }
}
```

### ステップ7: Module登録

`src/platforms/mod.rs`:

```rust
pub mod traits;
pub mod bluesky;
pub mod twitter;
pub mod newplatform; // 追加

pub use traits::*;
pub use bluesky::BlueskyClient;
pub use twitter::TwitterClient;
pub use newplatform::NewPlatformClient; // 追加
```

---

## テスト

### モックテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockPlatform;

    #[async_trait]
    impl SocialPlatform for MockPlatform {
        fn name(&self) -> &'static str {
            "Mock"
        }

        async fn verify_credentials(&self) -> Result<bool> {
            Ok(true)
        }

        async fn post_text(&self, _message: &str) -> Result<PostResponse> {
            Ok(PostResponse {
                platform: "Mock".to_string(),
                post_id: "test123".to_string(),
                url: None,
                timestamp: chrono::Utc::now(),
            })
        }

        fn max_message_length(&self) -> usize {
            280
        }
    }

    #[tokio::test]
    async fn test_mock_post() {
        let client = MockPlatform;
        let result = client.post_text("Test message").await;
        assert!(result.is_ok());
    }
}
```

---

## ベストプラクティス

### 1. エラーハンドリング

- プラットフォーム固有のエラーを`SocialCliError`に変換
- Rate limitエラーは明示的に処理
- リトライロジックを実装

### 2. 認証情報管理

- APIキー/トークンはキーチェーンに保存
- セッションは可能な限り再利用
- 定期的な認証確認

### 3. レート制限対応

- 各プラットフォームの制限を把握
- バックオフアルゴリズムの実装
- ユーザーへの適切なフィードバック

### 4. ログ出力

- 機密情報をログに含めない
- デバッグレベルで詳細情報
- 本番では警告以上のみ

---

## 次のステップ

- [security.md](security.md) - セキュリティのベストプラクティス
- [architecture.md](architecture.md) - システムアーキテクチャ
