mod components;
mod pages;

use components::*;
//use pages::*;

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
    let report_choice = Callback::from(|c: Card| {
        log!(serde_json::to_string_pretty(&c).unwrap());
    });

    html! {
        <>
            <h1>{"Hallo :D 🦀"}</h1>

            <DraftOptionDisplay report_choice={report_choice} />
        </>
    }
}