use axum::{
    Json, http::StatusCode, response::{IntoResponse, Response},
};

use  serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]

pub enum AppError {
    #[error("task not found")]
    NotFound,

    #[error("invalid input: {0}")]
    Validation(String),

    #[error("database error: {0}")]
    DataBase(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound=> (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(_)=> (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::DataBase(_)=> {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let  body = Json(json!({"error" : message}));

        (status,body).into_response()
    }   
}