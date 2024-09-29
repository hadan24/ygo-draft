use std::error::Error;
use serde_derive::{Deserialize, Serialize};
use reqwest::Client;
use crate::card;

#[derive(Debug, Default, Deserialize, Serialize)]
struct YGOProResponse { data: Vec<card::Card> }

// simple query function to test w/, to be commented out
pub async fn get_cards_test() -> Result<Vec<card::Card>, Box<dyn Error>> {
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let req = String::from(url) + "name="
        + "Tornado Dragon|"     // given example card
        + "Decode Talker|"      // test w/ Link monster
        + "Pot of Greed|"       // test w/ Spell
        + "Solemn Judgment|"    // test w/ Trap
        + "The Calculator|"     // test w/ `?` stat value
        + "Number F0: Utopic Future";   // test w/ no Level/Rank

    match reqwest::get(req).await?
            .json::<YGOProResponse>().await
    {
        Ok(resp) => Ok(resp.data),
        Err(e) => {
            eprintln!("{:?}", e);
            Ok(Vec::default())
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CardPool {
    pub main_deck:  Vec<card::Card>,
    pub extra_deck: Vec<card::Card>,
}

impl CardPool {
    pub fn new() -> Self {
        CardPool::default()
    }

    pub async fn get_cards(&self) -> Result<Self, Box<dyn Error>> {
        let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
        let client = reqwest::Client::new();

        let main_deck   = CardPool::get_main(&client, url).await?;
        let extra_deck  = CardPool::get_extra(&client, url).await?;

        Ok(CardPool { main_deck, extra_deck })
    }

    async fn get_main(client: &Client, url: &str)
        -> Result<Vec<card::Card>, Box<dyn Error>>
    {
        let normals = client.get(url)
            .query(&[("level", "lte4"), ("type", "Normal Monster"), ("atk", "gte1900")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;
        let effects = client.get(url)
            .query(&[("level", "lte4"), ("type", "Effect Monster")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;
        let spells = client.get(url)
            .query(&[("type", "Spell Card")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;
        let traps = client.get(url)
            .query(&[("type", "Trap Card")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;

            let mut main = normals;
            main.extend(effects.into_iter());
            main.extend(spells.into_iter());
            main.extend(traps.into_iter());

            Ok(main)
    }

    async fn get_extra(client: &Client, url: &str)
        -> Result<Vec<card::Card>, Box<dyn Error>>
    {
        let fusions = client.get(url)
            .query(&[("type", "Fusion Monster")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;
        let synchros = client.get(url)
            .query(&[("type", "Synchro Monster")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;
        let xyzs = client.get(url)
            .query(&[("type", "Xyz Monster")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;
        let links = client.get(url)
            .query(&[("type", "Link Monster")])
            .send().await?
            .json::<YGOProResponse>().await?
            .data;

        let mut extra = fusions;
        extra.extend(synchros.into_iter());
        extra.extend(xyzs.into_iter());
        extra.extend(links.into_iter());

        Ok(extra)
    }
}
