use anyhow::{Context, Result};
use atrium_api::agent::AtpAgent;
use atrium_api::types::string::{AtIdentifier, Datetime};
use atrium_api::types::Unknown;
use atrium_xrpc_client::reqwest::ReqwestClient;
use std::env;

/// Post a message to Bluesky
/// Returns the URL of the created post
pub async fn post(message: &str) -> Result<String> {
    // Get credentials from environment variables
    let identifier = env::var("BLUESKY_IDENTIFIER")
        .context("BLUESKY_IDENTIFIER not set in .env file")?;
    let password = env::var("BLUESKY_APP_PASSWORD")
        .context("BLUESKY_APP_PASSWORD not set in .env file")?;

    // Create ATP agent
    let agent = AtpAgent::new(
        ReqwestClient::new("https://bsky.social"),
        atrium_api::agent::store::MemorySessionStore::default(),
    );

    // Login and get session
    let session = agent
        .login(&identifier, &password)
        .await
        .map_err(|e| {
            eprintln!("DEBUG - Bluesky login error: {:?}", e);
            anyhow::anyhow!("Failed to authenticate with Bluesky. Please check your credentials. Original error: {:?}", e)
        })?;

    // Create post record
    let record = atrium_api::app::bsky::feed::post::RecordData {
        created_at: Datetime::now(),
        embed: None,
        entities: None,
        facets: None,
        labels: None,
        langs: None,
        reply: None,
        tags: None,
        text: message.to_string(),
    };

    // Convert record to Unknown
    let record_unknown: Unknown = serde_json::from_value(serde_json::to_value(&record)?)?;

    // Create the post
    let response = agent
        .api
        .com
        .atproto
        .repo
        .create_record(
            atrium_api::com::atproto::repo::create_record::InputData {
                collection: "app.bsky.feed.post"
                    .parse()
                    .map_err(|e: &str| anyhow::anyhow!(e))?,
                record: record_unknown,
                repo: AtIdentifier::Did(session.did.clone()),
                rkey: None,
                swap_commit: None,
                validate: None,
            }
            .into(),
        )
        .await
        .context("Failed to create post on Bluesky")?;

    // Extract rkey from URI (format: at://did:plc:xxx/app.bsky.feed.post/rkey)
    let rkey = response
        .uri
        .split('/')
        .last()
        .context("Failed to extract rkey from post URI")?;

    // Get handle or DID from session for reliable URL construction
    // Prefer handle over DID for user-friendly URLs
    let profile_id = session.handle.as_ref();

    // Construct post URL
    let post_url = format!("https://bsky.app/profile/{}/post/{}", profile_id, rkey);

    Ok(post_url)
}
