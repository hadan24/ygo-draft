use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct YGOProResponse {
    data: Vec<Card>
}

#[derive(Debug, Deserialize)]
struct Card {
    name:        String,
    desc:        String,
    race:        CardType,

    #[serde(alias="frameType")]
    frame_type:  FrameType,

    card_images: Vec<CardImage>,
    atk:         Option<u32>,
    def:         Option<u32>,
    attribute:   Option<Attribute>,
    level:       Option<u32>,
    linkval:     Option<u32>,
    linkmarkers: Option<Vec<LinkMarker>>,
}

#[derive(Debug, Deserialize)]
struct CardImage {
    image_url: String,
    image_url_small: String,
    image_url_cropped: String
}

// Variants are in all caps to match JSON
#[derive(Debug, Deserialize)]
enum Attribute {
    #[serde(alias="FIRE")]  Fire,
    #[serde(alias="WATER")] Water,
    #[serde(alias="EARTH")] Earth,
    #[serde(alias="WIND")]  Wind,
    #[serde(alias="DARK")]  Dark,
    #[serde(alias="LIGHT")] Light
}

#[derive(Debug, Deserialize)]
enum CardType { // In-game "type"
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

    Normal,
    Continuous,
    Counter,
    Field,
    Equip,

    #[serde(alias="Quick-Play")]
    QuickPlay
}

// Variants all lower to match JSON
#[derive(Debug, Deserialize)]
enum FrameType { // In-game card type
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
enum LinkMarker {
    Top,
    Bottom,
    Left,
    Right,
    #[serde(alias="Bottom-Left")]   BottomLeft,
    #[serde(alias="Bottom-Right")]  BottomRight,
    #[serde(alias="Top-Left")]      TopLeft,
    #[serde(alias="Top-Right")]     TopRight
}

fn main() {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let decode_req = String::from(url) + "name=Decode Talker";
    let pot_req = String::from(url) + "name=Pot of Greed";
    let tornado_req = String::from(url) + "name=Tornado Dragon";

    let resp = reqwest::blocking::get(decode_req).unwrap();
    let cards: YGOProResponse = resp.json().unwrap();
    dbg!(cards.data);

    let resp = reqwest::blocking::get(pot_req).unwrap();
    let cards: YGOProResponse = resp.json().unwrap();
    dbg!(cards.data);

    let resp = reqwest::blocking::get(tornado_req).unwrap();
    let cards: YGOProResponse = resp.json().unwrap();
    dbg!(cards.data);
}
