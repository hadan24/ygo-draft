fn main() {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";

    let req = String::from(url) + "name="
        + "Decode Talker|"
        + "Pot of Greed|"
        + "Solemn Judgment|"
        + "Tornado Dragon|"
        + "The Calculator|"
        + "Number F0: Utopic Future";

    let cards = ygo_draft::get_cards(&req);
    dbg!(cards.data);
}
