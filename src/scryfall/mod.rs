pub mod api;
mod tags;

use std::{
    collections::{btree_map::Entry, BTreeMap},
    thread,
    time::{Duration, Instant},
};

use api::ScryfallCard;

use reqwest::blocking::Client;

use crate::utils::ToS;

pub struct ScryfallClient {
    pub client: Client,
    pub last: Instant,
    pub database: BTreeMap<String, ScryfallCard>,
}

impl ScryfallClient {
    const DELAY: Duration = Duration::from_millis(150);
    const SCRYFALL_FILE: &str = "ScryfallCache.json";

    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(None)
            .build()?;

        let database = match std::fs::read_to_string(Self::SCRYFALL_FILE) {
            Ok(file) => serde_json::from_str(&file)?,
            Err(_) => BTreeMap::new(),
        };

        Ok(Self {
            client,
            last: Instant::now()
                .checked_sub(Self::DELAY)
                .unwrap_or_else(Instant::now),
            database,
        })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        std::fs::write(Self::SCRYFALL_FILE, serde_json::to_string(&self.database)?)?;
        Ok(())
    }

    pub fn get_scryfall_card_art(&mut self, card_name: &str) -> anyhow::Result<ScryfallCard> {
        let entry = self.database.entry(card_name.s());

        if let Entry::Occupied(o) = entry {
            return Ok(o.get().clone());
        }

        eprintln!("Downloading Scryfall art for {}", card_name);

        thread::sleep(Self::DELAY.saturating_sub(self.last.elapsed()));

        let request = self
            .client
            .get(format!("https://api.scryfall.com/cards/named/"))
            .query(&[("exact", card_name)])
            .header(
                "User-Agent",
                format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            )
            .header("Accepts", "text/json")
            .build()?;

        let response = self.client.execute(request)?;

        self.last = Instant::now();

        let body = response.bytes()?.to_vec();

        Ok(entry.or_insert(serde_json::from_slice(&body)?).clone())
    }
}
