use std::{
    thread,
    time::{Duration, Instant},
};

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{
    proxy::Art,
    utils::{iter::IterExt, ToS},
};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ScryfallCard {
    #[allow(unused)]
    Error(ScryfallError),
    Single(ScryfallArt),
    Multi(ScryfallMulti),
}

impl ScryfallCard {
    pub fn arts(&self) -> Vec<Art> {
        match self {
            Self::Error(_) => vec![],
            Self::Single(a) => vec![a.art()],
            Self::Multi(m) => m.arts(),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ScryfallError {
    #[serde(rename = "object")]
    _object: Error,
    #[serde(rename = "status")]
    pub _status: usize,
    #[serde(rename = "code")]
    pub _code: String,
    #[serde(rename = "details")]
    pub _details: String,
}

#[derive(Deserialize, Default)]
#[serde(rename = "error")]
pub struct Error;

#[derive(Deserialize, Default)]
#[serde(rename = "card")]
pub struct Card;

#[derive(Deserialize, Default)]
pub struct ScryfallArt {
    #[serde(rename = "object")]
    _object: Card,
    pub artist: String,
    pub image_uris: ImageUris,
}

impl ScryfallArt {
    pub fn art(&self) -> Art {
        Art {
            credit: self.artist.s(),
            url: self.image_uris.art_crop.s(),
            full: false,
            center_text: false,
            scryfall: true,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ImageUris {
    pub art_crop: String,
}

#[derive(Deserialize)]
pub struct ScryfallMulti {
    #[serde(rename = "object")]
    _object: Card,
    pub card_faces: Vec<ScryfallArt>,
}

impl ScryfallMulti {
    pub fn arts(&self) -> Vec<Art> {
        self.card_faces.iter().map(ScryfallArt::art).collvect()
    }
}

pub struct ScryfallClient {
    pub client: Client,
    pub last: Instant,
}

impl ScryfallClient {
    const DELAY: Duration = Duration::from_millis(150);

    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(None)
            .build()?;

        Ok(Self {
            client,
            last: Instant::now()
                .checked_sub(Self::DELAY)
                .unwrap_or_else(Instant::now),
        })
    }

    pub fn get_scryfall_card_art(&mut self, card_name: &str) -> anyhow::Result<ScryfallCard> {
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

        Ok(serde_json::from_slice(&body)?)
    }
}
