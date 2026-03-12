//! HTTP handlers for the AZC registry.

mod packages;
mod users;

pub use packages::*;
pub use users::*;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::PgPool;
use crate::models::ErrorResponse;
use crate::storage::Storage;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub storage: Storage,
}

/// Error type for API responses
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorResponse {
                error: self.message,
                detail: None,
            }),
        )
            .into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError {
                status: StatusCode::NOT_FOUND,
                message: "Resource not found".to_string(),
            },
            _ => ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Database error".to_string(),
            },
        }
    }
}