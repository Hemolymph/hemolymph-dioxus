mod backend;
mod components;
mod router;

use components::{CardDetails, Results};
use dioxus::prelude::*;
use router::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(not(debug_assertions))]
static HOST: &'static str = "https://hemolymph.net";

#[cfg(debug_assertions)]
pub static HOST: &str = "http://127.0.0.1:8080";

fn main() {
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
