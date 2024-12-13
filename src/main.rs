mod cli;
mod converter;
mod http;

use clap::Parser;
use tracing::{info, error};
use anyhow::Result;
use chrono;
use std::time::Instant;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;
use crate::converter::markdown_converter::MarkdownConverter;
use crate::http::{fetch_html, save_to_file};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with custom filter
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        // Suppress html5ever warnings
        .parse("html5ever=error")?
        .add_directive("url_to_markdown=info".parse()?);
        
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    // Parse command line arguments
    let cli = Cli::parse();
    
    info!("Fetching HTML from {}", cli.url);
    
    // Fetch HTML content
    let html = match fetch_html(&cli.url).await {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to fetch HTML: {}", e);
            return Err(e.into());
        }
    };

    // Start timing
    let start_time = Instant::now();
    // Convert HTML to Markdown
    let converter = MarkdownConverter::new();
    let mut markdown = String::new();

    // Add URL source and timestamp before conversion
    markdown.push_str(&format!("URL Source: {}\n", cli.url));
    markdown.push_str(&format!("Generated on: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));

    // Convert HTML to markdown and append
    match converter.convert(&html) {
        Ok(content) => markdown.push_str(&content),
        Err(e) => {
            error!("Failed to convert HTML to Markdown: {}", e);
            return Err(e.into());
        }
    };

    // Handle output
    match cli.output {
        Some(path) => {
            info!("Saving markdown to {}", path);
            save_to_file(&markdown, &path).await?;
        }
        None => println!("{}", markdown),
    }

    // Show processing time in CLI
    info!("Processing time: {:.2?}", start_time.elapsed());

    Ok(())
}
