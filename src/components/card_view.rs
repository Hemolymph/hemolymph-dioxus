use dioxus::prelude::*;
use hemoglobin::cards::Card;

#[component]
pub fn CardView(card: Card) -> Element {
    let image = get_filegarden_link(&card.name);
    rsx! {
        div {
            class: "card_result",
            img {
                class: "card_result_img",
                src: "{image}",
            }
    }    }
}

fn get_filegarden_link(name: &str) -> String {
    format!(
        "https://file.garden/ZJSEzoaUL3bz8vYK/bloodlesscards/{}.png",
        name.replace(' ', "").replace('ä', "a")
    )
}
