use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct YGOProResponse {
    data: Vec<Card>
}

// might need to create diff structs for diff cards
/*
    ALL cards: id, name, type (normal/eff/spell/trap/etc), frameType, desc (flavor/eff text), ygoprodeck_url
    monsters: atk, def, level, race (dragon/insect/etc), attribute
    s/t: race (field/continuous/etc)

    other: archetype, scale (pend), linkval, linkmarkers, 
*/
#[derive(Deserialize, Debug)]
struct Card {
    name: String,
    desc: String,
    race: String,
    frameType: String,  // MUST be camel-case for serde to recognize field
    atk: Option<u32>,
    def: Option<u32>,
    level: Option<u32>,
    attribute: Option<String>,
    linkval: Option<u32>,
    linkmarkers: Option<Vec<String>>,
    card_images: Vec<CardImage>,
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
    let cards: YGOProResponse = resp.json().expect("No card returned");

    dbg!(cards.data);
    println!("------------------------------------------------------");
    println!("Hello, world!");
}
