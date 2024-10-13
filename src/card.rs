use serde_derive::Serialize;
use crate::queries::ResponseCard;
use rand::{self, Rng};

#[derive(Clone, Debug, Serialize)]
pub struct Card {
    name: String,
    img_link: String
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
    pub fn new((main, extra): (Vec<ResponseCard>, Vec<ResponseCard>))
        -> Self
    {
        CardPool {
            main_deck:  main.into_iter().map(Card::new).collect(),
            extra_deck: extra.into_iter().map(Card::new).collect()
        }
    }
    
    pub fn generate_main_deck_options(self: &CardPool) -> (Card, Card, Card) {
        let mut rng = rand::thread_rng();
        let pool = &self.main_deck;

        let mut i_s = [rng.gen_range(0..pool.len()), 0, 0];
        
        while i_s[0] == i_s[1] || i_s[0] == i_s[2] || i_s[1] == i_s[2] {
            i_s[1] = rng.gen_range(0..pool.len());
            i_s[2] = rng.gen_range(0..pool.len());
        }
        
        (pool[i_s[0]].clone(), pool[i_s[1]].clone(), pool[i_s[2]].clone())
    }
    
    pub fn generate_extra_deck_options(self: &CardPool) -> (Card, Card, Card) {
        let mut rng = rand::thread_rng();
        let pool = &self.extra_deck;

        let mut i_s = [rng.gen_range(0..pool.len()), 0, 0];
        
        while i_s[0] == i_s[1] || i_s[0] == i_s[2] || i_s[1] == i_s[2] {
            i_s[1] = rng.gen_range(0..pool.len());
            i_s[2] = rng.gen_range(0..pool.len());
        }
        
        (pool[i_s[0]].clone(), pool[i_s[1]].clone(), pool[i_s[2]].clone())
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