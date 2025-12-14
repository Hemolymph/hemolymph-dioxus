use dioxus::prelude::*;
use hemoglobin::cards::Card;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use crate::{get_filegarden_link, router::Route};

#[component]
pub fn CardView(card: Card, img_idx: usize) -> Element {
    let img_path = card.get_image_path(img_idx);
    let image = get_filegarden_link(&img_path);
    let clipboard = window().map(|w| w.navigator().clipboard());
    let copy = move |_| {
        let clipboard = clipboard.clone();
        let img_path = img_path.clone();
        spawn(async move {
            if let Some(clipboard) = clipboard.clone() {
                let promise = clipboard.write_text(&img_path);
                JsFuture::from(promise).await.unwrap();
            }
        });
    };
    let id = card.id.clone();
    rsx! {
        div {
            key: "{id}_div",
            class: "card_result",
            Link {
                key: "{id}_link",
                to: Route::CardDetails { id: card.id, img_idx },
                img {
                    key: "{id}_img",
                    class: "card_result_img",
                    src: "{image}",
                }
            }
            button {
                onclick: copy,
                "Copy Marrow ID"
            }
        }
    }
}
