use std::sync::{Arc, LazyLock, RwLock};

use hemoglobin::cards::Card;

use super::Global;

#[cfg(feature = "server")]
pub static CARD_CHANGED: Global<Vec<CardDiff>> =
    LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));

pub enum CardDiff {
    Changed { old: Box<Card>, new: Box<Card> },
    New(Box<Card>),
}
