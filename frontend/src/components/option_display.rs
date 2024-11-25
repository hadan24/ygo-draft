use std::ops::Deref;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{Card, DraftOptionsArray, CardOption};

#[derive(PartialEq, Properties)]
pub struct OptionDisplayProps {
    pub report_choice: Callback<Card>
}

#[function_component]
pub fn DraftOptionDisplay(props: &OptionDisplayProps) -> Html {
    let state: UseStateHandle<Option<DraftOptionsArray>> = use_state(|| None);

    let report_card_choice = props.report_choice.clone();
    let temp = state.clone();   // keeps original state usable for html
    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        // if outside, requires temp to exist as long as the callback does
        // callback lasts for `static presumably b/c it's used in the html
        let curr_cards = temp.as_ref();
        if let Some(cards) = curr_cards {
            let chosen = event.submitter().unwrap() // Option<HtmlElement>
                .get_attribute("value").unwrap();   // Option<String>
    
            // assumes all unique cards, currently guaranteed by backend
            for c in cards.iter() {
                if chosen == c.name {
                    report_card_choice.emit(c.clone());
                    break;
                }
            }
        }

        let new_state = temp.clone();
        spawn_local( async move {
            let api_cards = Request::get("http://127.0.0.1:8000/main")
                .send().await
                    // Result<gloo::net::http::response::Response, gloo::net::error::Error>
                .expect("Backend should be running, also ensure the URL is correct")
                .json::<DraftOptionsArray>().await
                    // Result<DraftOptions, gloo::net::error::Error>
                .unwrap();
            new_state.set(Some(api_cards));
        });
    });
    
    
    html!{
        <form onsubmit={on_submit}>{
        match state.deref() {
            None => html!{ <button>{"Start"}</button> },

            Some(cards) =>
                cards.iter().map(|c| html!{
                    <CardOption
                        name={c.name.clone()}
                        img_link={c.img_link.clone()}
                    />
                }).collect::<Html>()
        }
        }</form>
    }
}