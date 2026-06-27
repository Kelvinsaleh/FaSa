use std::env;
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // 1. Fetch the Database URL injected by Render
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    println!("Connecting to Neon PostgreSQL database...");
    
    // 2. Connect to the Neon cluster and verify the connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Neon database");

    println!("Database connection successful! 🎉");

    // 3. Fetch the network port from Render, defaulting to 10000 locally
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let address = format!("0.0.0.0:{}", port);

    println!("Starting server on {}...", address);
    let listener = TcpListener::bind(&address).expect("Failed to bind to network port");

    println!("Server is active and listening for requests!");

    // Keep the application running indefinitely
    for stream in listener.incoming() {
        if let Ok(_stream) = stream {
            println!("Incoming connection detected!");
        }
    }
}