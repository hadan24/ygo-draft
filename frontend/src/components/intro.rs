use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Intro() -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <div class={styles}>
        <p>
            {"Welcome to my Yu-Gi-Oh Draft Simulator! This is a personal
            project to learn Rust, Yew, and full-stack web development! If
            you're new to Yu-Gi-Oh, then I recommend checking out the resources
            and communities listed "}
            <a href="https://reddit.com/r/Yugioh101/comments/1f50riu/introduction_to_yugioh/" target="_blank">
                {"here"}
            </a>
            {" and "}
            <a href="https://reddit.com/r/Yugioh101/wiki/judge_resources/" target="_blank">
                {"here!"}
            </a>
        </p>
        <p>
            {"If you're new to "} <i>{"draft"}</i> {" and "} <i>{"drafting"}</i>
            {", it's a different way to play Yu-Gi-Oh from normal. "}
            {"Instead of using cards from your collection like in real life or
            Master Duel, or from the entire card pool like in various 3rd-party
            simulators, you build your deck bit by bit from a series of
            restricted options. You pick whichever card you think is best for
            your deck each round until you have a complete deck to duel with."}
        </p>
        <p>
            {"The goal is to have fun in an environment where decks are weaker
            than usual, and to test your card evaluation and improvisational
            deck-building skills! It more closely mimics the feeling of
            \"playground\" or \"kitchen table\" Yu-Gi-Oh of kids trying to
            cobble together working decks from their haphazard collections, but
            in a more structured, easily replicated format!"}
        </p>
        <p>
            {"If there's anything I left out or you would like to see added to
            this introduction for a fuller/better explanation, please make an
            issue on the project's "}
            <a href="https://github.com/hadan24/ygo-draft/issues/" target="_blank">
                {"Github issues page"}
            </a>{"."}
        </p>
        </div>
    }
}

const STYLE: &str =
r#"

"#;