use clap::{Parser, Subcommand};
use anyhow::Result;

mod bluesky;
mod editor;
mod threads;
mod x;

/// Supported social media platforms
#[derive(Debug, Clone, Copy)]
enum Platform {
    Bluesky,
    X,
    Threads,
}

impl Platform {
    /// Get the display name of the platform
    fn name(&self) -> &'static str {
        match self {
            Platform::Bluesky => "Bluesky",
            Platform::X => "X",
            Platform::Threads => "Threads",
        }
    }

    /// Post a message to this platform
    async fn post(&self, message: &str) -> Result<String> {
        match self {
            Platform::Bluesky => bluesky::post(message).await,
            Platform::X => x::post(message).await,
            Platform::Threads => threads::post(message).await,
        }
    }

    /// Get all platforms
    fn all() -> Vec<Platform> {
        vec![Platform::Bluesky, Platform::X, Platform::Threads]
    }
}

#[derive(Parser)]
#[command(name = "social-cli")]
#[command(about = "Multi-platform social media posting CLI tool", long_about = None)]
struct Cli {
    /// Message to post (posts to all platforms if no subcommand is specified)
    #[arg(short, long)]
    message: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Post a message to Bluesky only
    Bluesky {
        /// Message to post (opens editor if not provided)
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Post a message to X (Twitter) only
    X {
        /// Message to post (opens editor if not provided)
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Post a message to Threads only
    Threads {
        /// Message to post (opens editor if not provided)
        #[arg(short, long)]
        message: Option<String>,
    },
}

/// Get message from option or open editor
fn get_message(message_opt: Option<String>) -> Result<String> {
    message_opt.map(Ok).unwrap_or_else(|| editor::open_editor())
}

/// Post to a single platform and display result
async fn post_to_platform(platform: Platform, message: &str) -> Result<()> {
    let post_url = platform.post(message).await?;
    println!("✓ Posted to {} successfully!", platform.name());
    println!("View your post: {}", post_url);
    Ok(())
}

/// Post to a platform with error handling (for multi-platform posting)
async fn post_to_platform_safe(platform: Platform, message: &str) {
    match platform.post(message).await {
        Ok(post_url) => {
            println!("✓ Posted to {} successfully!", platform.name());
            println!("  {}", post_url);
        }
        Err(e) => {
            println!("✗ Failed to post to {}: {}", platform.name(), e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Bluesky { message }) => {
            let message = get_message(message)?;
            post_to_platform(Platform::Bluesky, &message).await
        }
        Some(Commands::X { message }) => {
            let message = get_message(message)?;
            post_to_platform(Platform::X, &message).await
        }
        Some(Commands::Threads { message }) => {
            let message = get_message(message)?;
            post_to_platform(Platform::Threads, &message).await
        }
        None => {
            // Post to all platforms
            let message = get_message(cli.message)?;

            println!("Posting to all platforms...\n");

            for platform in Platform::all() {
                post_to_platform_safe(platform, &message).await;
            }

            println!("\nPosting complete!");
            Ok(())
        }
    }
}
