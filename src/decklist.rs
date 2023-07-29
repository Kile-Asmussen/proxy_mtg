use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::proxy_builder::{BasicLand, ProxyBuilder};

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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Constructed {
    pub cards: Vec<Artoid>,
    pub basics: Vec<Landoid>,
}

pub trait CardParser {
    type Output;
    type NormalCard: ProxyBuilderNormal + ProxyBuilder<Output = Self::Output>;
    type SagaCard: ProxyBuilderSaga + ProxyBuilder<Output = Self::Output>;
}
