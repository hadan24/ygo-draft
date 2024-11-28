use std::ops::Deref;
use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use crate::{
    components::{DraftOptionDisplay, DeckList},
    Card
};

#[derive(PartialEq, Properties)]
pub struct DraftProps {}

#[function_component]
pub fn Draft(_props: &DraftProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    let deck: UseStateHandle<Vec<Card>> = use_state(|| {Vec::new()});
    let temp = deck.clone();
    let report_choice = Callback::from(move |c: Card| {
        log!(serde_json::to_string_pretty(&c).unwrap());

        let mut new_deck = temp.deref().to_vec();
        new_deck.push(c);
        temp.set(new_deck);
    });

    html! {
        <>
        <h1>{"Hallo :D 🦀"}</h1>
        <div class={styles}>
            <DraftOptionDisplay report_choice={report_choice} />
            <DeckList list={deck.deref().clone()} />
        </div>
        </>
    }
}

const STYLE: &str =
r#"
display: flex;
"#;