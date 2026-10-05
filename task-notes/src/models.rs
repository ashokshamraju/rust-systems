use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub pinned: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub body: Option<String>,
    pub pinned: Option<bool>,
}