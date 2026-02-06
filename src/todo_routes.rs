//! todo_routes
//! =========
//!
//! Author: oghenemarho
//! Created: 05/02/2026
//! Project: axum-playground
//!
//! Description:
//!

use crate::errors::ApiError;
use crate::response::ApiResponse;
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid, // generate on server not DB
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub created_at: Option<DateTime<Utc>>, // DB handles this
    pub updated_at: Option<DateTime<Utc>>, // DB handles this
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status")]
pub enum Status {
    Completed,  // Task completed
    InProgress, // Task in progress
    NotStarted, // Task not started
    Deleted,    // change status to delete to enable recovery if needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoObject {
    pub name: String,
    pub description: Option<String>,
    pub status: Option<Status>,
}

#[axum::debug_handler]
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateTodoObject>,
) -> Result<ApiResponse, ApiError> {
    // store item in db
    let todo = sqlx::query_as!(
        Todo,
        r#"
            INSERT INTO todos (id, name, description, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, description, status AS "status!: Status", created_at, updated_at
        "#,
        Uuid::new_v4(),
        body.name,
        body.description,
        body.status.unwrap_or(Status::NotStarted) as Status,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?;

    // return response with created todo
    Ok(ApiResponse::JsonData(vec![todo]))
}
pub async fn get_todos() -> Result<ApiResponse, ApiError> {
    todo!()
}

pub async fn get_todo(Path(id): Path<Uuid>) {}

pub async fn edit_todo(Path(id): Path<Uuid>, Json(body): Json<Todo>) {}

pub async fn delete_todo(Path(id): Path<Uuid>) {}
