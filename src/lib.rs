use serde_derive::{Deserialize, Serialize};
use std::error::Error;

mod card;

#[derive(Debug, Deserialize, Serialize)]
pub struct YGOProResponse { pub data: Vec<card::Card> }

pub async fn get_cards(req: &str) -> Result<YGOProResponse, Box<dyn Error>> {
    Ok(
        reqwest::get(req)
        .await?
        .json::<YGOProResponse>()
        .await?
    )
}