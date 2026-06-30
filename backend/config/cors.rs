use axum::http::{header, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn build_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec![
            "http://localhost:5173".parse().unwrap(),
            "https://fa-sa-seven.app.vercel.app".parse().unwrap(),
        ]))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
}