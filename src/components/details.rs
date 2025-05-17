use dioxus::prelude::*;
use hemoglobin::cards::rich_text::{RichElement, RichString};

use crate::{
    backend,
    components::{CardHemolink, CardView, QueryHemolink},
    get_filegarden_link, Query,
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

            let alternates = card.images.len();
            rsx! {
                // Main {}
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

fn render_rich_string(string: &RichString) -> Element {
    let mut paragraphs = vec![];
    for element in string {
        match element {
            RichElement::String(string) => {
                if paragraphs.is_empty() {
                    paragraphs.push(vec![]);
                }

                let lines = &mut string.lines();
                if let Some(x) = lines.next().filter(|x| !x.is_empty()) {
                    paragraphs
                        .last_mut()
                        .unwrap()
                        .push(RichElement::String(x.to_string()));
                }

                for line in lines.filter(|x| !x.is_empty()) {
                    paragraphs.push(vec![RichElement::String(line.to_string())]);
                }
            }
            el @ (RichElement::CardId {
                display: _,
                identity: _,
            }
            | RichElement::SpecificCard { display: _, id: _ }
            | RichElement::CardSearch {
                display: _,
                search: _,
            }) => match paragraphs.last_mut() {
                Some(last) => last.push(el.clone()),
                None => paragraphs.push(vec![el.clone()]),
            },
            el @ RichElement::Saga(_) => paragraphs.push(vec![el.clone()]),
            RichElement::LineBreak => paragraphs.push(vec![]),
        }
    }

    paragraphs
        .iter()
        .map(|x| {
            let x = x.iter().map(|x| match x {
                RichElement::String(string) => rsx! {"{string}"},
                RichElement::CardId {
                    display,
                    identity: _,
                } => rsx! { "{display}" },
                RichElement::SpecificCard { display, id } => rsx! {
                    CardHemolink {
                        display: display.clone(),
                        card_id: id.clone(),
                    }
                    // Link {
                    //     to: Route::CardDetailSimple { id: id.clone() },
                    //     class: "hemolink",
                    //     "{display}"
                    // }
                },
                RichElement::CardSearch { display, search } => rsx! {
                    QueryHemolink {
                        display: display.clone(),
                        query: search.clone(),
                    }
                    // Link {
                    //     to: Route::Results { query: search.clone() } ,
                    //     class: "hemolink",
                    //     "{display}"
                    // }
                },
                RichElement::Saga(list) => {
                    let list = list.iter().map(render_rich_string);
                    rsx! {
                        ol {
                            for item in list {
                                li { {item} }
                            }
                        }
                    }
                }
                RichElement::LineBreak => rsx! { br {}},
            });
            rsx! { p {
                for item in x {
                    {item}
                }
            } }
        })
        .reduce(|acc, el| rsx! { {acc} {el}})
        .unwrap_or(rsx! {})
}

fn get_ascii_titlecase(s: &str) -> String {
    let mut b = s.to_string();
    if let Some(r) = b.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    b
}
