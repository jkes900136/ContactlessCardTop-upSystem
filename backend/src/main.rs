use axum::Router;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

mod config;
mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::Config::from_env();

    // Setup database connection pool
    let pool = db::init_db_pool(&config.database_url).await?;

    // Build router
    let app = Router::new()
        .nest("/api", routes::api_routes())
        .layer(TraceLayer::new___()); // Note: Simplified for initial structure

    let addr = SocketAddr::from_str(&config.server_addr).unwrap();
    println!("Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
