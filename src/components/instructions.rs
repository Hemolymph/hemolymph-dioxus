use dioxus::prelude::*;

use crate::{components::QueryHemolink, Query};

#[component]
pub fn Instructions() -> Element {
    use_effect(move || {
        let mut ctx = consume_context::<Query>();
        ctx.has_been_filled.set(true);
    });
    rsx! {
        section {
            h1 {
                "Hemolymph Syntax Guide"
            }
            "All the examples in this guide are clickable. They are written in bold text and yellow, like all the clickable search queries in Hemolymph."
            StringQuery { }
            NumberQuery { }
            QueryQuery { }
            OrAndXor { }
            Negation { }
            Sorting { }
        }
    }
}

#[component]
fn StringQuery() -> Element {
    rsx! {
        section {
            h2 {
                "Text Queries"
            }
            p {
                "You can match all string properties in a card. Here are some aliases for certain properties:"
            }
            ul {
                NameAlias { property: "name", aliases: ["n"] }
                NameAlias { property: "description", aliases: ["desc"] }
                NameAlias { property: "kin", aliases: ["k"] }
                NameAlias { property: "keyword", aliases: ["kw"] }
                NameAlias { property: "flavortext", aliases: ["ft"] }
            }
            StringQueryEqualityNotice { }
            Examples {
                examples: [
                    ("n:ant", "all cards with \"ant\" in their name"),
                    ("n=\"lost man\"", "all cards named exactly \"lost man\""),
                ]
            }
        }
    }
}

#[component]
fn NumberQuery() -> Element {
    rsx! {
        section {
            h2 {
                "Number Queries"
            }
            p {
                "You can match all number properties in a card."
            }
            ul {
                NameAlias { property: "cost", aliases: ["c"] }
                NameAlias { property: "health", aliases: ["h"] }
                NameAlias { property: "defense", aliases: ["d"] }
                NameAlias { property: "power", aliases: ["p"] }
            }

            Examples {
                examples: [
                    ("c>3", "all cards with cost greater than 3"),
                    ("d!=2", "all cards with defense different from 2"),
                    ("p<=2", "all cards with power less than or equal to 2"),
                ]
            }
        }
    }
}

#[component]
fn QueryQuery() -> Element {
    rsx! {
        section {
            h2 {
                "Recursive Queries"
            }
            p {
                "The following properties are matched another query, which is written in parentheses:"
            }
            ul {
                NameAlias { property: "devours", aliases: ["devs"] }
                NameAlias { property: "devouredby", aliases: ["dby"] }
            }

            Examples {
                examples: [
                    ("devs: (c>2)", "all cards that devour cards with cost greater than 2"),
                    ("dby: (mantis c>2)", "all cards devoured by cards with cost greater than 2 that also have the word mantis written on them"),
                ]
            }
        }
    }
}

#[component]
fn OrAndXor() -> Element {
    rsx! {
        section {
            h2 {
                "OR and XOR"
            }
            p {
                "You can match cards that match one query or another using OR"
            }
            Examples {
                examples: [
                    ("c>2 OR p=1", "all cards with cost greater than 2 or power equal to 1, or both"),
                ]
            }
            p {
                "You can match cards that match only one of two queries using XOR"
            }
            Examples {
                examples: [
                    ("c>2 XOR p=1", "all cards with cost greater than 2 or power equal to 1, but not both"),
                ]
            }
            p {
                "You can use parentheses to create more complex queries."
            }
        }
    }
}

#[component]
fn Negation() -> Element {
    rsx! {
        section {
            h2 {
                "Negation"
            }
            p {
                "You can match cards that do not match a query using a dash ("
                code { "-" }
                ")."
            }
            Examples {
                examples: [
                    ("-h=2", "all cards whose cost is not equal to 2"),
                ]
            }
            p {
                "Notably, this does not match cards that do not have health. There may be situations where you want this, so there exists a lenient negation with an exclamation mark ("
                code { "!" }
                ")."
            }
            Examples {
                examples: [
                    ("!p>2", "all cards without power greater than 2 (including ones that don't have a power stat)"),
                ]
            }
            p {
                "You can use parentheses to create more complex negations."
            }
            Examples {
                examples: [
                    ("!(p=2 XOR c=3)", "all cards that either have power=2 and cost=3, or have neither of those"),
                ]
            }
        }
    }
}

#[component]
fn Sorting() -> Element {
    rsx! {
        section {
            h2 {
                "Sorting"
            }
            p {
                "You can put a SORT clause in the outermost query of your search."
            }
            Examples {
                examples: [
                    ("p=2 SORT c ascending", "all cards with power=2, sorted by cost, ascending"),
                    ("d=3 h=1 SORT n descending", "all cards with defense=3, health=2, sorted alphabetically by name, descending"),
                ]
            }
        }
    }
}

#[component]
fn StringQueryEqualityNotice() -> Element {
    rsx! {
        p {
            "If your match has more than one word, you must use double quotes ("
            code { "\"" }
            ") around it."
        }
        p {
            "With all string queries, using a colon ("
            code { ":" }
            ") will retrieve any result that contains what you're looking for, whereas an equals sign ("
            code { "=" }
            ") will match only results that are "
            em { "exactly"}
            " the text you're searching. Most of the time, you want to use a colon."
        }
        p {
            "You can also use regex."
        }
    }
}

#[component]
fn NameAlias<const N: usize>(property: &'static str, aliases: [&'static str; N]) -> Element {
    let aliases = aliases
        .into_iter()
        .map(|x| rsx! { code { "{x}" } })
        .reduce(|acc, el| rsx! { {acc} "," {el} });

    let alias_text = match aliases {
        Some(aliases) => rsx! {
            "has the following aliases: "
            {aliases}
        },
        None => rsx! {"has no aliases."},
    };

    rsx! {
        li {
            code { "{property}" }
            " "
            { alias_text }
        }
    }
}

#[component]
fn Examples<const N: usize>(examples: [(&'static str, &'static str); N]) -> Element {
    let examples = examples;
    rsx! {
        ul {
            for idx in 0..N {
                li {
                    QueryHemolink {
                        query: "{examples[idx].0}", display: "{examples[idx].0}"
                    }
                    " will match "
                    i { "{examples[idx].1}" }
                }
            }
        }
    }
}
