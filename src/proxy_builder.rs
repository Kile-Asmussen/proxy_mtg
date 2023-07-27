use std::{default, fmt::Display, path::Path};

use serde::{de::Visitor, Deserialize, Serialize};

pub trait ProxyBuilder {
    type Output;

    fn build(&mut self) -> Self::Output;
    fn name(&mut self, name: &str) -> &mut Self;
    fn type_line(&mut self, type_line: &str) -> &mut Self;
    fn color_indicator(&mut self, colors: &[String]) -> &mut Self;
    fn color_identity(&mut self, colors: &[String]) -> &mut Self;
    fn mana_cost(&mut self, mana_cost: &str) -> &mut Self;
    fn art_filename(&mut self, art_filename: &Path) -> &mut Self;
    fn art_credits(&mut self, artist: &str) -> &mut Self;
    fn set_legendary(&mut self, is_legendary: bool) -> &mut Self;
}

pub trait DeckBuilder {
    type Input;
    type Output;
    type Result;

    fn add_card(&mut self, card: Self::Input) -> &mut Self;

    fn build(&self, out: &mut Self::Output) -> Self::Result;
}

pub trait ProxyBuilderNormal {
    fn rules_text(&mut self, rules_text: &str) -> &mut Self;
    fn flavor_text(&mut self, flavor_text: &str) -> &mut Self;
    fn corner_bubble(&mut self, corner_bubble: &str) -> &mut Self;
}

pub trait ProxyBuilderReversible: ProxyBuilder {
    type Back: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
    fn back(&mut self) -> &mut Self::Back;
}

pub trait ProxyBuilderSaga: ProxyBuilder {
    fn step_text(&mut self, steps: &[u32], rules_text: &str) -> &mut Self;
    fn include_reminder(&mut self, remind: bool) -> &mut Self;
    fn flavor_text(&mut self, text: &str) -> &mut Self;
}

// pub trait ProxyBuilderAdventure: ProxyBuilder {
//     type Adventure: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
//     fn adventure(&mut self) -> &mut Self::Adventure;
// }

// pub trait ProxyBuilderSplit: ProxyBuilder {
//     type RightSide: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
//     fn right_side(&mut self) -> &mut Self::RightSide;
// }

// pub trait ProxyBuilderFlip: ProxyBuilder {
//     type FlipSide: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
//     fn flip_side(&mut self) -> &mut Self::FlipSide;
// }

pub trait BasicLandProxyBuilder {
    type Output;

    fn art(&mut self, land: BasicLand, art_filename: &Path, artist: &str) -> &mut Self;

    fn build(&mut self, land: BasicLand) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicLand {
    Base(CoreLand),
    Snow(CoreLand),
    Wastes,
}

impl Default for BasicLand {
    fn default() -> Self {
        Self::Base(Default::default())
    }
}

impl BasicLand {
    pub const ALL: &[BasicLand] = &[
        Self::Base(CoreLand::Plains),
        Self::Base(CoreLand::Island),
        Self::Base(CoreLand::Swamp),
        Self::Base(CoreLand::Mountain),
        Self::Base(CoreLand::Forest),
        Self::Snow(CoreLand::Plains),
        Self::Snow(CoreLand::Island),
        Self::Snow(CoreLand::Swamp),
        Self::Snow(CoreLand::Mountain),
        Self::Snow(CoreLand::Forest),
        Self::Wastes,
    ];
}

impl Display for BasicLand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicLand::Base(s) => <CoreLand as Display>::fmt(s, f),
            BasicLand::Snow(s) => f.write_fmt(format_args!("Snow-Covered {}", s)),
            BasicLand::Wastes => f.write_str("Wastes"),
        }
    }
}

impl Serialize for BasicLand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for BasicLand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BasicLandVisitor)
    }
}

struct BasicLandVisitor;

impl<'a> Visitor<'a> for BasicLandVisitor {
    type Value = BasicLand;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Name of a basic land")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        BasicLand::ALL
            .iter()
            .find(|l| v == &format!("{}", l))
            .map(|s| *s)
            .ok_or(E::custom(format!("{} is not the name of a basic land", v)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CoreLand {
    #[default]
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
}

impl Display for CoreLand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
