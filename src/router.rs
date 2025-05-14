use crate::components::CardDetailSimple;
use crate::components::Instructions;
use crate::components::SearchBar;
use crate::CardDetails;
use crate::Query;
use crate::Results;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(SearchBar)]
    #[route("/")]
    Main,
    #[route("/:query")]
    Results { query: String },
    #[route("/card/:id")]
    CardDetailSimple { id: String },
    #[route("/card/:id/:img_idx")]
    CardDetails { id: String, img_idx: usize },
    #[route("/syntax")]
    Instructions {},
}

#[component]
pub fn Main() -> Element {
    use_effect(move || {
        let mut ctx = consume_context::<Query>();
        ctx.has_been_filled.set(false);
    });
    rsx! {}
}
