use axum::{extract::State, Json};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use validator::Validate;

use crate::{
    error::AppError,
    jwt::{authenticate, issue_token},
    models::{AuthResponse, LoginRequest, SignUpRequest, User, UserResponse},
    password::{hash_password, verify_password},
    state::AppState,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(format_validation_errors(&e)))?;

    let email = payload.email.trim().to_lowercase();

    let existing = sqlx::query_scalar!("SELECT id FROM users WHERE email = $1", email)
        .fetch_optional(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::EmailTaken);
    }

    let password_hash = hash_password(&payload.password)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, password_hash, created_at, updated_at
        "#,
        payload.name.trim(),
        email,
        password_hash
    )
    .fetch_one(&state.db)
    .await?;

    let token = issue_token(user.id, &user.email, &state.jwt_secret, state.jwt_expiry_hours)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(format_validation_errors(&e)))?;

    let email = payload.email.trim().to_lowercase();

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password_hash, created_at, updated_at FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    let valid = verify_password(&payload.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = issue_token(user.id, &user.email, &state.jwt_secret, state.jwt_expiry_hours)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn me(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<UserResponse>, AppError> {
    let auth = authenticate(bearer.token(), &state.jwt_secret)?;

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password_hash, created_at, updated_at FROM users WHERE id = $1"#,
        auth.user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::Unauthorized)?;

    Ok(Json(user.into()))
}

fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    errors
        .field_errors()
        .into_iter()
        .flat_map(|(_, errs)| errs.iter())
        .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
        .collect::<Vec<_>>()
        .join(", ")
}
