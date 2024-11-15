mod components;
mod pages;

use components::*;
//use pages::*;

use gloo::console::log;
use yew::prelude::*;

#[derive(Default)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub img_link: String
}

#[function_component]
pub fn App() -> Html {
    log!("Hallo :D 🦀");

    html! {
        <>
            <h1>{"Hallo :D 🦀"}</h1>

            <OptionDisplay />
        </>
    }
}