use std::ops::Deref;
use stylist::Style;
use yew::prelude::*;
use crate::{
    components::{DeckList, OptionDisplay},
    Card,
    DeckListsState,
    DraftOptionSource
};


#[function_component]
pub fn Draft() -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");
    let decklist = use_state(|| {DeckListsState::default()});

    let temp = decklist.clone();
    let report_choice = Callback::from(move |c: Card| {
        use DraftOptionSource::*;
        let curr_source = &SOURCE_ORDER[temp.main.len() + temp.extra.len()];

        let mut new_deck = match curr_source {
            Main => temp.main.clone(),
            Extra => temp.extra.clone()
        };
        new_deck.push(c);

        match curr_source {
            Main => temp.set(DeckListsState {
                main: new_deck,
                extra: temp.extra.clone()
            }),
            Extra => temp.set(DeckListsState {
                main: temp.main.clone(),
                extra: new_deck
            })
        }
    });

    html! {
        <>
        <h1>{"Hallo :D 🦀"}</h1>
        <div class={styles}>
            <OptionDisplay 
                report_choice={report_choice}
                next_source={SOURCE_ORDER[decklist.main.len() + decklist.extra.len() + 1].clone()}
            />
            <DeckList list={decklist.deref().clone()} />
        </div>
        </>
    }
}

const STYLE: &str =
r#"
display: grid;
grid-template-columns: 70% 30%;
grid-template-areas: 
    "options decklist"
    "stats decklist";
"#;

const SOURCE_ORDER: [DraftOptionSource; 55] = {
    use DraftOptionSource::*;
    [   // 1  2     3     4     5     6     7     8     9      10     11
        Main, Main, Main, Main, Main, Main, Main, Main, Extra, Extra, Extra,
        Main, Main, Main, Main, Main, Main, Main, Main, Extra, Extra, Extra,
        Main, Main, Main, Main, Main, Main, Main, Main, Extra, Extra, Extra,
        Main, Main, Main, Main, Main, Main, Main, Main, Extra, Extra, Extra,
        Main, Main, Main, Main, Main, Main, Main, Main, Extra, Extra, Extra,
    ]
};