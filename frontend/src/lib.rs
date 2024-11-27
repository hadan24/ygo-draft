mod components;
mod pages;

use components::*;
use pages::*;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;


#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub img_link: String
}
pub type DraftOptionsArray = [Card; 3];

#[function_component]
pub fn App() -> Html {
    log!("Hallo :D 🦀");

    html! {
        <Draft />
    }
}