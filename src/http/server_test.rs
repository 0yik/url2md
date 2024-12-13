use super::*;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_convert_url_success() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/https://example.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/markdown; charset=utf-8"
    );
}

#[tokio::test]
async fn test_convert_url_invalid_url() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/not_a_valid_url")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_convert_url_fetch_error() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/https://nonexistent.example.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_convert_url_encoded() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/https%3A%2F%2Fexample.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/markdown; charset=utf-8"
    );
}
