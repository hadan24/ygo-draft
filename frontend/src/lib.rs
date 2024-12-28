mod types;
mod components;
mod pages;

pub use types::*;
use pages::*;
use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Index,
    #[at("/draft")]
    Draft
}
fn route(routes: Routes) -> Html {
    match routes {
        Routes::Index => html!{ <Index /> },
        Routes::Draft => html!{ <Draft /> }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routes> render={route} />
        </BrowserRouter>
    }
}