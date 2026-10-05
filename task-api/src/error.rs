
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// This is the same "custom error enum" pattern from your AppError exercise,
// just using `thiserror` instead of hand-writing `impl fmt::Display`.
// `#[error("...")]` generates the Display impl for you — same end result,
// less boilerplate. `#[from]` auto-generates a `From<sqlx::Error> for AppError`
// impl, which is what makes `?` work directly on sqlx calls (see handlers.rs).
#[derive(Debug, Error)]
pub enum AppError {
    #[error("task not found")]
    NotFound,

    #[error("invalid input: {0}")]
    Validation(String),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

// This is the axum-specific piece: implementing `IntoResponse` tells axum
// how to turn our error type into an actual HTTP response, so handlers
// can just return `Result<T, AppError>` and axum handles the rest.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Database(_) => {
                // Never leak raw database error details to the client —
                // log them server-side (in a real app) and return a generic message.
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
