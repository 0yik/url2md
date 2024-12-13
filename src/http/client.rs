use anyhow::Result;
use reqwest::Client;
use url::Url;
use once_cell::sync::Lazy;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .build()
        .expect("Failed to create HTTP client")
});

pub async fn fetch_html(url: &Url) -> Result<String> {
    let response = CLIENT.get(url.as_str())
        .header("Accept-Encoding", "gzip, deflate, br")
        .send()
        .await?;
    let html = response.text().await?;
    Ok(html)
}

pub async fn save_to_file(content: &str, path: &str) -> Result<()> {
    tokio::fs::write(path, content).await?;
    Ok(())
}

#[cfg(test)]
#[path = "client_test.rs"]
mod tests;
