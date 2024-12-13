mod cli;
mod converter;
mod http;

use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tracing::info;
use anyhow::Result;
use tokio::net::TcpListener;
use clap::Parser;

use crate::cli::Args;
use crate::converter::markdown_converter::MarkdownConverter;
use crate::http::{fetch_html, save_to_file};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with custom filter
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Parse command line arguments
    let args = Args::parse();

    if args.url.is_none() {
        // Start API server mode
        let app = http::server::create_router()
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            );

        let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
        let listener = TcpListener::bind(addr).await?;
        info!("API server listening on http://localhost:{}", args.port);
        
        axum::serve(listener, app.into_make_service())
            .await?;
    } else {
        // Handle CLI mode
        let url = args.url.unwrap();
        let html = fetch_html(&url).await?;
        let converter = MarkdownConverter::new();
        let markdown = converter.convert(&html)?;
        
        if let Some(output_path) = args.output.as_deref() {
            save_to_file(&markdown, output_path).await?;
            info!("Saved markdown to {}", output_path);
        } else {
            println!("{}", markdown);
        }
        
    }

    Ok(())
}
