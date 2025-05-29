pub mod cards;
pub mod metadata;
pub mod types;

use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::map::IntoIter;

use crate::utils::iter::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtomicCardsFile {
    pub meta: metadata::MetaData,
    pub data: HashMap<String, cards::Cardoid>,
}

impl AtomicCardsFile {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let atomic_cards_file_json = std::fs::read("AtomicCards.json")?;
        let atomic_cards: AtomicCardsFile = serde_json::from_slice(&atomic_cards_file_json[..])?;

        let mut malformed_cards = vec![];

        for (name, cardoid) in &atomic_cards.data {
            if cardoid.sides().len() < 1 || !cardoid.sides().is_sorted() {
                malformed_cards.push(name.clone())
            }
        }

        if malformed_cards.is_empty() {
            Ok(atomic_cards)
        } else {
            Err(Box::new(AtomicCardsBuildError(malformed_cards)))
        }
    }
}

#[derive(Debug)]
pub struct AtomicCardsBuildError(pub Vec<String>);

impl Display for AtomicCardsBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("The following atomic cards were malformed:\n")?;

        for name in &self.0 {
            f.write_fmt(format_args!("  {}\n", name))?;
        }

        Ok(())
    }
}

impl Error for AtomicCardsBuildError {}
