use clap::Parser;
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// URL to convert to markdown
    #[arg(value_parser = parse_url)]
    pub url: Option<Url>,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Port number for the API server (default: 3000)
    #[arg(short = 'P', long, default_value = "3000")]
    pub port: u16,
}

fn parse_url(s: &str) -> Result<Url, String> {
    Url::parse(s).map_err(|e| e.to_string())
}
