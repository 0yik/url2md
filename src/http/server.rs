use axum::{
    extract::Path,
    routing::{get},
    Router,
    response::Response,
    http::{StatusCode, header},
    body::Body,
};
use serde::Serialize;
use url::Url;
use crate::converter::markdown_converter::MarkdownConverter;
use super::client::fetch_html;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub fn create_router() -> Router {
    Router::new()
        .route("/*url", get(convert_url))
}

async fn convert_url(
    Path(url): Path<String>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Get the full URL by removing the leading slash and handling protocol
    let url_str = if url.starts_with('/') {
        url[1..].to_string()
    } else {
        url
    };

    // Handle unencoded URLs by checking for http:// or https://
    let parsed_url = if url_str.contains("://") {
        match Url::parse(&url_str) {
            Ok(url) => url,
            Err(e) => return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid URL: {}", e),
            )),
        }
    } else {
        // Try to decode if it's URL-encoded
        match urlencoding::decode(&url_str) {
            Ok(decoded) => match Url::parse(&decoded) {
                Ok(url) => url,
                Err(e) => return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Invalid URL after decoding: {}", e),
                )),
            },
            Err(e) => return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid URL encoding: {}", e),
            )),
        }
    };

    // Fetch HTML content
    let html = match fetch_html(&parsed_url).await {
        Ok(content) => content,
        Err(e) => return Err((
            StatusCode::BAD_REQUEST,
            format!("Failed to fetch URL: {}", e),
        )),
    };

    // Convert to markdown
    let converter = MarkdownConverter::new();
    match converter.convert(&html) {
        Ok(markdown) => {
            let response = Response::builder()
                .header(header::CONTENT_TYPE, "text/markdown; charset=utf-8")
                .body(Body::from(markdown))
                .unwrap();
            Ok(response)
        },
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

#[cfg(test)]
#[path = "server_test.rs"]
mod tests;
