use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Artoid {
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
    #[serde(default)]
    pub repeats: usize,
    #[serde(default)]
    pub notes: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckList(pub BTreeMap<String, Vec<Artoid>>);
