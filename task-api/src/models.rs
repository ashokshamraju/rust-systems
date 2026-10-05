use serde::{Deserialize, Serialize};

// The shape of a task as stored/returned. `sqlx::FromRow` lets us decode
// a database row directly into this struct.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

// Separate "input" types are idiomatic here: the caller shouldn't be able
// to set `id` or `done` when creating a task, so we don't reuse `Task` itself.
#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub description: String,
}

// All fields optional, since a PATCH-style update might only change one field.
#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    pub description: Option<String>,
    pub done: Option<bool>,
}
