use std::collections::VecDeque;
use std::sync::{Arc, LazyLock, RwLock};

use hemoglobin::cards::Card;
use serde::{Deserialize, Serialize};

use super::Global;

#[cfg(feature = "server")]
pub static CARD_CHANGED: Global<VecDeque<CardDiff>> =
    LazyLock::new(|| Arc::new(RwLock::new(VecDeque::new())));

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CardDiff {
    Changed { old: Box<Card>, new: Box<Card> },
    New(Box<Card>),
}
