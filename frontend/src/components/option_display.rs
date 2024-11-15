use yew::prelude::*;
use crate::Card;

#[derive(PartialEq, Properties)]
pub struct OptionDisplayProps {
    //pub report_card_choice: Callback<>
}

#[function_component]
pub fn OptionDisplay(_props: &OptionDisplayProps) -> Html {
    // http call to back end, store json in state
    // callback passed to CardChoice to get card
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

    
    html!{
        <form>
            <CardOption
                name={cards[0].name.clone()}
                img_link={cards[0].img_link.clone()}
            />
            <CardOption
                name={cards[1].name.clone()}
                img_link={cards[1].img_link.clone()}
            />
            <CardOption
                name={cards[2].name.clone()}
                img_link={cards[2].img_link.clone()}
            />
        </form>
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
            <button>{"Select"}</button>
        </div>
    }
}