use std::ops::Deref;
use gloo::net::http::Request;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{
    Card,
    DraftOptionsArray,
    DraftOptionSource,
    components::CardOption
};

#[derive(PartialEq, Properties)]
pub struct OptionDisplayProps {
    pub report_choice: Callback<Card>,
    pub next_source: DraftOptionSource
}

#[function_component]
pub fn OptionDisplay(props: &OptionDisplayProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");
    let state: UseStateHandle<Option<DraftOptionsArray>> = use_state(|| None);

    let report_callback = props.report_choice.clone();
    let next_source = props.next_source.clone();
    let temp = state.clone();   // keeps original state usable for html
    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        // if `temp.as_ref()` is outside the callback, temp is
        // required to exist as long as the callback does,
        // which is ('static) presumably b/c it's used in the html
        if let Some(cards) = temp.as_ref() {
            report_card_choice(&e, cards, &report_callback);
        }

        get_next_options(&temp, &next_source);
    });
    
    
    html!{
        <form onsubmit={on_submit} class={styles}>{
        match state.deref() {
            None => html!{ <button>{"Start"}</button> },
            Some(cards) => draft_options_to_html(cards)
        }
        }</form>
    }
}

fn report_card_choice(
    e: &SubmitEvent, cards: &DraftOptionsArray, cb: &Callback<Card>
) {
    // leaving unwrap()s here since how the app is built and the
    // `if let` check ensures the event has a named submitter
    let chosen = e.submitter()
        .expect("Form should have submitter button")
        .get_attribute("value")
        .expect("Button should have value (card name)");

    // assumes all unique cards, guaranteed by backend
    for c in cards {
        if chosen == c.name {
            cb.emit(c.clone());
            break;
        }
    }
}

fn get_next_options(
    new_state: &UseStateHandle<Option<DraftOptionsArray>>,
    next_source: &DraftOptionSource
) {
    let new_state = new_state.clone();
    let next_source = next_source.clone();
    spawn_local( async move {
        let url = match next_source {
            DraftOptionSource::Main => "http://127.0.0.1:8000/main",
            DraftOptionSource::Extra => "http://127.0.0.1:8000/extra"
        };
        
        let api_cards = Request::get(url)
            .send().await
            .expect("Backend should be running & ensure the URL is correct")
            .json::<DraftOptionsArray>().await
            .expect("Backend should return valid JSON");

        new_state.set(Some(api_cards));
    });
}

fn draft_options_to_html(cards: &DraftOptionsArray) -> Html {
    cards.iter().map(|c| html!{
        <CardOption
            name={c.name.clone()}
            img_link={c.img_link.clone()}
        />
    }).collect::<Html>()
}

const STYLE: &str = 
r#"
margin: 5px;
border: 1px solid black;
padding: 5px;
display: flex;
width: 70%;
height: auto;
"#;