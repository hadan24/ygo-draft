use stylist::Style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardOptionProps {
    pub name: AttrValue,
    pub img_link: AttrValue,
}

#[function_component]
pub fn CardOption(props: &CardOptionProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html!{
        <div class={styles}>
            <h3>{&props.name}</h3>
            <img src={props.img_link.clone()} />
            <br />
            <button value={props.name.clone()}>{"Select"}</button>
        </div>
    }
}

const STYLE: &str =
r#"
width: 33%;
text-align: center;

img {
    width: 200px;
    margin-top: auto;
}
"#;