use anyhow::Result;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod middleware;
mod routes;
mod state;
mod storage;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "writeup_server=info,tower_http=info".parse().unwrap()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "/etc/writeup-platform/config.toml".to_string());

    let cfg = config::Config::from_file(&config_path)?;
    let addr = format!("{}:{}", cfg.server.host, cfg.server.port);

    let state = state::AppState::new(cfg).await?;
    let app = routes::build_router(Arc::new(state));

    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
