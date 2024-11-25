use std::ops::Deref;
use gloo::{
    console::log,
    net::http::Request
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{Card, DraftOptionsArr, CardOption};

#[derive(PartialEq, Properties)]
pub struct OptionDisplayProps {
    pub report_choice: Callback<Card>
}

#[function_component]
pub fn OptionDisplay(props: &OptionDisplayProps) -> Html {
    let cards = use_state(|| {
        [
            Card {
                id: 6247535,
                name: "Borreload eXcharge Dragon".to_string(),
                img_link:"img/6247535.jpg".to_string()
            },
            Card {
                id: 10515412,
                name: "Lightstorm Dragon".to_string(),
                img_link:"img/10515412.jpg".to_string()
            },
            Card {
                id: 24094258,
                name: "Heavymetalfoes Electrumite".to_string(),
                img_link:"img/24094258.jpg".to_string()
            }
        ]
    });

    let report_card_choice = props.report_choice.clone();
    let curr_cards = cards.deref().clone();
    let onsubmit = {
        let new_cards_state = cards.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let chosen = event.submitter().unwrap() // Option<HtmlElement>
                .get_attribute("value").unwrap();   // Option<String>

            // assumes all unique cards, currently guaranteed by backend
            for c in curr_cards.iter() {
                if chosen == c.name {
                    report_card_choice.emit(c.clone());
                    break;
                }
            }

            let new_cards_state = new_cards_state.clone();
            spawn_local( async move {
                let api_cards = Request::get("http://127.0.0.1:8000/main")
                .send().await
                    // Result<gloo::net::http::response::Response, gloo::net::error::Error>
                .expect("Backend should be running, also ensure the URL is correct")
                .json::<DraftOptionsArr>().await
                    // Result<DraftOptions, gloo::net::error::Error>
                .unwrap();

                log!(serde_json::to_string_pretty(&api_cards).unwrap());
                new_cards_state.set(api_cards);
            });
        })
    };
    
    html!{
        <form onsubmit={onsubmit}>{
            cards.iter().map(|c| html!{
                <CardOption
                    name={c.name.clone()}
                    img_link={c.img_link.clone()}
                />
            }).collect::<Html>()
        }</form>
    }
}