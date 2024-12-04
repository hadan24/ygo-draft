use serde::Deserialize;

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub img_link: String
}
pub type DraftOptionsArray = [Card; 3];

#[derive(Clone, PartialEq)]
pub enum DraftOptionSource { Main, Extra }

#[derive(Clone, Default, PartialEq)]
pub struct DeckListsState {
    pub main: Vec<Card>,
    pub extra: Vec<Card>
}