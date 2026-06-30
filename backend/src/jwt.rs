use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn issue_token(user_id: Uuid, email: &str, secret: &str, expiry_hours: i64) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::hours(expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::from)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

/// Authenticated user info, derived from a verified Bearer token.
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

/// Verifies a raw bearer token string and returns the authenticated user.
/// Handlers extract the token themselves via `TypedHeader<Authorization<Bearer>>`
/// and pass `bearer.token()` in here -- avoids implementing a custom
/// FromRequestParts extractor, which has fragile lifetime requirements.
pub fn authenticate(bearer_token: &str, secret: &str) -> Result<AuthUser, AppError> {
    let claims = verify_token(bearer_token, secret)?;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;
    Ok(AuthUser {
        user_id,
        email: claims.email,
    })
}
