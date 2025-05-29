pub mod cards;
pub mod metadata;
pub mod types;

use std::{
    collections::{HashMap},
    error::Error,
    fmt::{},
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::map::IntoIter;

use crate::vec_entry::IterExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtomicCardsFile {
    pub meta: metadata::MetaData,
    pub data: HashMap<String, cards::Cardoid>,
}

impl AtomicCardsFile {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let atomic_cards_file_json = std::fs::read("AtomicCards.json")?;
        let atomic_cards: AtomicCardsFile = serde_json::from_slice(&atomic_cards_file_json[..])?;

        return Ok(atomic_cards);
    }
}

