//! User-related handlers.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;
use crate::handlers::{ApiError, AppState};
use crate::models::*;

/// Register a new user
pub async fn register_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    // Validate input
    if req.username.len() < 3 || req.username.len() > 32 {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Username must be between 3 and 32 characters".to_string(),
        });
    }

    if !req.username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Username can only contain alphanumeric characters, underscores, and hyphens".to_string(),
        });
    }

    // Hash password
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|_| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to hash password".to_string(),
        })?;

    // Generate API key
    let api_key = Uuid::new_v4().to_string();

    // Insert user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, api_key)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, password_hash, api_key, created_at
        "#,
        req.username,
        req.email,
        password_hash,
        api_key,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.constraint() == Some("users_username_key") {
                return ApiError {
                    status: StatusCode::CONFLICT,
                    message: "Username already taken".to_string(),
                };
            }
            if db_err.constraint() == Some("users_email_key") {
                return ApiError {
                    status: StatusCode::CONFLICT,
                    message: "Email already registered".to_string(),
                };
            }
        }
        ApiError::from(e)
    })?;

    Ok(Json(UserResponse {
        username: user.username,
        email: user.email,
        api_key: user.api_key,
    }))
}

/// Get user by username
pub async fn get_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<UserPublicResponse>, ApiError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, api_key, created_at
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| ApiError {
        status: StatusCode::NOT_FOUND,
        message: format!("User '{}' not found", username),
    })?;

    // Get user's packages
    let packages = sqlx::query!(
        r#"
        SELECT name FROM packages WHERE owner_id = $1 LIMIT 10
        "#,
        user.id,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(UserPublicResponse {
        username: user.username,
        packages: packages.into_iter().map(|p| p.name).collect(),
    }))
}

/// Response for user registration
#[derive(Debug, serde::Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
    pub api_key: String,
}

/// Public user response
#[derive(Debug, serde::Serialize)]
pub struct UserPublicResponse {
    pub username: String,
    pub packages: Vec<String>,
}