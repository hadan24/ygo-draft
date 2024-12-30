use gloo::file::{File, ObjectUrl};
use stylist::Style;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;
use crate::DeckListsState;

#[derive(PartialEq, Properties)]
pub struct DeckDownloadProps {
    pub list: DeckListsState
}

#[function_component]
pub fn DeckDownload(props: &DeckDownloadProps) -> Html {
    let styles = Style::new(STYLE).expect("Ensure CSS is valid");
    let anch_ref = use_node_ref();

    let list_clone = props.list.clone();
    let ref_clone = anch_ref.clone();
    let onclick = move |_| {
        let ydk_contents = build_ydk(&list_clone);
        let file = File::new("deck.ydk", &*ydk_contents);
        let url = ObjectUrl::from(file);
        
        if let Some(a) = ref_clone.cast::<HtmlAnchorElement>() {
            a.set_href(&url);
            a.set_download("deck.ydk");
        }
    };

    html! {
        <div class={styles}>
            <h2>{"You're done! Click below to download your deck."}</h2>
            <p>
                {"You will get a "} <span>{"\".ydk\""}</span>
                {" file compatible with the "}
                <a href="https://ygoprodeck.com/deckbuilder/" target="_blank">
                    {"YGOProDeck deck builder"}
                </a>{", "}
                <a href="https://www.duelingbook.com/" target="_blank">
                    {"DuelingBook"}
                </a>{", and "}
                <a href="https://omega.duelistsunite.org/" target="_blank">
                    {"YGO Omega"}
                </a>{" platforms/simulators."}

                <br />
                {"More export formats may come in the future!"}
            </p>
            <button {onclick}><a ref={anch_ref}>
                {"Download"}
            </a></button>
        </div>
    }
}

fn build_ydk(lists: &DeckListsState) -> String {
    let mut contents = String::from("#created by sHaDe drafter\n#main\n");
    for card in lists.main.iter() {
        contents.push_str(&format!("{}\n", card.id));
    }
    contents.push_str("#extra\n");
    for card in lists.extra.iter() {
        contents.push_str(&format!("{}\n", card.id));
    }

    contents
}

const STYLE: &str =
r#"
margin: 5px;
border: 1px solid black;
padding: 5px;

text-align: center;
span {
    font-family: monospace;
}

grid-area: options;
"#;