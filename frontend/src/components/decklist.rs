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
            <ul class={"main"}>
                <h4>{"Main Deck: "}{&props.list.main.len()}</h4>
                { decklist_to_html(&props.list.main) }
            </ul>
            <ul class={"extra"}>
                <h4>{"Extra Deck: "}{&props.list.extra.len()}</h4>
                { decklist_to_html(&props.list.extra) }
            </ul>
        </div>
    }
}

fn decklist_to_html(deck: &[Card]) -> Html {
    deck.iter().map(|c| html!{
        <li key={c.name.clone()}> {c.name.clone()} </li>
    }).collect::<Html>()
}

const STYLE: &str =
r#"
margin: 5px;
border: 1px solid black;
padding: 0px 15px;
grid-area: decklist;

.main {
    border: 1px solid orange;
    padding: 10px 0px 10px 40px;
}

.extra {
    border: 1px solid purple;
    padding: 10px 0px 10px 40px;
}
"#;