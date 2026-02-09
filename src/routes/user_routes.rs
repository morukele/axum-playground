//! user_routes
//! =========
//!
//! Author: oghenemarho
//! Created: 09/02/2026
//! Project: axum-playground
//!
//! Description:
//! A route to hold under information

use crate::errors::ApiError;
use crate::response::ApiResponse;
use crate::AppState;
use axum::extract::State;
use axum::Form;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<LoginData>,
) -> Result<ApiResponse, ApiError> {
    // TODO: make this function secured
    // 1. Get user from db
    // 2. Create refresh token for the user
    // 3. Create claim with user email and id
    // 4. Generate JWT token with claims
    // 5. Add cookie to Jar and return success (redirect??)

    todo!()
}
