use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::path::Path;
use std::sync::mpsc::TryRecvError;
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, RwLock},
    time::Duration,
};

use dioxus::prelude::*;
use hemoglobin::cards::Card;

#[cfg(feature = "server")]
thread_local! {
    pub static CARDS: Arc<RwLock<HashMap<String, Card>>> = {
        let data = fs::read_to_string("../hemolymph-static/files/cards.json").expect("Unable to read cards.json");
        let cards = serde_json::from_str::<Vec<Card>>(&data).unwrap();
        Arc::new(RwLock::new(create_card_map(cards)))
    };
}

#[server(endpoint = "search")]
pub async fn process_query(query: String) -> Result<Vec<Card>, ServerFnError> {
    if query.trim().is_empty() {
        return Ok(vec![]);
    }

    let query_s = hemoglobin::search::query_parser::parse_query(&query);
    let query = query_s.map_err(|x| ServerFnError::new("Failed to parse query."))?;
    let cards: Vec<_> = CARDS.with(|cards| {
        let cards = cards.read().unwrap();
        hemoglobin::search::search(&query, cards.values())
            .into_iter()
            .cloned()
            .collect()
    });

    Ok(cards)
}

#[server(endpoint = "card")]
pub async fn get_card_id(id: String) -> Result<Card, ServerFnError> {
    CARDS.with(|cards| {
        cards
            .read()
            .unwrap()
            .get(&id)
            .ok_or(ServerFnError::new("Id does not exist"))
            .cloned()
    })
}

#[cfg(feature = "server")]
fn create_card_map(vec: Vec<Card>) -> HashMap<String, Card> {
    vec.into_iter().map(|x| (x.id.clone(), x)).collect()
}

#[cfg(feature = "server")]
pub fn setup_card_debounce() {
    use std::thread::sleep;

    std::thread::spawn(|| {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_secs(1), tx).unwrap();
        debouncer
            .watcher()
            .watch(
                Path::new("../hemolymph-static/files"),
                RecursiveMode::Recursive,
            )
            .unwrap();
        loop {
            match rx.try_recv() {
                Ok(Ok(events)) => {
                    for event in events {
                        if event.path.ends_with("cards.json") {
                            match fs::read_to_string("../hemolymph-static/files/cards.json") {
                                Ok(data) => match serde_json::from_str::<Vec<Card>>(&data) {
                                    Ok(data) => {
                                        CARDS.with(|cards| {
                                            let mut cards = cards.write().unwrap();
                                            *cards = create_card_map(data);
                                        });
                                        // let mut cards = CARDS.write().await;
                                        // *cards = create_card_map(data);
                                        println!("Successfully reloaded cards.json");
                                    }
                                    Err(x) => eprintln!("Failed to load cards.json: {x:#?}"),
                                },
                                Err(x) => eprintln!("Failed to read cards.json: {x:#?}"),
                            }
                        }
                    }
                }
                Ok(Err(error)) => eprintln!("Failed to watch: {error:#?}"),
                Err(TryRecvError::Disconnected) => {
                    eprintln!("File watcher was disconnected. This should not happen.");
                }
                Err(TryRecvError::Empty) => (),
            }
            sleep(Duration::from_secs(0));
        }
    });
}
