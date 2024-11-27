use stylist::Style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DeckListProps {}

#[function_component]
pub fn DeckList(_props: &DeckListProps) -> Html {
    let style = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <div class={style}>
        
        </div>
    }
}

const STYLE: &str =
r#"

"#;