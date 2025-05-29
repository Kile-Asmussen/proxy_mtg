pub mod decklist;

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    sync::atomic,
};

use rand::rand_core::block;
use serde::{Deserialize, Serialize};

use crate::atomic_cards::{
    cards::Cardoid,
    types::{Type, WUBRG},
    AtomicCardsFile,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(default, rename = "artFile")]
    pub art_file: Vec<PathBuf>,
    #[serde(default, rename = "artCredit")]
    pub art_credit: Vec<String>,
    #[serde(default, rename = "fullArt")]
    pub full_art: bool,
    #[serde(default, rename = "flavorText")]
    pub flavor_text: Vec<String>,
    #[serde(default)]
    pub copies: usize,
    #[serde(default, rename = "reminderText")]
    pub reminder_text: bool,
    #[serde(default = "repeats_default")]
    pub repeats: usize,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub tags: BTreeSet<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default, skip_serializing, skip_deserializing)]
    pub cardoid: Option<Cardoid>,
}

fn repeats_default() -> usize {
    1
}
