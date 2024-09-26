use std::error::Error;
use axum::{response::IntoResponse, routing::get, Json, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let req = String::from(url) + "name="
        + "Tornado Dragon|"     // given example card
        + "Decode Talker|"      // test w/ Link monster
        + "Pot of Greed|"       // test w/ Spell
        + "Solemn Judgment|"    // test w/ Trap
        + "The Calculator|"     // test w/ `?` stat value
        + "Number F0: Utopic Future";   // test w/ no Level/Rank
    let cards = ygo_draft::get_cards(&req).await?;

    
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(|| async {Json(cards.data).into_response()} ));

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}
