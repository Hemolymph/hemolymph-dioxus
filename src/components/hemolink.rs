use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn CardHemolink(display: String, card_id: String) -> Element {
    rsx! {
        Link {
            to: Route::CardDetailSimple { id: card_id.clone() },
            class: "hemolink",
            "{display}"
        }
    }
}

#[component]
pub fn QueryHemolink(display: String, query: String) -> Element {
    rsx! {
        Link {
            to: Route::Results { query } ,
            class: "hemolink",
            "{display}"
        }
    }
}
