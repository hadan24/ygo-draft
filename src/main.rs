fn main() {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";

    let test_reqs = vec![
        String::from(url) + "name=Decode Talker",
        String::from(url) + "name=Pot of Greed",
        String::from(url) + "name=Solemn Judgment",
        String::from(url) + "name=Tornado Dragon",
    ];

    for req in test_reqs {
        let cards = ygo_draft::get_cards(&req);
        dbg!(cards.data);
    }
}
