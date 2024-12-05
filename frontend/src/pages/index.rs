use stylist::Style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct IndexProps {}

#[function_component]
pub fn Index(_props: &IndexProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <div class={styles}>
        
        </div>
    }
}

const STYLE: &str =
r#"

"#;