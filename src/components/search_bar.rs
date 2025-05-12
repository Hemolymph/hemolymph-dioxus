use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;
use std::time::Duration;

use crate::{router::Route, Query};

#[component]
pub fn SearchBar() -> Element {
    let navigator = use_navigator();
    let mut context = use_context::<Query>();
    let mut query_debounce = use_debounce(Duration::from_millis(400), move |query: String| {
        context.query.set(query.clone());
        if !query.trim().is_empty() {
            context.has_been_filled.set(true);
        }
        navigator.push(Route::Results { query });
    });
    let mut empty_query_debounce =
        use_debounce(Duration::from_millis(1500), move |query: String| {
            if query.trim().is_empty() {
                context.query.set(String::new());
                context.has_been_filled.set(false);
                navigator.push(Route::Main);
            }
        });

    let has_been_filled = context.has_been_filled.read();
    let class = if *has_been_filled { "top" } else { "home" };

    let query = context.query.read();

    rsx! {
        div {
            class: "search-box {class}",
            img {
                id: "logo",
                src: "https://file.garden/ZJSEzoaUL3bz8vYK/hemolymphlogo.png",
            }
            input {
                key: "search-bar",
                id: "search-bar",
                type: "text",
                value: "{query}",
                placeholder: "Type your search here. Search for () to see all cards.",
                autofocus: true,
                oninput: move |evt| {
                    let value = evt.value();
                    empty_query_debounce.action(value.clone());
                    query_debounce.action(value);
                }
            }
        }
        Outlet::<Route> {}
    }
}
