use serde::{Deserialize, Serialize};

use super::{metadata::*, types::*};

use std::{collections::BTreeSet, fmt::Display};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Cardoid(Vec<Card>);

impl Cardoid {
    pub fn is_empty() {}

    pub fn iter(&self) -> <&Vec<Card> as IntoIterator>::IntoIter {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> <&mut Vec<Card> as IntoIterator>::IntoIter {
        self.0.iter_mut()
    }

    pub fn front(&self) -> &Card {
        &self.0[0]
    }

    pub fn back(&self) -> Option<&Card> {
        self.0.get(1)
    }
}

impl Display for Cardoid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let front = self.front();
        if let Some(back) = self.back() {
            f.write_fmt(format_args!("> {}", front.name));

            f.write_str("\n> FRONT:\n")?;
            Display::fmt(&(&front), f)?;
            f.write_str("\n> BACK:\n")?;
            Display::fmt(&back, f)?;
        } else {
            Display::fmt(&front, f)?;
        }
        return Ok(());
    }
}

impl IntoIterator for Cardoid {
    type Item = Card;

    type IntoIter = <Vec<Card> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Cardoid {
    type Item = &'a Card;

    type IntoIter = <&'a Vec<Card> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Cardoid {
    type Item = &'a mut Card;

    type IntoIter = <&'a mut Vec<Card> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    #[serde(default, rename = "asciiName")]
    pub ascii_name: String,
    #[serde(default, rename = "attractionLights")]
    pub attraction_lights: Vec<String>,
    #[serde(rename = "colorIdentity")]
    pub color_identity: BTreeSet<WUBRG>,
    #[serde(default, rename = "colorIndicator")]
    pub color_indicator: BTreeSet<WUBRG>,
    pub colors: BTreeSet<WUBRG>,
    #[serde(default, rename = "convertedManaCost")]
    pub converted_mana_cost: f64,
    #[serde(default)]
    pub defense: String,
    #[serde(default, rename = "edhrecRank")]
    pub edhrec_rank: Option<f64>,
    #[serde(default, rename = "edhrecSaltiness")]
    pub edhrec_saltiness: Option<f64>,
    #[serde(default, rename = "faceConvertedManaCost")]
    pub face_converted_mana_cost: f64,
    #[serde(default, rename = "faceManaValue")]
    pub face_mana_value: f64,
    #[serde(default, rename = "faceName")]
    pub face_name: String,
    #[serde(default, rename = "firstPrinting")]
    pub first_printing: String,
    #[serde(default, rename = "foreignData")]
    pub foreign_data: Vec<ForeignData>,
    #[serde(default)]
    pub hand: String,
    #[serde(default, rename = "hasAlternativeDeckLimit")]
    pub has_alternative_deck_limit: bool,
    pub identifiers: Identifiers,
    #[serde(default, rename = "isFunny")]
    pub is_funny: bool,
    #[serde(default, rename = "isReserved")]
    pub is_reserved: bool,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub layout: Layout,
    #[serde(default, rename = "leadershipSkills")]
    pub leadership_skills: LeadershipSkills,
    pub legalities: Legalities,
    #[serde(default)]
    pub life: String,
    #[serde(default)]
    pub loyalty: String,
    #[serde(default, rename = "manaCost")]
    pub mana_cost: String,
    #[serde(default, rename = "manaValue")]
    pub mana_value: f64,
    pub name: String,
    #[serde(default)]
    pub power: String,
    #[serde(default)]
    pub printings: Vec<String>,
    #[serde(rename = "purchaseUrls")]
    pub purchase_urls: PurchaseUrls,
    #[serde(default, rename = "relatedCards")]
    pub related_cards: RelatedCards,
    #[serde(default)]
    pub rulings: Vec<Ruling>,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub subsets: Vec<String>,
    pub subtypes: Vec<String>,
    pub supertypes: Vec<Supertype>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub toughness: String,
    #[serde(rename = "type")]
    pub type_line: String,
    pub types: Vec<Type>,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = &self.face_name;
        if name.is_empty() {
            name = &self.name;
        }
        f.write_fmt(format_args!("> {} {}", name, self.mana_cost))?;
        f.write_fmt(format_args!(
            "\n> ({}) {}",
            WUBRG::wubrg(&self.colors),
            self.type_line
        ))?;
        for line in self.text.lines() {
            f.write_fmt(format_args!("\n> {}", line))?;
        }
        if self.types.contains(&Type::Planeswalker) {
            f.write_fmt(format_args!("\n> [{}]", self.loyalty))?;
        }
        if self.types.contains(&Type::Creature) {
            f.write_fmt(format_args!("\n> {}/{}", self.power, self.toughness))?;
        }
        if self.types.contains(&Type::Battle) {
            f.write_fmt(format_args!("\n> {{{}}}", self.defense))?;
        }
        Ok(())
    }
}
