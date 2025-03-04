use serde::Serialize;
use tracing::{debug, trace};
use crate::queries::{ResponseCard, get_cards};

#[derive(Clone, Debug, Serialize)]
pub struct Card {
    pub id:     u32,
    pub name:   String,
    pub img_link: String
}

impl Card {
    pub fn new(c: ResponseCard) -> Self {
        Card {
            id: c.id,
            name: c.name,
            img_link: c.card_images[0].image_url.clone()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CardPool {
    pub main_deck:  Vec<Card>,
    pub extra_deck: Vec<Card>,
}
pub type DraftOptions = [Card; 3];

impl CardPool {
    /*  reqwest could fail if:
        - something went wrong while sending the YGOPro API request
            if e.is_request(), "check request builder and queries"
        - a redirect loop was detected
        - the redirect limit was exhaused
            if e.is_timeout() || e.is_redirect, "check YGOPro API status"
        - the response body is not JSON
            if e.is_body(), "no JSON response, check YGOPro API docs"
        - the JSON could not be deserialized into a <YGOProResponse>
            if e.is_decode(), "check queries or YGOPro API docs"
    */
    pub async fn new_from_ygopro() -> Result<Self, reqwest::Error> {
        let (main_deck, extra_deck) = get_cards().await?;

        Ok(CardPool {
            main_deck:  main_deck.into_iter().map(Card::new).collect(),
            extra_deck: extra_deck.into_iter().map(Card::new).collect()
        })
    }
    
    pub fn generate_draft_options(pool: &[Card]) -> DraftOptions {
        let mut rng = rand::rng();
        let i_s = rand::seq::index::sample(&mut rng, pool.len(), 3)
            .into_vec();
        let options = [
            pool[i_s[0]].clone(),
            pool[i_s[1]].clone(),
            pool[i_s[2]].clone()
        ];

        debug!("indices: {}, {}, {}", i_s[0], i_s[1], i_s[2]);
        trace!("card names: {:?}", options);
        options
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