mod tags;

use std::{
    thread,
    time::{Duration, Instant},
};

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::{
    proxy::Art,
    scryfall::tags::{DeserializeAsTag, Tag},
    utils::{iter::IterExt, ToS},
};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScryfallCard {
    Error(ScryfallError),
    Single(ScryfallSingle),
    Multi(ScryfallMulti),
}

impl ScryfallCard {
    pub fn arts(&self) -> Vec<Art> {
        match self {
            Self::Error(_) => vec![],
            Self::Single(s) => s.arts(),
            Self::Multi(m) => m.arts(),
        }
    }
}

#[test]
fn scryfall_card() -> anyhow::Result<()> {
    match serde_json::from_str::<ScryfallCard>(
        r#"{"object":"error","status":404,"code":"not_found","details":"Blah"}"#,
    )? {
        ScryfallCard::Error(_) => Ok(()),
        _ => Err(anyhow::anyhow!("Not Error")),
    }?;

    match serde_json::from_str::<ScryfallCard>(
        r#"{"object":"card","artist":"Frida Kahlo","image_uris":{"art_crop":"http://localhost:80/"}}"#,
    )? {
        ScryfallCard::Single(_) => Ok(()),
        _ => Err(anyhow::anyhow!("Not Single")),
    }?;

    match serde_json::from_str::<ScryfallCard>(
        r#"{"object":"card","card_faces":[{"artist":"Frida Kahlo","image_uris":{"art_crop":"http://localhost:80/"}}]}"#,
    )? {
        ScryfallCard::Multi(_) => Ok(()),
        _ => Err(anyhow::anyhow!("Not Multi")),
    }?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct ScryfallError {
    #[serde(rename = "object")]
    _object: Tag<Error>,
    #[serde(rename = "status")]
    pub _status: usize,
    #[serde(rename = "code")]
    pub _code: String,
    #[serde(rename = "details")]
    pub _details: String,
}

#[derive(Default, Serialize)]
pub struct Error;
impl DeserializeAsTag for Error {
    const TAG: &'static str = "error";
}

#[test]
fn scryfall_error() -> anyhow::Result<()> {
    serde_json::from_str::<ScryfallError>(
        r#"{"object":"error","status":404,"code":"not_found","details":"Blah"}"#,
    )?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct ScryfallArt {
    pub artist: String,
    pub image_uris: ImageUris,
}

#[derive(Default)]
pub struct Card;
impl DeserializeAsTag for Card {
    const TAG: &'static str = "card";
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

#[derive(Deserialize, Serialize)]
pub struct ImageUris {
    pub art_crop: String,
}

#[derive(Deserialize, Serialize)]
pub struct ScryfallSingle {
    #[serde(rename = "object")]
    pub _object: Tag<Card>,
    #[serde(flatten)]
    pub art: ScryfallArt,
}

impl ScryfallSingle {
    pub fn arts(&self) -> Vec<Art> {
        vec![self.art.art()]
    }
}

#[test]
fn scryfall_single() -> anyhow::Result<()> {
    serde_json::from_str::<ScryfallSingle>(
        r#"{"object":"card","artist":"Frida Kahlo","image_uris":{"art_crop":"http://localhost:80/"}}"#,
    )?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct ScryfallMulti {
    #[serde(rename = "object")]
    _object: Tag<Card>,
    pub card_faces: Vec<ScryfallArt>,
}

impl ScryfallMulti {
    pub fn arts(&self) -> Vec<Art> {
        self.card_faces.iter().map(ScryfallArt::art).collvect()
    }
}

#[test]
fn scryfall_multi() -> anyhow::Result<()> {
    serde_json::from_str::<ScryfallMulti>(
        r#"{"object":"card","card_faces":[{"artist":"Frida Kahlo","image_uris":{"art_crop":"http://localhost:80/"}}]}"#,
    )?;
    Ok(())
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
        println!("Downloading Scryfall art for {}", card_name);

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

        println!("{}", String::from_utf8(body.clone())?);

        Ok(serde_json::from_slice(&body)?)
    }
}
