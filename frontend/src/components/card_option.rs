use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardOptionProps {
    pub name: String,
    pub img_link: String,
}

#[function_component]
pub fn CardOption(props: &CardOptionProps) -> Html {
    html!{
        <div class="card">
            <h3>{&props.name}</h3>
            <img src={props.img_link.clone()} width="400px" />
            <br />
            <button value={props.name.clone()}>{"Select"}</button>
        </div>
    }
}