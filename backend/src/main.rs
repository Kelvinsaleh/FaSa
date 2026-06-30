use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

mod config;
mod routes;

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Neon database");

    println!("Database connection verified! 🚀");

    // Build routes using handlers from our modular files
    let app = Router::new()
        .route("/api/auth/signup", post(routes::auth::signup_handler))
        .route("/api/auth/signin", post(routes::auth::signin_handler))
        .with_state(pool)
        .layer(config::cors::build_cors_layer()); // Inject modular CORS settings

    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address format");

    println!("Server operating securely on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}