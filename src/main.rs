use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct YGOProResponse {
    data: Vec<Card>
}

#[derive(Deserialize, Debug)]
struct Card {
    name:        String,
    desc:        String,
    race:        String,
    frameType:   String,  // original JSON mixes camel- & snake-case
    card_images: Vec<CardImage>,
    atk:         Option<u32>,
    def:         Option<u32>,
    level:       Option<u32>,
    attribute:   Option<String>,
    linkval:     Option<u32>,
    linkmarkers: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct CardImage {
    image_url: String,
    image_url_small: String,
    image_url_cropped: String
}

fn main() {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let req = String::from(url) + "name=Pot of Greed";

    let resp = reqwest::blocking::get(req).unwrap();
    let cards: YGOProResponse = resp.json().unwrap();

    dbg!(cards.data);
}
