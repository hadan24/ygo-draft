use stylist::Style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RulesProps {}

#[function_component]
pub fn Rules(_props: &RulesProps) -> Html {
    let styles = Style::new(RULES_STYLE).expect("Ensure CSS is valid");

    html! {
        <div class={styles}>
        <p>
            {"Welcome to my Yu-Gi-Oh Draft Simulator! If you're new to
            Yu-Gi-Oh, then I recommend checking out the resources and
            communities listed "}
            <a href="https://www.reddit.com/r/Yugioh101/comments/1f50riu/introduction_to_yugioh/">
                {"here"}
            </a>
            {" and "}
            <a href="https://www.reddit.com/r/Yugioh101/wiki/judge_resources/">
                {"here."}
            </a>
            {""}
        </p>
        </div>
    }
}

const RULES_STYLE: &str =
r#"

"#;