use crate::utils::iter::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Supertype {
    Basic,
    Legendary,
    Snow,

    #[serde(untagged)]
    Other(String),
}

impl Display for Supertype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(s) => f.write_str(s),
            x => Debug::fmt(x, f),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Artifact,
    Battle,
    Creature,
    Enchantment,
    Instant,
    Kindred,
    Land,
    Planeswalker,
    Sorcery,

    #[serde(untagged)]
    Other(String),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(s) => f.write_str(s),
            x => Debug::fmt(x, f),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WUBRG {
    W,
    U,
    B,
    R,
    G,
}

impl WUBRG {
    pub fn wubrg(colors: &BTreeSet<WUBRG>) -> String {
        let res = colors
            .into_iter()
            .map(|c| format!("{:?}", c))
            .collvect()
            .join("");

        if res.is_empty() {
            "C".to_string()
        } else {
            res
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            WUBRG::W => "white",
            WUBRG::U => "blue",
            WUBRG::B => "black",
            WUBRG::R => "red",
            WUBRG::G => "green",
        }
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub enum Layout {
    #[serde(rename = "adventure")]
    Adventure,
    #[serde(rename = "aftermath")]
    Aftermath,
    #[serde(rename = "art_series")]
    ArtSeries,
    #[serde(rename = "augment")]
    Augment,
    #[serde(rename = "case")]
    Case,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "double_faced_token")]
    DoubleFacedToken,
    #[serde(rename = "emblem")]
    Emblem,
    #[serde(rename = "flip")]
    Flip,
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "leveler")]
    Leveler,
    #[serde(rename = "meld")]
    Meld,
    #[serde(rename = "modal_dfc")]
    ModalDfc,
    #[serde(rename = "mutate")]
    Mutate,
    #[serde(rename = "normal")]
    #[default]
    Normal,
    #[serde(rename = "planar")]
    Planar,
    #[serde(rename = "prototype")]
    Prototype,
    #[serde(rename = "reversible_card")]
    ReversibleCard,
    #[serde(rename = "saga")]
    Saga,
    #[serde(rename = "scheme")]
    Scheme,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "transform")]
    Transform,
    #[serde(rename = "vanguard")]
    Vanguard,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LeadershipSkills {
    pub brawl: bool,
    pub commander: bool,
    pub oathbreaker: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Side {
    #[serde(rename = "a")]
    #[default]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(untagged)]
    O(String),
}
