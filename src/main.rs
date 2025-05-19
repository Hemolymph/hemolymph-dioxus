#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]
#![allow(clippy::semicolon_if_nothing_returned)]

mod backend;
mod components;
mod router;

use components::{CardDetails, CardHemolink, QueryHemolink, Results};
use dioxus::prelude::*;
use hemoglobin::cards::rich_text::{RichElement, RichString};
use router::Route;
use server_fn::client::set_server_url;

const FAVICON: Asset = asset!("/assets/hemo_icon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(not(debug_assertions))]
static HOST: &'static str = "https://hemolymph.net";

#[cfg(debug_assertions)]
pub static HOST: &str = "http://127.0.0.1:8080";

fn main() {
    #[cfg(feature = "server")]
    {
        backend::setup_card_debounce();
    }
    set_server_url(HOST);
    dioxus::launch(App);
}

#[derive(Clone)]
struct Query {
    query: Signal<String>,
    has_been_filled: Signal<bool>,
}

#[component]
fn App() -> Element {
    let query = use_signal(String::new);
    let has_been_filled = use_signal(|| false);
    use_context_provider::<Query>(|| Query {
        query,
        has_been_filled,
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

fn get_filegarden_link(name: &str) -> String {
    format!(
        "https://file.garden/ZJSEzoaUL3bz8vYK/bloodlesscards/{}.png",
        name.replace(' ', "").replace('ä', "a")
    )
}

fn get_ascii_titlecase(s: &str) -> String {
    let mut b = s.to_string();
    if let Some(r) = b.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    b
}

fn render_rich_string(string: &RichString) -> Element {
    let mut paragraphs = vec![];
    for element in string {
        match element {
            RichElement::String(string) => {
                if paragraphs.is_empty() {
                    paragraphs.push(vec![]);
                }

                let lines = &mut string.lines();
                if let Some(x) = lines.next().filter(|x| !x.is_empty()) {
                    paragraphs
                        .last_mut()
                        .unwrap()
                        .push(RichElement::String(x.to_string()));
                }

                for line in lines.filter(|x| !x.is_empty()) {
                    paragraphs.push(vec![RichElement::String(line.to_string())]);
                }
            }
            el @ (RichElement::CardId {
                display: _,
                identity: _,
            }
            | RichElement::SpecificCard { display: _, id: _ }
            | RichElement::CardSearch {
                display: _,
                search: _,
            }) => match paragraphs.last_mut() {
                Some(last) => last.push(el.clone()),
                None => paragraphs.push(vec![el.clone()]),
            },
            el @ RichElement::Saga(_) => paragraphs.push(vec![el.clone()]),
            RichElement::LineBreak => paragraphs.push(vec![]),
        }
    }

    paragraphs
        .iter()
        .map(|x| {
            let x = x.iter().map(|x| match x {
                RichElement::String(string) => rsx! {"{string}"},
                RichElement::CardId {
                    display,
                    identity: _,
                } => rsx! { "{display}" },
                RichElement::SpecificCard { display, id } => rsx! {
                    CardHemolink {
                        display: display.clone(),
                        card_id: id.clone(),
                    }
                    // Link {
                    //     to: Route::CardDetailSimple { id: id.clone() },
                    //     class: "hemolink",
                    //     "{display}"
                    // }
                },
                RichElement::CardSearch { display, search } => rsx! {
                    QueryHemolink {
                        display: display.clone(),
                        query: search.clone(),
                    }
                    // Link {
                    //     to: Route::Results { query: search.clone() } ,
                    //     class: "hemolink",
                    //     "{display}"
                    // }
                },
                RichElement::Saga(list) => {
                    let list = list.iter().map(render_rich_string);
                    rsx! {
                        ol {
                            for item in list {
                                li { {item} }
                            }
                        }
                    }
                }
                RichElement::LineBreak => rsx! { br {}},
            });
            rsx! { p {
                for item in x {
                    {item}
                }
            } }
        })
        .reduce(|acc, el| rsx! { {acc} {el}})
        .unwrap_or(rsx! {})
}
