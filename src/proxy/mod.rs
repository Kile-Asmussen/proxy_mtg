pub mod decklists;

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    sync::atomic,
};

use rand::rand_core::block;
use serde::{Deserialize, Serialize};

use crate::atomic_cards::{cardoids::Cardoid, metadata::ForeignData, types::*, AtomicCardsFile};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(default, rename = "artFile")]
    pub art_urls: Vec<String>,
    #[serde(default, rename = "artCredit")]
    pub art_credits: Vec<String>,
    #[serde(default, rename = "fullArt")]
    pub full_art: Vec<bool>,
    #[serde(default, rename = "flavorText")]
    pub flavor_text: Vec<String>,
    #[serde(default)]
    pub copies: usize,
    #[serde(default, rename = "reminderText")]
    pub reminder_text: bool,
    #[serde(default = "Proxy::repeats_default")]
    pub repeats: usize,
    #[serde(default)]
    pub sideboard: bool,
    #[serde(default)]
    pub token: bool,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub tags: BTreeSet<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub customize: Vec<ForeignData>,
    #[serde(default)]
    pub cardoid: Cardoid,
}

impl Proxy {
    pub fn layout(&self) -> &Layout {
        (&self.cardoid).layout()
    }

    pub fn in_deck(&self) -> bool {
        !(self.sideboard || self.layout() == &Layout::Token)
    }

    fn repeats_default() -> usize {
        1
    }
}

impl Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cardoid.fmt(f);

        f.write_str("\n> # # #")?;

        f.write_fmt(format_args!("\n> category: {}", self.category))?;
        if !self.tags.is_empty() {
            f.write_str("\n> tags: ");
            f.write_str(
                &self
                    .tags
                    .iter()
                    .map(Clone::clone)
                    .collect::<Vec<_>>()
                    .join(", "),
            )?;
        }

        if self.repeats > 1 {
            f.write_fmt(format_args!("> copies: {}", self.repeats))?;
        }

        return Ok(());
    }
}
