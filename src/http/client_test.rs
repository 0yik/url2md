#[cfg(test)]
mod tests {
    use crate::http::client::{fetch_html, save_to_file};
    use url::Url;
    use tempfile;
    use tokio;

    #[tokio::test]
    async fn test_save_to_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.md");
        let content = "Test content";

        save_to_file(content, file_path.to_str().unwrap()).await.unwrap();

        let saved_content = tokio::fs::read_to_string(file_path).await.unwrap();
        assert_eq!(saved_content, content);
    }

    #[tokio::test]
    async fn test_fetch_html() {
        // This test requires an internet connection
        let url = Url::parse("https://example.com").unwrap();
        let result = fetch_html(&url).await.unwrap();
        assert!(result.contains("<html"));
        assert!(result.contains("</html>"));
    }

    #[tokio::test]
    async fn test_fetch_html_not_found() {
        let url = Url::parse("https://example.com/not-found").unwrap();
        let result = fetch_html(&url).await;
        assert!(result.is_ok()); // Even 404 pages return HTML
    }
}
