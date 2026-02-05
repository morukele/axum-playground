mod custom_middleware;
mod extractors;
mod user_routes;
mod web_socket;

use crate::custom_middleware::check_hello_world;
use crate::extractors::JsonOrForm;
use crate::web_socket::websocket_handler;
use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::middleware;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use axum_extra::headers::Origin;
use axum_extra::TypedHeader;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

#[derive(Serialize)]
struct Message {
    message: String,
    origin: String,
}

enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>),
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

enum ApiError {
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

#[derive(Deserialize)]
struct Submission {
    message: String,
}

#[axum::debug_handler]
async fn my_function(
    TypedHeader(origin): TypedHeader<Origin>,
    JsonOrForm(json): JsonOrForm<Submission>,
) -> Result<ApiResponse, ApiError> {
    println!("Json Message: {}", json.message);
    println!("Header Origin: {}", origin.hostname());

    let res = Message {
        message: json.message,
        origin: origin.hostname().to_string(),
    };

    Ok(ApiResponse::JsonData(vec![res]))
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

async fn do_something(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    Err(ApiError::Unauthorised)
}

fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .without_v07_checks()
        .route("/", get(hello_world))
        .route("/do_something", get(do_something))
        .route("/my-function", get(my_function))
        .route("/ws", get(websocket_handler))
        .layer(middleware::from_fn(check_hello_world))
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    api_state: ApiState,
}

#[derive(Clone)]
struct ApiState {}

// support converting an `AppState` into a `ApiState`
impl FromRef<AppState> for ApiState {
    fn from_ref(input: &AppState) -> Self {
        input.api_state.clone()
    }
}

#[tokio::main]
async fn main() {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/database".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app_state = Arc::new(AppState {
        db: pool,
        api_state: ApiState {},
    });

    let app = Router::new()
        // .without_v07_checks()
        .merge(init_router(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
