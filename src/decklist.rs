use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::proxy_builder::BasicLand;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Artoid {
    pub name: String,
    #[serde(default)]
    pub art_file: PathBuf,
    #[serde(default)]
    pub art_credit: String,
    #[serde(default)]
    pub flavor_text: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Landoid {
    pub name: BasicLand,
    pub number: usize,
    #[serde(default)]
    pub art_file: PathBuf,
    #[serde(default)]
    pub art_credit: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DeckEDH {
    pub commanders: Vec<Artoid>,
    pub the_99ish: Vec<Artoid>,
    pub basics: Vec<Landoid>,
}
