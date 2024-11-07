use std::panic;
use axum::{
    http::Method,
    routing::get,
    Router
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use ygo_draft_server::{
    routes::*,
    card::CardPool
};

#[tokio::main]
async fn main() {
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

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await.expect("TcpListener should work w/ hard-coded localhost");
    let cors = CorsLayer::new()
        .allow_methods(Method::GET)
        .allow_origin(Any);
    let router = Router::new()
        .route("/", get(handler_root))
        .route("/main", get(get_main_opts))
        .route("/extra", get(get_extra_opts))
        .fallback(handler_404)
        .with_state(card_pool)
        .layer(cors);
    info!("router created");

    info!("listening on TCPListenter: {:?}", tcp_listener);
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("Must be able to run the server (std::io::error)");

}