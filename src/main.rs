use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";

    let req = String::from(url) + "name="
        + "Tornado Dragon|"     // given example card
        + "Decode Talker|"      // test w/ Link monster
        + "Pot of Greed|"       // test w/ Spell
        + "Solemn Judgment|"    // test w/ Trap
        + "The Calculator|"     // test w/ `?` stat value
        + "Number F0: Utopic Future";   // test w/ no Level/Rank

    let cards = ygo_draft::get_cards(&req).await?;
    for c in cards.data {
        dbg!(c);
    }

    Ok(())
}
