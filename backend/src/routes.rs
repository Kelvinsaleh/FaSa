use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/auth/signup", post(handlers::signup))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/me", get(handlers::me))
}

async fn health() -> &'static str {
    "ok"
}
