use std::ops::Deref;
use yew::prelude::*;
use crate::Card;

#[derive(PartialEq, Properties)]
pub struct OptionDisplayProps {
    pub report_choice: Callback<Card>
}

#[function_component]
pub fn OptionDisplay(props: &OptionDisplayProps) -> Html {
    // http call to back end, store json in state
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
    let cards_clone = cards.deref().clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let chosen = event.submitter().unwrap() // Option<HtmlElement>
            .get_attribute("value").unwrap();           // Option<String>

        // assumes all unique cards, currently guaranteed by backend
        for c in cards_clone.iter() {
            if chosen == c.name {
                report_card_choice.emit(c.clone());
                return;
            }
        }
    });
    
    html!{
        <form onsubmit={onsubmit}>{
            cards.iter().map(|c| {
                html!{<CardOption
                    name={c.name.clone()}
                    img_link={c.img_link.clone()}
                />}
            }).collect::<Html>()
        }</form>
    }
}


#[derive(PartialEq, Properties)]
struct CardOptionProps {
    name: String,
    img_link: String,
}

#[function_component]
fn CardOption(props: &CardOptionProps) -> Html {
    html!{
        <div class="card">
            <h3>{&props.name}</h3>
            <img src={props.img_link.clone()} width="400px" />
            <br />
            <button value={props.name.clone()}>{"Select"}</button>
        </div>
    }
}