use std::error::Error;
use serde::Serialize;
use crate::queries::{ResponseCard, get_cards};

#[derive(Clone, Debug, Serialize)]
pub struct Card {
    pub name: String,
    pub img_link: String
}

impl Card {
    pub fn new(c: ResponseCard) -> Self {
        Card {
            name: c.name,
            img_link: c.card_images[0].image_url.clone()
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CardPool {
    pub main_deck:  Vec<Card>,
    pub extra_deck: Vec<Card>,
}

impl CardPool {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let (main_deck, extra_deck) = get_cards().await?;

        Ok(CardPool {
            main_deck:  main_deck.into_iter().map(Card::new).collect(),
            extra_deck: extra_deck.into_iter().map(Card::new).collect()
        })
    }
    
    pub fn generate_draft_options(pool: &[Card]) -> (Card, Card, Card) {
        let mut rng = rand::thread_rng();
        let i_s = rand::seq::index::sample(&mut rng, pool.len(), 3);

        (
            pool[i_s.index(0)].clone(),
            pool[i_s.index(1)].clone(),
            pool[i_s.index(2)].clone()
        )
    }
}
/*
#[derive(Debug, Deserialize)]
pub enum Attribute {
    #[serde(alias="FIRE")]  Fire,
    #[serde(alias="WATER")] Water,
    #[serde(alias="EARTH")] Earth,
    #[serde(alias="WIND")]  Wind,
    #[serde(alias="DARK")]  Dark,
    #[serde(alias="LIGHT")] Light
}

#[derive(Debug, Deserialize)]
pub enum LinkMarker {
    Top,
    Bottom,
    Left,
    Right,
    #[serde(alias="Bottom-Left")]   BottomLeft,
    #[serde(alias="Bottom-Right")]  BottomRight,
    #[serde(alias="Top-Left")]      TopLeft,
    #[serde(alias="Top-Right")]     TopRight
}
*/