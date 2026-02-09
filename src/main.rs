mod errors;
mod logger;
mod response;
mod routes;

use crate::errors::ApiError;
use crate::response::ApiResponse;
use axum::routing::{delete, get, post, put};
use axum::Router;
use dotenv::dotenv;
use routes::todo_routes::{create_todo, delete_todo, edit_todo, get_todo, get_todos};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/todo", post(create_todo))
        .route("/todo", get(get_todos))
        .route("/todo/{id}", get(get_todo))
        .route("/todo/{id}", put(edit_todo))
        .route("/todo/{id}", delete(delete_todo))
        .with_state(state)
}

pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() {
    // Initialize env variables
    dotenv().ok();

    // Initialize tracing + log bridging
    logger::init_tracing();

    // read db connection string from .env file
    let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
        tracing::warn!("Unable to use .env config, with error : {}", e);
        "postgres://user:password@localhost/todo".to_string()
    });

    // create db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .map_err(|e| tracing::error!("Unable to create database pool, with error: {}", e))
        .expect("Unable to connect to the database");

    // migrate schema to database
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| tracing::error!("Unable to migrate database, with error: {}", e))
        .expect("Unable to migrate database schema");

    let app_state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .merge(init_router(app_state))
        // Add a TraceLayer to automatically create and enter spans
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| tracing::error!("Unable to bind TCP listener, with error: {}", e))
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap()); // structured logging

    axum::serve(listener, app).await.unwrap();
}
