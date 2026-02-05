//! user_routes.rs
//! =========
//!
//! Author: oghenemarho
//! Created: 05/02/2026
//! Project: axum-playground
//!
//! Description:
//!
use axum::routing::get;
use axum::Router;

async fn get_users() {}
async fn get_user() {}

pub fn users_routers() -> Router {
    Router::new()
        .without_v07_checks()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
}
