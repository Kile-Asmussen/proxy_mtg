use crate::utils::iter::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
    ops::Sub,
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
#[repr(u8)]
pub enum WUBRG {
    W,
    U,
    B,
    R,
    G,
}

impl WUBRG {
    pub fn render(colors: &BTreeSet<WUBRG>) -> String {
        match &colors.iter().map(Clone::clone).collvect()[..] {
            &[] => "C".to_string(),
            &[a] => format!("{a}"),
            &[a, b] => {
                if a - b <= 2 {
                    format!("{a}{b}")
                } else {
                    format!("{b}{a}")
                }
            }
            &[a, b, c] => {
                if a - b == 2 || c - a == 3 {
                    format!("{a}{b}{c}")
                } else if b - c == 2 || a - b == 3 {
                    format!("{b}{c}{a}")
                } else {
                    format!("{c}{a}{b}")
                }
            }
            &[a, b, c, d] => {
                if d - a == 2 {
                    format!("{a}{b}{c}{d}")
                } else if a - b == 2 {
                    format!("{b}{c}{d}{a}")
                } else if b - c == 2 {
                    format!("{c}{d}{a}{b}")
                } else {
                    format!("{d}{a}{b}{c}")
                }
            }
            &[_, _, _, _, _] => "WUBRG".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn wubrg() -> BTreeSet<WUBRG> {
        use WUBRG::*;
        BTreeSet::from_iter([W, U, B, R, G])
    }

    pub fn colorless() -> BTreeSet<WUBRG> {
        BTreeSet::new()
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

impl Sub for WUBRG {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        if self == rhs {
            0
        } else if self < rhs {
            (self as usize).abs_diff(rhs as usize)
        } else {
            (self as usize).abs_diff(rhs as usize + 5)
        }
    }
}

impl Sub for &WUBRG {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl Display for WUBRG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            WUBRG::W => "W",
            WUBRG::U => "U",
            WUBRG::B => "B",
            WUBRG::R => "R",
            WUBRG::G => "G",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum CardLayout {
    #[serde(rename = "adventure")]
    Adventure,
    #[serde(rename = "aftermath")]
    Aftermath,
    #[serde(rename = "case")]
    Case,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "flip")]
    Flip,
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
    #[serde(rename = "prototype")]
    Prototype,
    #[serde(rename = "reversible_card")]
    ReversibleCard,
    #[serde(rename = "saga")]
    Saga,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "transform")]
    Transform,
    #[serde(untagged)]
    Other(String),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FaceLayout {
    Aftermath,
    Basic,
    Battle,
    Case,
    Class,
    Creature,
    Flip,
    Fuse,
    Leveler,
    Mutate,
    Omenventure,
    Planeswalker,
    Prototype,
    Room,
    Saga,
    Split,
    Unadorned,
    Unsupported,
}

impl FaceLayout {
    pub fn is_vertical(self) -> bool {
        match self {
            Self::Case | Self::Class | Self::Saga => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LeadershipSkills {
    pub brawl: bool,
    pub commander: bool,
    pub oathbreaker: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    #[serde(rename = "a")]
    #[default]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "d")]
    D,
    #[serde(rename = "e")]
    E,
}
