use std::collections::HashMap;
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct YGOProQuery {
    data: Vec<HashMap<String, Value>>
}

fn main() {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let req = String::from(url) + "name=Tornado Dragon";

    let resp = reqwest::blocking::get(req).unwrap();
    let cards: YGOProQuery = resp.json().unwrap();

    println!("Hello, world!");
    dbg!(cards.data);
}
