pub mod cardoids;
pub mod cards;
pub mod metadata;
pub mod types;

use std::{
    collections::{BTreeSet, HashMap},
    error::Error,
    fmt::Display,
    fs::{self},
    time::Instant,
};

use indexmap::IndexSet;
use serde::Deserialize;

use crate::utils::iter::*;

#[derive(Deserialize, Debug, Clone)]
pub struct AtomicCardsFile {
    pub meta: metadata::MetaData,
    pub data: HashMap<String, cardoids::Cardoid>,
}

impl AtomicCardsFile {
    pub fn load(verbose: bool) -> anyhow::Result<Self> {
        let (atomic_cards_file_json, start) = Self::read_or_download(verbose)?;

        let atomic_cards: AtomicCardsFile = serde_json::from_slice(&atomic_cards_file_json[..])?;

        if verbose {
            println!(
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
        const ATOMIC_CARDS_FILENAME: &'static str = "AtomicCards.json";
        const ATOMIC_CARDS_URL: &'static str = "https://mtgjson.com/api/v5/AtomicCards.json";

        let mut start: Instant;

        if !(fs::exists(ATOMIC_CARDS_FILENAME)?) {
            println!("{} not found, downloading...", ATOMIC_CARDS_FILENAME);
            start = Instant::now();

            let client = reqwest::blocking::ClientBuilder::new()
                .timeout(None)
                .build()?;

            let request = client.get(ATOMIC_CARDS_URL).build()?;

            let response = client.execute(request)?;

            let downloaded = response.bytes()?.to_vec();

            if verbose {
                println!(
                    "Downloaded {} megabytes in {} seconds.",
                    downloaded.len() / 1024 / 1000,
                    start.elapsed().as_secs()
                );

                println!("Loading cards database...");
            }
            start = Instant::now();
            std::fs::write(ATOMIC_CARDS_FILENAME, &downloaded[..])?;

            Ok((downloaded, start))
        } else {
            if verbose {
                println!("Loading cards database...");
            }
            start = Instant::now();

            Ok((std::fs::read(ATOMIC_CARDS_FILENAME)?, start))
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
