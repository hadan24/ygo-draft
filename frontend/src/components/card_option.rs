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
        <div key={&*props.name} class={styles}>
            <h3>{&props.name}</h3>
            <img src={&props.img_link} />
            <br />
            <button value={&props.name}>{"Select"}</button>
        </div>
    }
}

const STYLE: &str =
r#"
width: 33%;
text-align: center;

h3 {
    height: 2em;
}
img {
    width: 200px;
    margin-top: auto;
}
"#;