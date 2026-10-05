use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::{CreateTask, Task, UpdateTask};

// `State<SqlitePool>` is axum's dependency-injection mechanism: the pool we
// build once in main() gets handed to every handler automatically. This is
// the same Arc<T>-backed sharing you've already used for threads/async —
// axum's State wraps it for you under the hood.
pub async fn list_tasks(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Task>>, AppError> {
    // query_as! is checked against the real database schema at compile time
    // (given a DATABASE_URL at build time) — a typo in a column name is a
    // compile error, not a runtime surprise.
    let tasks = sqlx::query_as::<_, Task>("SELECT id, description, done FROM tasks")
        .fetch_all(&pool)
        .await?; // sqlx::Error -> AppError via the #[from] impl in error.rs

    Ok(Json(tasks))
}

pub async fn get_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>("SELECT id, description, done FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?; // Option<Task> -> Result<Task, AppError>

    Ok(Json(task))
}

pub async fn create_task(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    if payload.description.trim().is_empty() {
        return Err(AppError::Validation("description cannot be empty".into()));
    }

    let id = sqlx::query("INSERT INTO tasks (description, done) VALUES (?, false)")
        .bind(&payload.description)
        .execute(&pool)
        .await?
        .last_insert_rowid();

    Ok(Json(Task {
        id,
        description: payload.description,
        done: false,
    }))
}

pub async fn update_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    // Fetch first so we 404 correctly instead of silently updating 0 rows.
    let existing = sqlx::query_as::<_, Task>("SELECT id, description, done FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // `unwrap_or` here mirrors the Option-based partial update: keep the old
    // value for any field the caller didn't send.
    let description = payload.description.unwrap_or(existing.description);
    let done = payload.done.unwrap_or(existing.done);

    sqlx::query("UPDATE tasks SET description = ?, done = ? WHERE id = ?")
        .bind(&description)
        .bind(done)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(Task { id, description, done }))
}

pub async fn delete_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
