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
    Forbidden,
    Unauthorised,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            Self::Forbidden => StatusCode::FORBIDDEN.into_response(),
            Self::Unauthorised => StatusCode::UNAUTHORIZED.into_response(),
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
