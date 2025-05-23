mod card_diff;
pub use card_diff::CardDiff;
#[cfg(feature = "server")]
use card_diff::CARD_CHANGED;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::path::Path;
use std::sync::LazyLock;
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, RwLock},
    time::Duration,
};

use dioxus::prelude::*;
use hemoglobin::cards::Card;

type Global<T> = LazyLock<Arc<RwLock<T>>>;

#[cfg(feature = "server")]
pub static CARDS: Global<HashMap<String, Card>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[server(endpoint = "search")]
pub async fn process_query(query: String) -> Result<Vec<Card>, ServerFnError> {
    if query.trim().is_empty() {
        return Ok(vec![]);
    }

    let query_s = hemoglobin_search::query_parser::parse_query(&query);
    let query =
        query_s.map_err(|x| ServerFnError::new(format!("Failed to parse query: {x:#?}")))?;
    let cards: Vec<_> = {
        let cards = CARDS.read().unwrap();
        hemoglobin_search::search(&query, cards.values())
            .into_iter()
            .cloned()
            .collect()
    };

    Ok(cards)
}

#[server(endpoint = "card")]
pub async fn get_card_id(id: String) -> Result<Card, ServerFnError> {
    CARDS
        .read()
        .unwrap()
        .get(&id)
        .ok_or_else(|| ServerFnError::new("Id does not exist"))
        .cloned()
}

#[cfg(feature = "server")]
fn create_card_map(vec: Vec<Card>) -> HashMap<String, Card> {
    vec.into_iter().map(|x| (x.id.clone(), x)).collect()
}

#[cfg(feature = "server")]
pub fn setup_card_debounce() {
    use std::thread::sleep;

    match load_cards_json(false) {
        Ok(()) => println!("Successful first load of cards.json"),
        Err(LoadError::IoError(err)) => {
            eprintln!("Failed to read cards.json: {err:#?} from disk on first initialization")
        }
        Err(LoadError::SerdeJsonError(err)) => {
            eprintln!("Failed to deserialize cards.json: {err:#?} on first initialization")
        }
    }

    std::thread::spawn(|| {
        let mut debouncer = new_debouncer(Duration::from_secs(1), watcher_response).unwrap();
        loop {
            debouncer
                .watcher()
                .watch(
                    Path::new("../hemolymph-static/files"),
                    RecursiveMode::Recursive,
                )
                .unwrap();

            sleep(Duration::from_millis(10));
        }
    });
}

#[derive(Debug)]
enum LoadError {
    IoError(std::io::Error),
    SerdeJsonError(serde_json::Error),
}

#[cfg(feature = "server")]
fn watcher_response(events: DebounceEventResult) {
    match events {
        Ok(events) => {
            for event in events {
                if !event.path.ends_with("cards.json") {
                    continue;
                }
                match load_cards_json(true) {
                    Ok(()) => println!("Successfully reloaded cards.json"),
                    Err(LoadError::IoError(err)) => {
                        eprintln!("Failed to read cards.json: {err:#?} from disk after debounce")
                    }
                    Err(LoadError::SerdeJsonError(err)) => {
                        eprintln!("Failed to deserialize cards.json: {err:#?} after debounce")
                    }
                }
                break;
            }
        }
        Err(error) => eprintln!("Failed to watch: {error:#?}"),
    }
}

#[cfg(feature = "server")]
fn load_cards_json(generate_changes: bool) -> Result<(), LoadError> {
    use card_diff::{CardDiff, CARD_CHANGED};

    match fs::read_to_string("../hemolymph-static/files/cards.json") {
        Ok(data) => match serde_json::from_str::<Vec<Card>>(&data) {
            Ok(data) => {
                let new_map = create_card_map(data);
                if generate_changes {
                    let cards = CARDS.write().unwrap();
                    for (id, new_card) in &new_map {
                        if let Some(old_card) = cards.get(id) {
                            if new_card == old_card {
                                continue;
                            }
                            let mut cards_changed = CARD_CHANGED.write().unwrap();
                            cards_changed.push_front(CardDiff::Changed {
                                old: Box::new(old_card.clone()),
                                new: Box::new(new_card.clone()),
                            });
                        } else {
                            let mut cards_changed = CARD_CHANGED.write().unwrap();
                            cards_changed.push_front(CardDiff::New(Box::new(new_card.clone())));
                        }
                    }
                }
                let mut cards = CARDS.write().unwrap();
                *cards = new_map;
                drop(cards);
                Ok(())
            }
            Err(x) => Err(LoadError::SerdeJsonError(x)),
        },
        Err(x) => Err(LoadError::IoError(x)),
    }
}

#[server(endpoint = "changes")]
pub async fn card_changes() -> Result<VecDeque<CardDiff>, ServerFnError> {
    Ok(CARD_CHANGED.read().unwrap().clone())
}
