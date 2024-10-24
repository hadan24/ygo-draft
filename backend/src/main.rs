use axum::{
    routing::get,
    Router
};
use ygo_draft_server::{
    routes::*,
    card::CardPool
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pool = CardPool::new().await?;

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await.unwrap();
    let app = Router::new()
        .route("/", get(handler_root))
        .route("/main", get(get_main_opts))
        .route("/extra", get(get_extra_opts))
        .fallback(handler_404)
        .with_state(pool);

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}
