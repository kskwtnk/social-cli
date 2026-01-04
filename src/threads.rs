use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::env;

/// Response from creating a media container
#[derive(Debug, Deserialize)]
struct CreateContainerResponse {
    id: String,
}

/// Response from publishing a thread
#[derive(Debug, Deserialize)]
struct PublishResponse {
    id: String,
    #[serde(default)]
    permalink: Option<String>,
}

/// Post a message to Threads
/// Returns the URL of the created thread
pub async fn post(message: &str) -> Result<String> {
    // Get credentials from environment variables
    let user_id = env::var("THREADS_USER_ID")
        .context("THREADS_USER_ID not set in .env file")?;
    let access_token = env::var("THREADS_ACCESS_TOKEN")
        .context("THREADS_ACCESS_TOKEN not set in .env file")?;

    let client = Client::new();

    // Step 1: Create media container
    let create_url = format!("https://graph.threads.net/v1.0/{}/threads", user_id);

    let create_response = client
        .post(&create_url)
        .query(&[
            ("media_type", "TEXT"),
            ("text", message),
            ("access_token", &access_token),
        ])
        .send()
        .await
        .map_err(|e| {
            eprintln!("DEBUG - Threads create container error: {:?}", e);
            anyhow::anyhow!("Failed to create media container. Original error: {:?}", e)
        })?;

    let status = create_response.status();
    if !status.is_success() {
        let error_text = create_response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!(
            "Threads API returned error {} when creating container: {}",
            status,
            error_text
        ));
    }

    let container: CreateContainerResponse = create_response
        .json()
        .await
        .context("Failed to parse container creation response")?;

    // Step 2: Publish the container
    let publish_url = format!("https://graph.threads.net/v1.0/{}/threads_publish", user_id);

    let publish_response = client
        .post(&publish_url)
        .query(&[
            ("creation_id", &container.id),
            ("access_token", &access_token),
        ])
        .send()
        .await
        .context("Failed to publish thread")?;

    let status = publish_response.status();
    if !status.is_success() {
        let error_text = publish_response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!(
            "Threads API returned error {} when publishing: {}",
            status,
            error_text
        ));
    }

    let publish: PublishResponse = publish_response
        .json()
        .await
        .context("Failed to parse publish response")?;

    // Step 3: Get permalink by querying the post
    let post_url = format!("https://graph.threads.net/v1.0/{}", publish.id);

    let post_response = client
        .get(&post_url)
        .query(&[
            ("fields", "permalink"),
            ("access_token", &access_token),
        ])
        .send()
        .await;

    // Try to get permalink, fallback to app-based message
    let thread_url = if let Ok(response) = post_response {
        if let Ok(data) = response.json::<PublishResponse>().await {
            data.permalink.unwrap_or_else(|| {
                format!("https://www.threads.com/t/{} (Note: Open Threads app to view)", publish.id)
            })
        } else {
            format!("Post ID: {} (Note: Open Threads app to view)", publish.id)
        }
    } else {
        format!("Post ID: {} (Note: Open Threads app to view)", publish.id)
    };

    Ok(thread_url)
}
