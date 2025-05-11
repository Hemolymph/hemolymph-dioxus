use crate::components::SearchBar;
use crate::CardDetails;
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
    CardDetails { id: String },
}

#[component]
pub fn Main() -> Element {
    rsx! {}
}
