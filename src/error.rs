use axum::(http::StatusCode, response::(IntoResponse, Response), Json);
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError(
    #[error("Unauthorized: (0)")]
    Unauthorized(String),
    #[error("Bad request: [0]")]
    BadRequest(String),
    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
)

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) - match self {
            Self::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m),
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            _ => {
                tracing::error!("{:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string())
            }
        };
        (status, Json(json!({"error":msg}))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;