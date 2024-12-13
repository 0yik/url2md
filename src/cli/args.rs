use clap::Parser;
use url::Url;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// URL to convert to markdown
    #[arg(value_parser = parse_url)]
    pub url: Option<Url>,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Port number for the server
    #[arg(short = 'P', long, default_value_t = 3000)]
    pub port: u16,
}

pub fn parse_url(url: &str) -> Result<Url, String> {
    Url::parse(url).map_err(|e| e.to_string())
}

#[cfg(test)]
#[path = "args_test.rs"]
mod tests;
