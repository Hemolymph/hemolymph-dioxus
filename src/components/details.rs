use dioxus::prelude::*;
use hemoglobin::cards::Card;

use crate::{
    backend, components::CardView, get_ascii_titlecase, get_filegarden_link, render_rich_string,
    Query,
};

#[component]
pub fn CardDetails(id: String, img_idx: usize) -> Element {
    let card = use_resource(move || backend::get_card_id(id.clone()));

    use_effect(move || {
        let mut ctx = consume_context::<Query>();
        ctx.has_been_filled.set(true);
    });

    match &*card.value().read() {
        Some(Ok(card)) => {
            let alternates = card.images.len();
            rsx! {
                // Main {}
                CardDetailsView { card: card.clone(), img_idx }
                if alternates > 1 {
                    div {
                        class: "results",
                        for img_idx in 0..alternates {
                            CardView { card: card.clone(), img_idx }
                        }
                    }
                }
            }
        }
        Some(Err(err)) => rsx! {
            // Main {}
            "Error while loading card details: {err}"
        },
        None => rsx! {
            // Main {}
            "Loading"
        },
    }
}

#[component]
pub fn CardDetailSimple(id: String) -> Element {
    rsx! { CardDetails { id, img_idx: 0 }}
}

#[component]
pub fn CardDetailsView(card: Card, img_idx: usize) -> Element {
    let img = get_filegarden_link(&card.get_image_path(img_idx));
    let text = render_rich_string(&card.description);
    let kind = get_ascii_titlecase(&card.r#type);
    let flavor_text: Vec<_> = card
        .flavor_text
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            rsx! {
                p {
                    class:"flavor-line",
                    "{x}"
                }
            }
        })
        .collect();
    rsx! {
        div {
            class: "card_details",
            img {
                class: "card_details_img",
                src: img,
            }
            div {
                class: "card_details_text",
                h2 { "{card.name}" }
                hr {}
                p {
                    id: "cost-line",
                    "{kind} :: {card.cost} Blood"
                }
                hr {}
                { text }
                div {
                    class: "bottom",
                    if !flavor_text.is_empty() {
                        hr {}
                        for x in flavor_text {
                            {x}
                        }
                    }
                    hr {}
                    "{card.health}/{card.defense}/{card.power}"
                }
            }
        }
    }
}
