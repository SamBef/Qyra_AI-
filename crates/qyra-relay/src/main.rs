use axum::{Router, routing::get};
use tracing::info;
use qyra_core::config::AppConfig;

mod routes;
mod db;
mod middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "qyra_relay=debug,tower_http=info".into()),
        )
        .json()
        .init();

    info!("🚀 Starting Qyra Relay Server");

    // Load config from environment
    let config = AppConfig::load()?;
    info!(port = config.port, env = %config.app_env, "Config loaded");

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check));

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
