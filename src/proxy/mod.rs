pub mod decklists;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::atomic_cards::{cardoids::Cardoid, metadata::ForeignData, types::CardLayout};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(default)]
    pub arts: Vec<Art>,
    #[serde(default)]
    pub copies: usize,
    #[serde(rename = "reminderText", default = "Proxy::reminder_text_default")]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Art {
    pub url: String,
    pub credit: String,
    #[serde(default)]
    pub full: bool,
    pub center_text: bool,
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

    fn reminder_text_default() -> bool {
        true
    }
}
