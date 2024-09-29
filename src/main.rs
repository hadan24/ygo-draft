use axum::{response::IntoResponse, routing::get, Json, Router};
use ygo_draft::{get_cards, CardPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pool = CardPool::new(get_cards().await?);
    
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await.unwrap();
    let app = Router::new()
        .route("/", get(|| async {"Hallo :D 🦀"} ))
        .route("/main", get(|| async {Json(pool.main_deck).into_response()} ))
        .route("/extra", get(|| async {Json(pool.extra_deck).into_response()} ));

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}
