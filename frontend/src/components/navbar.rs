use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Routes;

#[function_component]
pub fn NavBar() -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");

    html! {
        <nav class={styles}>
        <ul>
            <li><Link<Routes> to={Routes::Index}>
                {"Home/Rules"}
            </Link<Routes>></li>
            <li><Link<Routes> to={Routes::Draft}>
                {"Draft"}
            </Link<Routes>></li>
        </ul>
        </nav>
    }
}

const STYLE: &str =
r#"

"#;
/*
https://www.w3schools.com/TAgs/tag_nav.asp
https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav
https://www.w3schools.com/Css/css_navbar.asp
https://yew.rs/docs/concepts/router
*/