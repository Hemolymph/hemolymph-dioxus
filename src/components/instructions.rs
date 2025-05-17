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
            class: "instructions",
            h1 {
                "Hemolymph Syntax Guide"
            }
            p { "Hemolymph provides more advanced syntax to make searches. Each individual meaningful element of a search is called a query. Two or more queries put together one after another will only retrieve results that match all of them." }
            p { "All the examples in this guide are clickable. They are written in bold text and yellow, like all the clickable search queries in Hemolymph." }
            hr {}
            FuzzyQuery { }
            hr {}
            StringQuery { }
            hr {}
            KinQuery { }
            hr {}
            NumberQuery { }
            hr {}
            QueryQuery { }
            hr {}
            OrAndXor { }
            hr {}
            Negation { }
            hr {}
            Sorting { }
        }
    }
}

#[component]
fn FuzzyQuery() -> Element {
    rsx! {
        section {
            h2 {
                "Basic Queries"
            }
            p {
                "A basic query consists of unstructured text, and will match all text in a card excluding flavor text. It prioritizes matches in this order."
            }
            ol {
                li { "Names" }
                li { "Kins" }
                li { "Keywords" }
                li { "Description" }
                li { "Type line" }
            }
            p {
                "All basic queries inside a single query group will be put together as one."
            }
            section {
                h3 {
                    "Examples"
                }
                Examples {
                    examples: [
                        ("dr vats", r#"all cards that contain the text "dr vats" anywhere"#),
                    ]
                }
            }
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
            table {
                class: "aliases",
                tr {
                    th { "Property" }
                    th { "Aliases" }
                }
                NameAlias { property: "name", aliases: ["n"] }
                NameAlias { property: "description", aliases: ["desc"] }
                NameAlias { property: "kin", aliases: ["k"] }
                NameAlias { property: "keyword", aliases: ["kw"] }
                NameAlias { property: "flavortext", aliases: ["ft"] }
            }
            StringQueryEqualityNotice { }
            section {
                h3 {
                    "Examples"
                }
                Examples {
                    examples: [
                        ("n:ant", "all cards with \"ant\" in their name"),
                        ("n=\"lost man\"", "all cards named exactly \"lost man\""),
                        ("n=/.*/", "all cards whose name matches the regex /.*/"),
                    ]
                }
            }
        }
    }
}

#[component]
fn KinQuery() -> Element {
    rsx! {
        section {
            h2 {
                "Kin Queries"
            }
            p {
                "Kin queries are similar to text queries, but are aware of the kin tree whenever you type a fully recognized kin."
            }
            p {
                "If your query has more than one word, you must use double quotes ("
                code { "\"" }
                ") around it."
            }
            p {
                "Using a colon ("
                code { ":" }
                ") will retrieve any result of the same kin that you're looking for, respecting the Kin Tree, whereas an equals sign ("
                code { "=" }
                ") will match only results that are "
                em { "exactly"}
                " of the kin you're searching. Most of the time, you want to use a colon."
            }
            p {
                "If the string you look for is not recognized as a kin, the search will be interpreted as a text query instead. You can also use a Regex."
            }


            section {
                h3 {
                    "Examples"
                }
                Examples {
                    examples: [
                        ("k:ant", "all ant kin cards" ),
                        ("k:insect", "all insect kin cards" ),
                        ("k=insect", "all cards of exactly insect kin" ),
                        ("k=sorc", r#"all cards whose kin is equal to the string "sorc""# ),
                        (r#"k:"blue k""#, r#"all cards whose kin contains the string "blue k""# ),
                    ]
                }
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
            table {
                class: "aliases",
                tr {
                    th { "Property" }
                    th { "Aliases" }
                }
                NameAlias { property: "cost", aliases: ["c"] }
                NameAlias { property: "health", aliases: ["h"] }
                NameAlias { property: "defense", aliases: ["d"] }
                NameAlias { property: "power", aliases: ["p"] }
            }

            section {
                h3 {
                    "Examples"
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
}

#[component]
fn QueryQuery() -> Element {
    rsx! {
        section {
            h2 {
                "Recursive Queries"
            }
            p {
                "The following properties are matched by another query group, which is written in parentheses:"
            }
            table {
                class: "aliases",
                tr {
                    th { "Property" }
                    th { "Aliases" }
                }
                NameAlias { property: "devours", aliases: ["devs"] }
                NameAlias { property: "devouredby", aliases: ["dby"] }
            }

            section {
                h3 {
                    "Examples"
                }
                Examples {
                    examples: [
                        ("devs: (c>2)", "all cards that devour cards with cost greater than 2"),
                        ("dby: (mantis c>2)", "all cards devoured by cards with cost greater than 2 and have the word mantis written on them"),
                    ]
                }
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
                "You can match cards that match one query group or another using OR"
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
                    ("- h=2", "all cards whose cost is not equal to 2"),
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
                "You can put a SORT clause in the outermost query group of your search. If no SORT clause is put, results will be sorted alphabetically, unless there is a basic query in the outermost query group. In that case they will be sorted by how closely they match the basic query."
            }
            section {
                h3 {
                    "Examples"
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
        Some(aliases) => aliases,
        None => rsx! { "None." },
    };

    rsx! {
        tr {
            td { code { "{property}" } }
            td { { alias_text } }
        }
    }
}

#[component]
fn Examples<const N: usize>(examples: [(&'static str, &'static str); N]) -> Element {
    let examples = examples;
    rsx! {
        table {
            class: "examples",
            tr {
                th { "Query" }
                th { "Will match..." }
            }
            for idx in 0..N {
                tr {
                    td {
                        QueryHemolink {
                            query: "{examples[idx].0}", display: "{examples[idx].0}"
                        }
                    }
                    td {
                        i { "{examples[idx].1}" }
                    }
                }
            }
        }
    }
}
