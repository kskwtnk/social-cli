use clap::{Parser, Subcommand};
use anyhow::Result;

mod bluesky;
mod editor;
mod threads;
mod x;

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

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Bluesky { message }) => {
            let message = message.map(Ok).unwrap_or_else(|| editor::open_editor())?;
            let post_url = bluesky::post(&message).await?;
            println!("✓ Posted to Bluesky successfully!");
            println!("View your post: {}", post_url);
            Ok(())
        }
        Some(Commands::X { message }) => {
            let message = message.map(Ok).unwrap_or_else(|| editor::open_editor())?;
            let post_url = x::post(&message).await?;
            println!("✓ Posted to X successfully!");
            println!("View your tweet: {}", post_url);
            Ok(())
        }
        Some(Commands::Threads { message }) => {
            let message = message.map(Ok).unwrap_or_else(|| editor::open_editor())?;
            let post_url = threads::post(&message).await?;
            println!("✓ Posted to Threads successfully!");
            println!("View your thread: {}", post_url);
            Ok(())
        }
        None => {
            // Post to all platforms
            let message = cli.message.map(Ok).unwrap_or_else(|| editor::open_editor())?;

            println!("Posting to all platforms...\n");

            // Post to Bluesky
            match bluesky::post(&message).await {
                Ok(post_url) => {
                    println!("✓ Posted to Bluesky successfully!");
                    println!("  {}", post_url);
                }
                Err(e) => {
                    println!("✗ Failed to post to Bluesky: {}", e);
                }
            }

            // Post to X
            match x::post(&message).await {
                Ok(post_url) => {
                    println!("✓ Posted to X successfully!");
                    println!("  {}", post_url);
                }
                Err(e) => {
                    println!("✗ Failed to post to X: {}", e);
                }
            }

            // Post to Threads
            match threads::post(&message).await {
                Ok(post_url) => {
                    println!("✓ Posted to Threads successfully!");
                    println!("  {}", post_url);
                }
                Err(e) => {
                    println!("✗ Failed to post to Threads: {}", e);
                }
            }

            println!("\nPosting complete!");
            Ok(())
        }
    }
}
