//! todo_routes
//! =========
//!
//! Author: oghenemarho
//! Created: 05/02/2026
//! Project: axum-playground
//!
//! Description:
//! Routes for the todo_handler

use crate::errors::ApiError;
use crate::response::ApiResponse;
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

//TODO: implement logging for the routes

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
    sqlx::query_as!(
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

    Ok(ApiResponse::Created)
}

#[axum::debug_handler]
pub async fn get_todos(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
            SELECT id, name, description, status AS "status!: Status", created_at, updated_at
            FROM todos
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| ApiError::NotFound)?;

    Ok(ApiResponse::JsonData(todos))
}

pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse, ApiError> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
            SELECT id, name, description, status AS "status!: Status", created_at, updated_at
            FROM todos
            WHERE id = $1
            "#,
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::NotFound)?;

    Ok(ApiResponse::JsonData(vec![todo]))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoObject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
}

#[axum::debug_handler]
pub async fn edit_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTodoObject>,
) -> Result<ApiResponse, ApiError> {
    // find item and update item
    let res = sqlx::query!(
        r#"
            UPDATE todos
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                status = COALESCE($3, status)
            WHERE id = $4
        "#,
        body.name,
        body.description,
        body.status as Option<Status>,
        id
    )
    .execute(&state.db)
    .await
    .map_err(|_| ApiError::NotFound)?;

    if res.rows_affected() > 0 {
        Ok(ApiResponse::Ok)
    } else {
        Err(ApiError::NotFound)
    }
}

#[axum::debug_handler]
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse, ApiError> {
    // find and delete items
    let res = sqlx::query!(
        r#"
            DELETE FROM todos
            WHERE id = $1
        "#,
        id
    )
    .execute(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?;

    if res.rows_affected() > 0 {
        Ok(ApiResponse::NoContent)
    } else {
        Err(ApiError::NotFound)
    }
}
