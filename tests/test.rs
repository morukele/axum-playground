//! test.rs
//! =========
//!
//! Author: oghenemarho
//! Created: 05/02/2026
//! Project: axum-playground
//!
//! Description:
//! A file to test axum routes

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;

fn app() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

#[tokio::test]
async fn test_hello_world() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World!");
}
