use stylist::Style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DecksProps {}

#[function_component]
pub fn Decks(_props: &DecksProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <div class={styles}>
        
        </div>
    }
}

const STYLE: &str =
r#"

"#;