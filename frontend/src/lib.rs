mod components;
mod pages;

use components::*;
//use pages::*;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Deserialize, Serialize)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub img_link: String
}
pub type DraftOptions = [Card; 3];

#[function_component]
pub fn App() -> Html {
    log!("Hallo :D 🦀");
    let report_choice = Callback::from(|c: Card| {
        log!(c.id, c.name, c.img_link);
    });

    html! {
        <>
            <h1>{"Hallo :D 🦀"}</h1>

            <OptionDisplay report_choice={report_choice} />
        </>
    }
}