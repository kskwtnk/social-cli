use clap::{Parser, Subcommand};
use anyhow::Result;

mod bluesky;

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
    Post {
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
        Commands::Post { message } => {
            let post_url = bluesky::post(&message).await?;
            println!("✓ Posted successfully!");
            println!("View your post: {}", post_url);
            Ok(())
        }
    }
}
