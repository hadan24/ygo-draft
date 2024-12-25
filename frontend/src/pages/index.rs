use stylist::Style;
use yew::prelude::*;
use crate::components::Rules;

#[derive(PartialEq, Properties)]
pub struct IndexProps {}

#[function_component]
pub fn Index(_props: &IndexProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <div class={styles}>
            <h1>{"Yu-Gi-Oh! Modern Draft"}</h1>
            <h2>{"General Rules"}</h2>
            <Rules />
        </div>
    }
}

const STYLE: &str =
r#"

"#;