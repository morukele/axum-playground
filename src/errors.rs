//! errors
//! =========
//!
//! Author: oghenemarho
//! Created: 06/02/2026
//! Project: axum-playground
//!
//! Description:
//! A file containing all the errors of the API route

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ApiError {
    BadRequest,
    InternalServerError,
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
