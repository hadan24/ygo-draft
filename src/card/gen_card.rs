use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Card {
    pub name:        String,
    pub desc:        String,
    pub card_images: Vec<CardImage>,

    #[serde(alias="race")]
    pub game_type:   GameType,
    #[serde(alias="frameType")]
    pub card_type:   CardType,

    pub atk:         Option<i32>,
    pub def:         Option<i32>,
    pub attribute:   Option<Attribute>,
    pub level:       Option<u32>,
    pub linkval:     Option<u32>,
    pub linkmarkers: Option<Vec<LinkMarker>>,
}

#[derive(Debug, Deserialize)]
pub struct CardImage {
    pub image_url: String,
    pub image_url_small: String,
    pub image_url_cropped: String
}

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
pub enum GameType {
    Aqua,
    Beast,
    Cyberse,
    Dinosaur,
    Dragon,
    Fairy,
    Fiend,
    Fish,
    Insect,
    Machine,
    Plant,
    Psychic,
    Pyro,
    Reptile,
    Rock,
    Spellcaster,
    Thunder,
    Warrior,
    Wyrm,
    Zombie,
    #[serde(alias="Beast-Warrior")]
    BeastWarrior,
    #[serde(alias="Sea Serpent")]
    SeaSerpent,
    #[serde(alias="Winged Beast")]
    WingedBeast,

    // Spell/Trap types
    Normal,
    Continuous,
    Counter,
    Field,
    Equip,
    #[serde(alias="Quick-Play")]
    QuickPlay
}

#[derive(Debug, Deserialize)]
pub enum CardType {
    #[serde(alias="normal")]    Normal,
    #[serde(alias="effect")]    Effect,
    #[serde(alias="fusion")]    Fusion,
    #[serde(alias="synchro")]   Synchro,
    #[serde(alias="xyz")]       Xyz,
    #[serde(alias="link")]      Link,
    #[serde(alias="spell")]     Spell,
    #[serde(alias="trap")]      Trap
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
