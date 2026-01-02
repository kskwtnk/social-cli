use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

const TEMPLATE: &str = r#"
# Enter your message above this line.
# Lines starting with '#' will be ignored.
# An empty message aborts the post.
"#;

/// Opens the user's preferred editor to compose a message
pub fn open_editor() -> Result<String> {
    // Get editor from environment variable, fallback to vim
    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string());

    // Create a temporary file with .txt extension
    let mut temp_file = NamedTempFile::new()
        .context("Failed to create temporary file")?;

    // Write template to the file
    temp_file.write_all(TEMPLATE.as_bytes())
        .context("Failed to write template to temporary file")?;

    // Get the path before closing
    let temp_path = temp_file.path().to_path_buf();

    // Flush to ensure content is written
    temp_file.flush()
        .context("Failed to flush temporary file")?;

    // Launch the editor and wait for it to close
    let status = Command::new(&editor)
        .arg(&temp_path)
        .status()
        .context(format!("Failed to launch editor: {}", editor))?;

    // Check if the editor exited successfully
    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status. Post aborted.");
    }

    // Read the edited content
    let content = fs::read_to_string(&temp_path)
        .context("Failed to read edited content")?;

    // Process the content: remove comment lines and trim
    let message = content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    // Check if message is empty
    if message.is_empty() {
        anyhow::bail!("Empty message. Post aborted.");
    }

    Ok(message)
}
