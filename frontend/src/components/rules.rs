use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Rules() -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <ul class={styles}>
            <li>{"3 options are shown at a time (Hearthstone-style)."}</li>
            <li>{"All Main Deck Monsters are level 4 or lower."}</li>
            <li>{"All monsters are treated as every type."}</li>
            <li>
                {"All Fusion Summoning effects can summon any Fusion
                Monster if all the materials are correct."}
            </li>
            <li>
                {"Ritual and Pendulum Monsters are not included due to their
                inherent need for extremely high levels of synergy that are
                practically impossible in draft modes."}
            </li>
        </ul>
    }
}

const STYLE: &str =
r#"

"#;