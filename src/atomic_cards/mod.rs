pub mod cardoids;
pub mod cards;
pub mod metadata;
pub mod types;

use std::{
    collections::BTreeSet,
    error::Error,
    fmt::Display,
    fs::{self},
    time::Instant,
};

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::utils::iter::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AtomicCardsFile {
    pub meta: metadata::MetaData,
    pub data: IndexMap<String, cardoids::Cardoid>,
}

impl AtomicCardsFile {
    const ATOMIC_CARDS_FILENAME: &'static str = "AtomicCards.json";
    const ATOMIC_CARDS_URL: &'static str = "https://mtgjson.com/api/v5/AtomicCards.json";

    pub fn load(verbose: bool) -> anyhow::Result<Self> {
        let (mut atomic_cards, start) = Self::read_or_download(verbose)?;

        let atomic_cards: AtomicCardsFile = serde_json::from_slice(&mut atomic_cards[..])?;

        if verbose {
            eprintln!(
                "Loaded {} cards in {} milliseconds.",
                atomic_cards.data.len(),
                start.elapsed().as_millis()
            );
        }

        let mut malformed_cards = IndexSet::new();

        for (name, cardoid) in &atomic_cards.data {
            if cardoid.sides().len() < 1 || !cardoid.sides().is_sorted() {
                malformed_cards.insert(name.clone());
            }

            let layouts = BTreeSet::from_iter(cardoid.iter().map(|c| c.layout.clone()));

            if layouts.len() > 1 {
                malformed_cards.insert(name.clone());
            }
        }

        if malformed_cards.is_empty() {
            Ok(atomic_cards)
        } else {
            Err(AtomicCardsBuildError(malformed_cards.into_iter().collvect()).into())
        }
    }

    fn read_or_download(verbose: bool) -> anyhow::Result<(Vec<u8>, Instant)> {
        let mut start = Instant::now();

        if !(fs::exists(Self::ATOMIC_CARDS_FILENAME)?) {
            eprintln!(
                "{} file not found, downloading...",
                Self::ATOMIC_CARDS_FILENAME
            );

            let client = reqwest::blocking::ClientBuilder::new()
                .timeout(None)
                .build()?;

            let request = client.get(Self::ATOMIC_CARDS_URL).build()?;

            let response = client.execute(request)?;

            let mut downloaded = response.bytes()?.to_vec();

            if verbose {
                eprintln!(
                    "Downloaded {} megabytes in {} seconds.",
                    downloaded.len() / 1024 / 1000,
                    start.elapsed().as_secs()
                );

                eprintln!("Storing cards database...");
                start = Instant::now();
            }

            let mut reserialized = vec![];
            let cleaned: AtomicCardsFile = serde_json::from_slice(&mut downloaded[..])?;

            serde_json::to_writer(&mut reserialized, &cleaned)?;

            std::fs::write(Self::ATOMIC_CARDS_FILENAME, &reserialized[..])?;

            if verbose {
                eprintln!(
                    "Stored {} megabytes in {} milliseconds.",
                    reserialized.len() / 1024 / 1000,
                    start.elapsed().as_millis()
                );

                eprintln!("Loading cards database...");
            }

            start = Instant::now();

            Ok((reserialized, start))
        } else {
            if verbose {
                eprintln!("Loading cards database...");
            }

            Ok((std::fs::read(Self::ATOMIC_CARDS_FILENAME)?, start))
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

fn is_default<T: Default + PartialEq>(it: &T) -> bool {
    T::default() == *it
}
