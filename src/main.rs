mod custom_middleware;
mod errors;
mod extractors;
mod response;
mod todo_routes;
mod web_socket;

use crate::todo_routes::{create_todo, delete_todo, edit_todo, get_todo, get_todos};
use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

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
    // read db connection string from .env file
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/todo".to_string());

    // create db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // migrate schema to database
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("unable to migrate database schema");

    let app_state = Arc::new(AppState { db: pool });

    let app = Router::new().merge(init_router(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
