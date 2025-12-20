use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "loafy_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Loafy Club API server...");

    // Get configuration from environment
    let port = std::env::var("API_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    // TODO: Initialize database pool
    // let database_url = std::env::var("DATABASE_URL")?;
    // let pool = loafy_db::create_pool(&database_url).await?;

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
