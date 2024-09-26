use serde_derive::Deserialize;

mod card;

#[derive(Debug, Deserialize)]
pub struct YGOProResponse { pub data: Vec<card::Card> }

pub fn get_cards(req: &str) -> YGOProResponse {
    let resp = reqwest::blocking::get(req).unwrap();
    
    resp.json().unwrap()
}