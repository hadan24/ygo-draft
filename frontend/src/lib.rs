mod types;
mod components;
mod pages;

pub use types::*;
use pages::*;
use yew::prelude::*;


#[function_component]
pub fn App() -> Html {
    html! {
        <Draft />
    }
}