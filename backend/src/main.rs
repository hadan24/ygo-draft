use axum::{
    routing::get,
    Router
};
use tracing::info;
use ygo_draft_server::{
    routes::*,
    card::CardPool
};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "ygo-draft-server=info".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_line_number(true)
        .with_writer(std::io::stderr)
        .pretty()
        .init();

    let pool = CardPool::new().await?;
    info!("cardpool created");

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await.expect("TcpListener should work w/ hard-coded localhost");
    let app = Router::new()
        .route("/", get(handler_root))
        .route("/main", get(get_main_opts))
        .route("/extra", get(get_extra_opts))
        .fallback(handler_404)
        .with_state(pool);
    info!("router created");

    info!("listening on {:?}", tcp_listener);
    axum::serve(tcp_listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("Must be able to run the server (std::io::error)");

    Ok(())
}