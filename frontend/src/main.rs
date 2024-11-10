use gloo::console::log;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    log!("Hallo :D 🦀");

    html! {
        <>
            <h1>{"Hallo :D 🦀"}</h1>
            <p>{ *counter }</p>
            <button {onclick}>{ "+1" }</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}