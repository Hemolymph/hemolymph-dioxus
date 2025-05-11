use dioxus::prelude::*;

use crate::{backend::process_query, components::CardView, router::Route, Query};

#[component]
pub fn Results(query: String) -> Element {
    let search = use_resource(move || {
        let thing = consume_context::<Query>();
        let thing = thing.query.read();
        process_query(thing.clone())
    });

    match &*search.value().read() {
        Some(Ok(cards)) => rsx! {
             div {
                 class: "results",
                 for card in cards.clone() {
                     Link {
                         to: Route::CardDetails { id: card.id.clone() },
                         CardView { card }
                     }
                 }
             }
        },
        Some(Err(err)) => rsx! {"failed to do search"},
        None => rsx! {"Loading..."},
    }
}
