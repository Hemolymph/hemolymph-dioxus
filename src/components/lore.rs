use dioxus::prelude::*;

use crate::Query;

#[component]
pub fn Lore() -> Element {
    use_effect(move || {
        let mut ctx = consume_context::<Query>();
        ctx.has_been_filled.set(true);
    });
    rsx! {
        section {
            class: "instructions dark",
            h1 {
                "The Forest"
            }
            p { "The stories of Bloodless take place in an infinite forest. The Forest is a strange place, where all lost things eventually end up. To some, ending up here is a new beginning. To others, it is the end." }
            p { "It is up to the players to piece these mysteries together." }
            hr {}
            section {
                img {
                    src: "https://file.garden/ZJSEzoaUL3bz8vYK/BlueQueenMel.png",
                    class: "side"
                }
                h2 {
                    "The Piezans"
                }
                p {
                    "The piezans are native to The Forest. They see themselves as conflict-driven, although the specifics of how this perception manifests can vary from culture to culture."
                }
                p {
                    "Piezans are majorly born male, and can turn female in certain environments. This fact has shaped their social norms and worldview, perceiving feminity as the result of effort and study, while masculinity is the default state."
                }
                p {
                    "Piezans have a hard exterior. Their languages often feature clicks that are made using the exterior plates around their mouths."
                }
            }
            hr {}
            section {
                img {
                    src: "https://file.garden/ZJSEzoaUL3bz8vYK/bloodlesscards/Luna.png",
                    class: "side"
                }
                h2 {
                    "The Humans"
                }
                p {
                    "Humans are not native to the Forest, and instead arrive from other worlds, where they have gotten lost both physically and metaphorically. Humans that wind up in the Forest often live in small, isolated communities away from the Piezan world, where they can keep the ways of their human societies."
                }
                p {
                    "Humans that aren't born in the Forest tend to think Piezans are very strange. This feeling is mutual."
                }
            }
            hr {}
            section {
                img {
                    src: "https://file.garden/ZJSEzoaUL3bz8vYK/bloodlesscards/ExiledFeatherman.png",
                    class: "side"
                }
                h2 {
                    "The Feathermen"
                }
                p {
                    "It's very unclear where feathermen come from or what they are like, as they don't typically welcome strangers."
                }
                p {
                    "From what is known, they hold on tightly their history, and the few artifacts they have given to the wider world are believed to be linked to Piezan and Human history."
                }
            }
            hr {}
            section {
                class: "stories",
                img {
                    src: "https://file.garden/ZJSEzoaUL3bz8vYK/bloodlesscards/FlashOfInsight.png",
                    class: "side"
                }
                h2 {
                    "Short Stories"
                }
                p { "These stories are not sorted in any particular way. But it's mostly chronological." }
                ul {
                    li {
                        a {
                            href: "https://file.garden/ZJSEzoaUL3bz8vYK/BloodlessStories/FlashOfInsight.pdf",
                            "A Flash Of Insight"
                        }
                        ": It is only recently that Dr. Kätta was hired by the Blue Queen. Struggling to handle the pressure, she falls asleep in the middle of her work. In her dream that night, she sees terrible visions."
                    }
                    li {
                        a {
                            href: "https://file.garden/ZJSEzoaUL3bz8vYK/BloodlessStories/LastLecture.pdf",
                            "Kätta's Last Lecture"
                        }
                        ": A retelling of a lecture that's long since been lost to time."
                    }
                    li {
                        a {
                            href: "https://file.garden/ZJSEzoaUL3bz8vYK/BloodlessStories/TitanCarcass.pdf",
                            "A Titan Carcass"
                        }
                        ": Dr. Vats was a human scientist. The only one of his kind in The Forest at this time. Despite all of the advances he made, everything and everyone comes to an end."
                    }
                }
            }
            hr {}
        }
    }
}
