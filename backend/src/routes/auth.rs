use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub email: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn signup_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<AuthRequest>,
) -> Result<(StatusCode, Json<String>), StatusCode> {
    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
        payload.email.trim().to_lowercase(),
        hashed_password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok((StatusCode::CREATED, Json("User registered successfully!".to_string()))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn signin_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        payload.email.trim().to_lowercase()
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let my_claims = Claims {
        sub: user.id.to_string(),
        exp: 10000000000,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("super_secret_jwt_key_change_me_later".as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token, email: payload.email }))
}