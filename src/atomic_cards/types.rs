use super::is_default;
use crate::utils::ToS;
use itertools::Itertools;
use rusqlite::{
    types::{ToSqlOutput, Value},
    ToSql,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
    ops::Sub,
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Supertype {
    Basic,
    Legendary,
    Snow,

    #[serde(other)]
    Unsupported,
}

impl Display for Supertype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<u8> for Supertype {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Basic,
            1 => Self::Legendary,
            2 => Self::Snow,
            _ => Self::Unsupported,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
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

    #[serde(other)]
    Unsupported,
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        const LO: u8 = Type::Artifact as u8;
        const HI: u8 = Type::Unsupported as u8;
        match value {
            LO..=HI => unsafe { std::mem::transmute(value as u8) },
            _ => Self::Unsupported,
        }
    }
}

impl Type {
    pub fn plural(&self) -> Option<String> {
        match self {
            Type::Sorcery => Some("Sorceries".s()),
            Type::Kindred | Type::Unsupported => None,
            rest => Some(format!("{}s", rest)),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Pie {
    W,
    U,
    B,
    R,
    G,
}

impl Pie {
    pub fn name(&self) -> &'static str {
        match self {
            Pie::W => "white",
            Pie::U => "blue",
            Pie::B => "black",
            Pie::R => "red",
            Pie::G => "green",
        }
    }
}

impl TryFrom<char> for Pie {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'W' | 'w' => Self::W,
            'U' | 'u' => Self::U,
            'B' | 'b' => Self::B,
            'R' | 'r' => Self::R,
            'G' | 'g' => Self::G,
            _ => return Err(()),
        })
    }
}

impl Sub for Pie {
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

impl Sub for &Pie {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl Display for Pie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Pie::W => "W",
            Pie::U => "U",
            Pie::B => "B",
            Pie::R => "R",
            Pie::G => "G",
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct WUBRG(pub BTreeSet<Pie>);

impl WUBRG {
    pub fn wubrg() -> WUBRG {
        use Pie::*;
        WUBRG(BTreeSet::from_iter([W, U, B, R, G]))
    }

    pub fn colorless() -> WUBRG {
        WUBRG(BTreeSet::new())
    }
}

impl<S: AsRef<str>> From<S> for WUBRG {
    fn from(s: S) -> WUBRG {
        WUBRG(BTreeSet::from_iter(
            s.as_ref().chars().filter_map(|c| Pie::try_from(c).ok()),
        ))
    }
}

impl ToString for WUBRG {
    fn to_string(&self) -> String {
        match &self.0.iter().map(Clone::clone).collect_vec()[..] {
            &[] => "C".s(),
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
            &[_, _, _, _, _] => "WUBRG".s(),
            _ => "".s(),
        }
    }
}

impl ToSql for WUBRG {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
#[repr(u8)]
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
    #[serde(other)]
    Unsupported,
}

impl ToSql for CardLayout {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as u8 as i64)))
    }
}

impl From<i64> for CardLayout {
    fn from(value: i64) -> Self {
        const LO: i64 = CardLayout::Adventure as u8 as i64;
        const HI: i64 = CardLayout::Unsupported as u8 as i64;
        match value {
            LO..=HI => unsafe { std::mem::transmute(value as u8) },
            _ => Self::Unsupported,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    pub fn is_landscape(self) -> bool {
        match self {
            Self::Battle | Self::Fuse | Self::Aftermath | _ => false,
        }
    }
}

impl Display for FaceLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LeadershipSkills {
    #[serde(default, skip_serializing_if = "is_default")]
    pub brawl: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub commander: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub oathbreaker: bool,
}

impl From<&[u8]> for LeadershipSkills {
    fn from(value: &[u8]) -> Self {
        Self {
            brawl: value.get(0).unwrap_or(&0) > &0,
            commander: value.get(1).unwrap_or(&0) > &0,
            oathbreaker: value.get(2).unwrap_or(&0) > &0,
        }
    }
}

impl ToSql for LeadershipSkills {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Blob(vec![
            self.brawl as u8,
            self.commander as u8,
            self.oathbreaker as u8,
        ])))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Side {
    #[serde(rename = "a")]
    #[default]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(other)]
    Unsupported,
}

impl ToSql for Side {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as u8 as i64)))
    }
}

impl From<i64> for Side {
    fn from(value: i64) -> Self {
        match value {
            0 => Side::A,
            1 => Side::B,
            _ => Side::Unsupported,
        }
    }
}
