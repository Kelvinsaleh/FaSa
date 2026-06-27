use std::env;
use std::net::TcpListener;

fn main() {
    // Render passes a dynamic port via the PORT environment variable. 
    // If it's not set locally, default to 10000.
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let address = format!("0.0.0.0:{}", port);

    println!("Starting server on {}...", address);

    // Bind to the port to keep the application running indefinitely
    let listener = TcpListener::bind(&address).expect("Failed to bind to port");

    println!("Server is live! Listening for incoming connections...");

    // This loop keeps the process alive forever handling incoming requests
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("New connection established!");
                // Later on, you'll pass this to your HTTP framework routes
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}