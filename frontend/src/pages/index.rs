use stylist::Style;
use yew::prelude::*;
use crate::components::{Intro, NavBar, Rules};

#[function_component]
pub fn Index() -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <>
        <NavBar />
        <div class={styles}>
            <h1>{"Yu-Gi-Oh! Modern Draft"}</h1>
            <Intro />

            <h2>{"General Rules"}</h2>
            <p><i>
                {"(These rules are not fully thought through as the focus was
                on building the app. I'm open to suggestions on the project's "}
                <a href="https://github.com/hadan24/ygo-draft/issues/" target="_blank">
                    {"Github issues page"}
                </a>{"!)"}
            </i></p>
            <Rules />
        </div>
        </>
    }
}

const STYLE: &str =
r#"

"#;