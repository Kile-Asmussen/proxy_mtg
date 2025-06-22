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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AtomicCardsFile {
    pub meta: metadata::MetaData,
    pub data: IndexMap<String, Cardoid>,
}

impl AtomicCardsFile {
    const ATOMIC_CARDS_DUMP: &'static str = "AtomicCards.dump.json";
    const ATOMIC_CARDS_FILE: &'static str = "AtomicCards.json";
    const ATOMIC_CARDS_DB: &'static str = "AtomicCards.sqlite";
    const ATOMIC_CARDS_URL: &'static str = "https://mtgjson.com/api/v5/AtomicCards.json";

    pub fn store(&self, conn: &Connection) -> anyhow::Result<()> {
        MetaData::setup(conn)?;
        Cardoid::setup(conn)?;

        MetaData::store_rows([(&self.meta, &mut ())], conn)?;

        let mut data = self
            .data
            .iter()
            .map(|(n, c)| (&*c, Cardoid_Keys { card_name: n.s() }))
            .collect_vec();

        Cardoid::store_rows(data.iter_mut().map(|(c, ck)| (*c, ck)), conn)?;

        Ok(())
    }

    pub fn load(conn: &Connection) -> anyhow::Result<AtomicCardsFile> {
        let meta = MetaData::load_rows([1], conn)?
            .pop()
            .ok_or(anyhow!("No metadata"))?
            .1;

        let data = IndexMap::from_iter(
            Cardoid::load_all(conn)?
                .into_iter()
                .map(|(_, c, ck)| (ck.card_name, c)),
        );

        Ok(Self { meta, data })
    }

    #[allow(unused)]
    pub fn validate(&self) -> anyhow::Result<()> {
        let mut malformed_cards = IndexSet::new();

        for (name, cardoid) in &self.data {
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
            Err(AtomicCardsBuildError(malformed_cards.into_iter().collect_vec()).into())
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
