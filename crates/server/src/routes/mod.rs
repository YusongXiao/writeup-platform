pub mod account;
pub mod admin;
pub mod game;
pub mod writeup;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::Method,
    routing::get,
};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::state::AppState;

pub fn build_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    Router::new()
        .nest("/api/account", account::router())
        .nest("/api/games", game::router())
        .nest("/api/writeup", writeup::router())
        .nest("/api/admin", admin::router())
        .route("/api/ping", get(|| async { "pong" }))
        .layer(DefaultBodyLimit::max(55 * 1024 * 1024))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
