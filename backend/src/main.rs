mod error;
mod handlers;
mod jwt;
mod models;
mod password;
mod routes;
mod state;

use std::env;

use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let app_state = AppState::new().await;

    sqlx::migrate!("./migrations")
        .run(&app_state.db)
        .await
        .expect("failed to run migrations");

    // Comma-separated list, e.g. "https://fa-sa-xlqd.vercel.app,http://localhost:5173"
    let frontend_origins = env::var("FRONTEND_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    let allowed_origins: Vec<HeaderValue> = frontend_origins
        .split(',')
        .map(|s| s.trim().trim_end_matches('/'))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<HeaderValue>().expect("invalid FRONTEND_ORIGIN entry"))
        .collect();

    tracing::info!("allowed CORS origins: {:?}", allowed_origins);

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    let app = routes::routes()
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{port}");

    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
