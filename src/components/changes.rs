use dioxus::prelude::*;

use crate::{
    backend::{card_changes, CardDiff},
    components::CardDetailsView,
};

#[component]
pub fn ChangeList() -> Element {
    let search = use_resource(card_changes);
    match &*search.value().read() {
        Some(Ok(changes)) => rsx! {
            div {
                class: "change_list",
                for (idx, change) in changes.iter().enumerate() {
                    if idx < 10 {
                        CardChange { change: change.clone() }
                    }
                }
            }
        },
        Some(Err(err)) => rsx! { "Couldn't load card changes: {err}" },
        None => rsx! { "Loading" },
    }
}

#[component]
fn CardChange(change: CardDiff) -> Element {
    match change {
        CardDiff::New(card) => {
            rsx!(
                div {
                    class: "card_change",
                    span {
                        class: "card_change_title",
                        "New Card"
                    }
                    CardDetailsView { card: *card, img_idx: 0 }
                }
            )
        }
        CardDiff::Changed { old, new } => {
            rsx!(
                div {
                    class: "card_change",
                    span {
                        class: "card_change_title",
                        "Card Changed"
                    }
                    CardDetailsView { card: *new, img_idx: 0 }
                }
            )
        }
    }
}
