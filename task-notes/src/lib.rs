pub mod db;
pub mod error;
pub mod handlers;
pub mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::SqlitePool;

pub fn app_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/tasks", get(handlers::list_tasks))
        .route("/tasks", post(handlers::create_task))
        .route("/tasks/:id", get(handlers::get_task))
        .route("/tasks/:id", put(handlers::update_task))
        .route("/tasks/:id", delete(handlers::delete_task))
        .with_state(pool)
}