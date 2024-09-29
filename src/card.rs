use serde_derive::Serialize;
use crate::queries::ResponseCard;

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