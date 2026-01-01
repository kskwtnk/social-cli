use clap::{Parser, Subcommand};
use anyhow::Result;

mod bluesky;
mod threads;
mod x;

#[derive(Parser)]
#[command(name = "social-cli")]
#[command(about = "Multi-platform social media posting CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Post a message to Bluesky
    Bluesky {
        /// Message to post
        #[arg(short, long)]
        message: String,
    },
    /// Post a message to X (Twitter)
    X {
        /// Message to post
        #[arg(short, long)]
        message: String,
    },
    /// Post a message to Threads
    Threads {
        /// Message to post
        #[arg(short, long)]
        message: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Bluesky { message } => {
            let post_url = bluesky::post(&message).await?;
            println!("✓ Posted to Bluesky successfully!");
            println!("View your post: {}", post_url);
            Ok(())
        }
        Commands::X { message } => {
            let post_url = x::post(&message).await?;
            println!("✓ Posted to X successfully!");
            println!("View your tweet: {}", post_url);
            Ok(())
        }
        Commands::Threads { message } => {
            let post_url = threads::post(&message).await?;
            println!("✓ Posted to Threads successfully!");
            println!("View your thread: {}", post_url);
            Ok(())
        }
    }
}
