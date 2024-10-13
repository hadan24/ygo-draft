use axum::{response::IntoResponse, routing::get, Json, Router};
use ygo_draft::{get_cards, CardPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pool = CardPool::new(get_cards().await?);
    let main_opts = pool.generate_main_deck_options();
    let extra_opts = pool.generate_extra_deck_options();
    
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await.unwrap();
    let app = Router::new()
        .route("/", get(|| async {"Hallo :D 🦀"} ))
        .route("/main", get(|| async {Json(main_opts).into_response()} ))
        .route("/extra", get(|| async {Json(extra_opts).into_response()} ));

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}
