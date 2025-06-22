use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    proxy::Art,
    scryfall::tags::{DeserializeAsTag, Tag},
    utils::ToS,
};

#[derive(Deserialize, Serialize, Clone)]
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
pub(crate) fn scryfall_card() -> anyhow::Result<()> {
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

#[derive(Deserialize, Serialize, Clone)]
pub struct ScryfallError {
    #[serde(rename = "object")]
    pub(crate) _object: Tag<Error>,
    #[serde(rename = "status")]
    pub _status: usize,
    #[serde(rename = "code")]
    pub _code: String,
    #[serde(rename = "details")]
    pub _details: String,
}

#[derive(Default, Serialize, Clone, Copy)]
pub struct Error;

impl DeserializeAsTag for Error {
    const TAG: &'static str = "error";
}

#[test]
pub(crate) fn scryfall_error() -> anyhow::Result<()> {
    serde_json::from_str::<ScryfallError>(
        r#"{"object":"error","status":404,"code":"not_found","details":"Blah"}"#,
    )?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ScryfallArt {
    pub artist: String,
    pub image_uris: ImageUris,
}

#[derive(Default, Clone, Copy)]
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
            scryfall: true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ImageUris {
    pub art_crop: String,
}

#[derive(Deserialize, Serialize, Clone)]
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
pub(crate) fn scryfall_single() -> anyhow::Result<()> {
    serde_json::from_str::<ScryfallSingle>(
        r#"{"object":"card","artist":"Frida Kahlo","image_uris":{"art_crop":"http://localhost:80/"}}"#,
    )?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ScryfallMulti {
    #[serde(rename = "object")]
    pub(crate) _object: Tag<Card>,
    pub card_faces: Vec<ScryfallArt>,
}

impl ScryfallMulti {
    pub fn arts(&self) -> Vec<Art> {
        self.card_faces.iter().map(ScryfallArt::art).collect_vec()
    }
}

#[test]
pub(crate) fn scryfall_multi() -> anyhow::Result<()> {
    serde_json::from_str::<ScryfallMulti>(
        r#"{"object":"card","card_faces":[{"artist":"Frida Kahlo","image_uris":{"art_crop":"http://localhost:80/"}}]}"#,
    )?;
    Ok(())
}
