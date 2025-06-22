use std::time::Instant;

use itertools::Itertools;
use rusqlite::Connection;

use crate::{
    atomic_cards::{
        cardoids::{Cardoid, Cardoid_Keys},
        metadata::MetaData,
        sqlite::SqliteTableImpl,
        AtomicCardsDb, AtomicCardsFile,
    },
    utils::ToS,
};

impl AtomicCardsFile {
    pub fn load_raw_file(verbose: bool) -> anyhow::Result<Vec<u8>> {
        if !(std::fs::exists(Self::ATOMIC_CARDS_DUMP)?) {
            return Ok(std::fs::read(Self::ATOMIC_CARDS_DUMP)?);
        }

        let start = Instant::now();
        if verbose {
            eprintln!("{} file not found, downloading...", Self::ATOMIC_CARDS_DUMP);
        }

        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(None)
            .build()?;

        let request = client.get(Self::ATOMIC_CARDS_URL).build()?;

        let response = client.execute(request)?;

        let downloaded = response.bytes()?.to_vec();

        std::fs::write(Self::ATOMIC_CARDS_DUMP, &downloaded)?;

        if verbose {
            eprintln!(
                "Downloaded {} megabytes in {} seconds.",
                downloaded.len() / 1024 / 1000,
                start.elapsed().as_secs()
            );
        }

        return Ok(downloaded);
    }

    pub fn load_json(verbose: bool) -> anyhow::Result<Self> {
        let start = Instant::now();

        let atomic_cards = if std::fs::exists(Self::ATOMIC_CARDS_FILE)? {
            let data = std::fs::read(Self::ATOMIC_CARDS_FILE)?;

            serde_json::from_slice(&data)?
        } else {
            let data = Self::load_raw_file(verbose)?;

            if verbose {
                eprintln!("{} not found, recreating...", Self::ATOMIC_CARDS_FILE);
            }

            let atomic_cards: AtomicCardsFile = serde_json::from_slice(&data)?;

            let data = serde_json::to_vec(&atomic_cards)?;

            std::fs::write(Self::ATOMIC_CARDS_FILE, data)?;

            atomic_cards
        };

        if verbose {
            eprintln!(
                "Loaded {}, {} cards in {} milliseconds.",
                Self::ATOMIC_CARDS_FILE,
                atomic_cards.data.len(),
                start.elapsed().as_millis()
            );
        }

        Ok(atomic_cards)
    }
}

impl AtomicCardsDb {
    #[allow(unused)]
    pub fn load_sqlite() -> anyhow::Result<AtomicCardsDb> {
        if std::fs::exists(Self::ATOMIC_CARDS_DB)? {
            Ok(AtomicCardsDb {
                conn: Connection::open(Self::ATOMIC_CARDS_DB)?,
            })
        } else {
            Ok(AtomicCardsDb {
                conn: Connection::open_in_memory()?,
            })
        }
    }

    #[allow(unused)]
    pub fn initialize(&self, file: &AtomicCardsFile) -> anyhow::Result<()> {
        MetaData::setup(&self.conn)?;
        Cardoid::setup(&self.conn)?;

        MetaData::store_rows([(&file.meta, &mut ())], &self.conn)?;

        let mut data = file
            .data
            .iter()
            .map(|(n, c)| (&*c, Cardoid_Keys { card_name: n.s() }))
            .collect_vec();

        Cardoid::store_rows(data.iter_mut().map(|(c, ck)| (*c, ck)), &self.conn)?;

        Ok(())
    }

    #[allow(unused)]
    pub fn save(&self) -> anyhow::Result<()> {
        self.conn.backup("main", Self::ATOMIC_CARDS_DB, None)?;
        Ok(())
    }
}
