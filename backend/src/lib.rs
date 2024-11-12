pub mod queries;
pub mod card;
pub mod routes;

use card::CardPool;
use routes::*;

use std::panic;
use axum::{
    http::Method,
    routing::get,
    Router
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;


pub async fn create_router() -> Router {
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_line_number(true)
        .with_writer(std::io::stderr)
        .pretty()
        .init();

    let card_pool = match CardPool::new_from_ygopro().await {
        Ok(p) => p,
        Err(e) => panic!("Couldn't create cardpool: {:?}", e)
    };
    info!("cardpool created");
    
    let cors = CorsLayer::new()
        .allow_methods(Method::GET)
        .allow_origin(Any);
    

    Router::new()
        .route("/", get(handler_root))
        .route("/main", get(main_opts))
        .route("/extra", get(extra_opts))
        .fallback(handler_404)
        .with_state(card_pool)
        .layer(cors)
}