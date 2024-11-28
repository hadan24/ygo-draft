use stylist::Style;
use yew::prelude::*;
use crate::Card;

#[derive(PartialEq, Properties)]
pub struct DeckListProps {
    pub list: Vec<Card>
}

#[function_component]
pub fn DeckList(props: &DeckListProps) -> Html {
    let style = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <ul class={style}>{
            props.list.iter().map(|c| html!{
                <li>{c.name.clone()}</li>
            }).collect::<Html>()
        }</ul>
    }
}

const STYLE: &str =
r#"
margin: 5px;
border: 1px solid black;
padding: 5px;
width: 30%;

li {
    margin-left: 20px;
}
"#;