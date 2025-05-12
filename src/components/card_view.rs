use dioxus::prelude::*;
use hemoglobin::cards::Card;

use crate::get_filegarden_link;

#[component]
pub fn CardView(card: Card) -> Element {
    let image = get_filegarden_link(&card.get_image_path(0));
    rsx! {
        div {
            class: "card_result",
            img {
                class: "card_result_img",
                src: "{image}",
            }
    }    }
}
