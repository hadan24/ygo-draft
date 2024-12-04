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
pub struct DraftOptionDisplayProps {
    pub report_choice: Callback<Card>,
    pub next_source: DraftOptionSource
}

#[function_component]
pub fn DraftOptionDisplay(props: &DraftOptionDisplayProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");
    let state: UseStateHandle<Option<DraftOptionsArray>> = use_state(|| None);

    let report_card_choice = props.report_choice.clone();
    let source = props.next_source.clone();
    let temp = state.clone();   // keeps original state usable for html
    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        // if `temp.as_ref()` is outside the callback, temp is required
        // to exist as long as the callback does, which is ('static)
        // presumably b/c it's used in the html
        let curr_cards = temp.as_ref();
        if let Some(cards) = curr_cards {
            // leaving unwrap()s here since how the app is built and the
            // `if let` check ensures the event has a named submitter
            let chosen = event.submitter()
                .expect("Form should have submitter button")
                .get_attribute("value")
                .expect("Button should have value (card name)");
    
            // assumes all unique cards, guaranteed by backend
            for c in cards {
                if chosen == c.name {
                    report_card_choice.emit(c.clone());
                    break;
                }
            }
        }

        let new_state = temp.clone();
        let source = source.clone();
        spawn_local( async move {
            let url = match source {
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