use dioxus::prelude::*;

use crate::{backend::process_query, components::CardView, Query};

#[component]
pub fn Results(query: String) -> Element {
    let mut ctx = consume_context::<Query>();
    use_effect(move || {
        ctx.has_been_filled.set(true);
        ctx.query.set(query.clone());
    });
    let search = use_resource(move || {
        let query = ctx.query.read();
        process_query(query.clone())
    });

    match &*search.value().read() {
        Some(Ok(cards)) => rsx! {
             div {
                 class: "results",
                 for card in cards.clone() {
                     CardView { card, img_idx: 0 }
                 }
             }
        },
        Some(Err(err)) => rsx! {"failed to do search"},
        None => rsx! {"Loading..."},
    }
}
