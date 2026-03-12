//! Authentication utilities.

use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

/// Create a JWT token
pub fn create_token(user_id: &str, username: &str, secret: &str) -> Result<String> {
    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Validate a JWT token
pub fn validate_token(token: &str, secret: &str) -> Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
