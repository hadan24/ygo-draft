use std::ops::Deref;
use stylist::Style;
use yew::prelude::*;
use crate::{
    components::{DeckList, DraftOptionDisplay, DraftOptionSource},
    Card
};

#[derive(Clone, Default, PartialEq)]
pub struct DeckListsState {
    pub main: Vec<Card>,
    pub extra: Vec<Card>
}

#[derive(PartialEq, Properties)]
pub struct DraftProps {}

#[function_component]
pub fn Draft(_props: &DraftProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");
    let deck = use_state(|| {DeckListsState::default()});

    let temp = deck.clone();
    let report_choice = Callback::from(move |c: Card| {
        use DraftOptionSource::*;
        let curr_source = SOURCE_ORDER[temp.main.len() + temp.extra.len()].clone();

        let mut new_deck = match curr_source {
            Main => temp.main.clone(),
            Extra => temp.extra.clone(),
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
            <DraftOptionDisplay 
                report_choice={report_choice}
                next_source={SOURCE_ORDER[deck.main.len() + deck.extra.len() + 1].clone()}
            />
            <DeckList list={deck.deref().clone()} />
        </div>
        </>
    }
}

const STYLE: &str =
r#"
display: flex;
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