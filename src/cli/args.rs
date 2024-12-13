use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// URL to fetch HTML from
    #[arg(short, long)]
    pub url: Url,

    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<String>,
}
