use dioxus::prelude::*;

use crate::router::Route;

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
