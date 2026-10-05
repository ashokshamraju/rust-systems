use axum::{
    Json, extract::{Path, State},
};

use sqlx::{SqlitePool};

use crate::error::AppError;
use crate::models::{CreateTask, Task, UpdateTask};

pub async fn list_tasks(
    State(pool): State<SqlitePool>
    )->Result<Json<Vec<Task>>, AppError> {
    
    let tasks = sqlx::query_as::<_,Task>("SELECT id, title, body,pinned FROM TASKS")
            .fetch_all(&pool)
            .await?;
    
    Ok(Json(tasks))
}

pub async fn get_task(State(pool): State<SqlitePool>,
    Path(id): Path<i64>
    )->Result<Json<Task>,AppError>{
    
    let task = sqlx::query_as::<_,Task>("SELECT id, title,body,pinned FROM tasks WHERE id=?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

        Ok(Json(task))
}

pub async fn create_task(State(pool): State<SqlitePool>,
        Json(payload): Json<CreateTask>)->Result<Json<Task>,AppError> {
    
    if payload.title.trim().is_empty() {
        return Err(AppError::Validation("title cannot be empty".into()));
    }

    if payload.body.trim().is_empty() {
        return Err(AppError::Validation("body acnnot be empty".into()));
    }

    let id = sqlx::query("INSERT INTO tasks(title,body,pinned) VALUES (?,?,false)")
        .bind(&payload.title)
        .bind(&payload.body)
        .execute(&pool)
        .await?
        .last_insert_rowid();

    Ok(Json(Task { id, title: payload.title, body: payload.body, pinned: false }))
}

pub async fn update_task(State(pool): State<SqlitePool>,
        Path(id): Path<i64>,
        Json(payload): Json<UpdateTask>,
    )->Result<Json<Task>, AppError> {
    
    let existing = sqlx::query_as::<_,Task>("SELECT id, title, body, pinned FROM tasks WHERE id = ?")
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let title = payload.title.unwrap_or(existing.title);
    let body = payload.body.unwrap_or(existing.body);
    let pinned = payload.pinned.unwrap_or(existing.pinned);

    sqlx::query("UPDATE  tasks SET title = ?, body = ?, pinned = ? WHERE id = ?")
        .bind(&title)
        .bind(&body)
        .bind(pinned)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(Task { id, title, body, pinned}))
}

pub async fn delete_task(State(pool): State<SqlitePool>,
        Path(id): Path<i64>,
    )->Result<(), AppError> {
    
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}