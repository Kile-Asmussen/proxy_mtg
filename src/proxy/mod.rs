pub mod decklists;

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
};

use rand::rand_core::block;
use serde::{Deserialize, Serialize};

use crate::{
    atomic_cards::{cardoids::Cardoid, metadata::ForeignData, types::CardLayout},
    utils::iter::IterExt,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(default, rename = "artFile")]
    pub art_urls: Vec<String>,
    #[serde(default, rename = "artCredit")]
    pub art_credits: Vec<String>,
    #[serde(default, rename = "artCredit")]
    pub full_art: Vec<bool>,
    #[serde(default)]
    pub copies: usize,
    #[serde(default, rename = "reminderText")]
    pub reminder_text: bool,
    #[serde(default = "Proxy::repeats_default")]
    pub repeats: usize,
    #[serde(default)]
    pub sideboard: bool,
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
    pub fn layout(&self) -> &CardLayout {
        (&self.cardoid).layout()
    }

    pub fn in_deck(&self) -> bool {
        !(self.sideboard || self.layout() == &CardLayout::Token)
    }

    fn repeats_default() -> usize {
        1
    }
}

impl Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cardoid.fmt(f);

        if f.alternate() {
            write!(f, "\n>")?;

            if !self.category.is_empty() {
                write!(f, "\n> category: {}", self.category)?;
            }

            if !self.tags.is_empty() {
                write!(
                    f,
                    "\n> tags: {}",
                    &self.tags.iter().map(Clone::clone).collvect().join(", ")
                )?;
            }

            if self.repeats > 1 {
                write!(f, "> copies: {}", self.repeats)?;
            }
        } else {
            write!(f, "###")?;

            if !self.category.is_empty() {
                write!(f, "\ncategory: {}", self.category)?;
            }

            if !self.tags.is_empty() {
                write!(
                    f,
                    "\ntags: {}",
                    &self.tags.iter().map(Clone::clone).collvect().join(", ")
                )?;
            }

            if self.repeats > 1 {
                write!(f, "copies: {}", self.repeats)?;
            }
        }

        return Ok(());
    }
}
