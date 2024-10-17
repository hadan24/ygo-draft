use axum::{response::IntoResponse, routing::get, Json, Router};
use ygo_draft::CardPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pool = CardPool::new().await?;
    let main_opts = CardPool::generate_draft_options(&pool.main_deck);
    let extra_opts = CardPool::generate_draft_options(&pool.extra_deck);
    
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await.unwrap();
    let app = Router::new()
        .route("/", get(|| async {"Hallo :D 🦀"} ))
        .route("/main", get(|| async {Json(main_opts).into_response()} ))
        .route("/extra", get(|| async {Json(extra_opts).into_response()} ));

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}
