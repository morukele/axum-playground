//! response
//! =========
//!
//! Author: oghenemarho
//! Created: 06/02/2026
//! Project: axum-playground
//!
//! Description:
//! A file containing all the response of the API route

use crate::todo_routes::Todo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Todo>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::Created => StatusCode::CREATED.into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}
