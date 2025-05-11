use dioxus::prelude::*;

use crate::backend;

#[component]
pub fn CardDetails(id: String) -> Element {
    let card = use_resource(move || backend::get_card_id(id.clone()));

    match &*card.value().read() {
        Some(Ok(card)) => rsx! {
            // Main {}
            "seeing details of {card.name}"
        },
        Some(Err(err)) => rsx! {
            // Main {}
            "Error while loading card details"
        },
        None => rsx! {
            // Main {}
            "Loading"
        },
    }
}
