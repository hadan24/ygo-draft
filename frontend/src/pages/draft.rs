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
    let report_choice = Callback::from(|c: Card| {
        log!(serde_json::to_string_pretty(&c).unwrap());
    });

    html! {
        <div class={styles}>
            <h1>{"Hallo :D 🦀"}</h1>

            <DraftOptionDisplay report_choice={report_choice} />
            <DeckList />
        </div>
    }
}

const STYLE: &str =
r#"

"#;