use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expiry_hours: i64 = env::var("JWT_EXPIRY_HOURS")
            .unwrap_or_else(|_| "168".to_string())
            .parse()
            .expect("JWT_EXPIRY_HOURS must be a number");

        let db = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("failed to connect to database");

        Self {
            db,
            jwt_secret,
            jwt_expiry_hours,
        }
    }
}
