use stylist::Style;
use yew::prelude::*;
use crate::{Card, DeckListsState};

#[derive(PartialEq, Properties)]
pub struct DeckListProps {
    pub list: DeckListsState
}

#[function_component]
pub fn DeckList(props: &DeckListProps) -> Html {
    let style = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <div class={style}>
            <ul class={"main"}>{ decklist_to_html(&props.list.main) }</ul>
            <ul class={"extra"}>{ decklist_to_html(&props.list.extra) }</ul>
        </div>
    }
}

fn decklist_to_html(deck: &[Card]) -> Html {
    deck.iter().map(|c| html!{
        <li>{c.name.clone()}</li>
    }).collect::<Html>()
}

const STYLE: &str =
r#"
margin: 5px;
border: 1px solid black;
padding: 10px;
width: 30%;

.main {
    border: 1px solid orange;
}

.extra {
    border: 1px solid purple;
}
"#;