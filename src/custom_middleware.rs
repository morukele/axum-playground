//! middleware
//! =========
//!
//! Author: oghenemarho
//! Created: 05/02/2026
//! Project: axum-playground
//!
//! Description:
//! A collection of middlewares for working with Axum

use axum::extract::Request;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

pub async fn check_hello_world(req: Request, next: Next) -> Result<Response, StatusCode> {
    if req.headers().get(CONTENT_TYPE).unwrap() != "application/json" {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(next.run(req).await)
}
