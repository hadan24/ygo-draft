use std::error::Error;
use serde::Deserialize;
use reqwest::Client;
use tracing::{info, instrument, trace};

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseCard {
    pub id:          u32,
    pub name:        String,
    pub card_images: Vec<CardImage>,
/*
    pub desc:        String,

    #[serde(alias="frameType")]
    pub card_type:   CardType,
    #[serde(alias="race")]
    pub game_type:   GameType,

    pub atk:         Option<i32>,
    pub def:         Option<i32>,
    pub attribute:   Option<Attribute>,
    pub level:       Option<u32>,
    pub linkval:     Option<u32>,
    pub linkmarkers: Option<Vec<LinkMarker>>,
*/
}

#[derive(Clone, Debug, Deserialize)]
pub struct CardImage {
    pub image_url: String,
    pub image_url_small: String,
}
/*
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
*/

#[derive(Debug, Default, Deserialize)]
struct YGOProResponse { data: Vec<ResponseCard> }

/* simple query function to test w/, to be commented out
pub async fn get_cards_test() -> Result<Vec<ResponseCard>, Box<dyn Error>> {
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
*/

pub async fn get_cards()
    -> Result<(Vec<ResponseCard>, Vec<ResponseCard>), Box<dyn Error>>
{
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let client = reqwest::Client::new();

    info!("updating card database");
    let main_deck   = get_main(&client, url).await?;
    let extra_deck  = get_extra(&client, url).await?;

    Ok((main_deck, extra_deck))
}

#[instrument]
async fn get_main(client: &Client, url: &str)
    -> Result<Vec<ResponseCard>, Box<dyn Error>>
{
    info!("getting normal monsters");
    let normals = client.get(url)
        .query(&[("level", "lte4"), ("type", "Normal Monster"), ("atk", "gte1900")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of normal monsters: {}", normals.len());

    info!("getting effect monsters");
    let effects = client.get(url)
        .query(&[("level", "lte4"), ("type", "Effect Monster")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of effect monsters: {}", effects.len());

    info!("getting spells");
    let spells = client.get(url)
        .query(&[("type", "Spell Card")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of spells: {}", spells.len());

    info!("getting traps");
    let traps = client.get(url)
        .query(&[("type", "Trap Card")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of traps: {}", traps.len());

    let mut main = normals;
    main.extend(effects.into_iter());
    main.extend(spells.into_iter());
    main.extend(traps.into_iter());

    Ok(main)
}

#[instrument]
async fn get_extra(client: &Client, url: &str)
    -> Result<Vec<ResponseCard>, Box<dyn Error>>
{
    info!("getting fusion monsters");
    let fusions = client.get(url)
        .query(&[("type", "Fusion Monster")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of fusion monsters: {}", fusions.len());

    info!("getting synchro monsters");
    let synchros = client.get(url)
        .query(&[("type", "Synchro Monster")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of synchro monsters: {}", synchros.len());

    info!("getting xyz monsters");
    let xyzs = client.get(url)
        .query(&[("type", "Xyz Monster")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of xyz monsters: {}", xyzs.len());

    info!("getting link monsters");
    let links = client.get(url)
        .query(&[("type", "Link Monster")])
        .send().await?
        .json::<YGOProResponse>().await?
        .data;
    trace!("number of link monsters: {}", links.len());

    let mut extra = fusions;
    extra.extend(synchros.into_iter());
    extra.extend(xyzs.into_iter());
    extra.extend(links.into_iter());

    Ok(extra)
}

