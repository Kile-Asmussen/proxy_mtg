pub mod cardoids;
pub mod cards;
pub mod download;
pub mod metadata;
pub mod sqlite;
pub mod types;

use std::{collections::BTreeSet, error::Error, fmt::Display};

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::{
    atomic_cards::{
        cardoids::{Cardoid, Cardoid_Keys},
        metadata::MetaData,
        sqlite::SqliteTable,
    },
    utils::ToS,
};

use anyhow::anyhow;

#[derive(Debug, Default)]
pub struct AtomicCards {
    db: Option<AtomicCardsDb>,
    file: Option<AtomicCardsFile>,
}

impl AtomicCards {
    pub fn lookup(&self, cardname: &str) -> Option<Cardoid> {
        if let Some(db) = &self.db {
            todo!();
        } else if let Some(file) = &self.file {
            file.data.get(cardname.clone()).map(Clone::clone)
        } else {
            None
        }
    }

    pub fn load_db(&mut self, verbose: bool) -> anyhow::Result<&mut Self> {
        Err(anyhow!("Unimplemented"))
    }

    pub fn load_json(&mut self, verbose: bool) -> anyhow::Result<&mut Self> {
        self.file = Some(AtomicCardsFile::load_json(verbose)?);
        Ok(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn card_names(&self) -> Vec<String> {
        if let Some(db) = &self.db {
            let x = Cardoid::load_all(&db.conn)?;
            x.into_iter().map(|(_, _, ck)| ck.card_name).collect_vec();
        } else if let Some(file) = &self.file {
            file.data.get(cardname.clone()).map(Clone::clone)
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn validate(&self) -> anyhow::Result<()> {
        let mut malformed_cards = IndexSet::new();

        for name in self.card_names() {
            let Some(cardoid) = self.lookup(&name) else {
                malformed_cards.insert(name);
                continue;
            };

            if cardoid.sides().len() < 1 || !cardoid.sides().is_sorted() {
                malformed_cards.insert(name.clone());
            }

            let layouts = BTreeSet::from_iter(cardoid.iter().map(|c| c.layout.clone()));

            if layouts.len() > 1 {
                malformed_cards.insert(name.clone());
            }
        }

        if malformed_cards.is_empty() {
            Ok(())
        } else {
            Err(AtomicCardsError(malformed_cards.into_iter().collect_vec()).into())
        }
    }
}

#[derive(Debug)]
pub struct AtomicCardsDb {
    conn: Connection,
}

impl AtomicCardsDb {
    const ATOMIC_CARDS_DB: &'static str = "AtomicCards.sqlite";
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AtomicCardsFile {
    pub meta: metadata::MetaData,
    pub data: IndexMap<String, Cardoid>,
}

impl AtomicCardsFile {
    const ATOMIC_CARDS_DUMP: &'static str = "AtomicCards.dump.json";
    const ATOMIC_CARDS_FILE: &'static str = "AtomicCards.json";
    const ATOMIC_CARDS_URL: &'static str = "https://mtgjson.com/api/v5/AtomicCards.json";
}

#[derive(Debug)]
pub struct AtomicCardsError(pub Vec<String>);

impl Display for AtomicCardsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("The following atomic cards were malformed:\n")?;

        for name in &self.0 {
            f.write_fmt(format_args!("  {}\n", name))?;
        }

        Ok(())
    }
}

impl Error for AtomicCardsError {}

fn is_default<T: Default + PartialEq>(it: &T) -> bool {
    T::default() == *it
}
