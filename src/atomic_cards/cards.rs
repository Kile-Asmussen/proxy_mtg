use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use super::{
    is_default,
    metadata::{ForeignData, Legalities, RelatedCards /*Ruling*/},
    types::{CardLayout, FaceLayout, LeadershipSkills, Side, Supertype, Type, WUBRG},
};

use std::{collections::BTreeSet, fmt::Display};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Card {
    #[serde(default, skip_serializing_if = "is_default", rename = "asciiName")]
    pub ascii_name: String,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "attractionLights"
    )]
    pub attraction_lights: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default", rename = "colorIdentity")]
    pub color_identity: BTreeSet<WUBRG>,
    #[serde(default, skip_serializing_if = "is_default", rename = "colorIndicator")]
    pub color_indicator: BTreeSet<WUBRG>,
    pub colors: BTreeSet<WUBRG>,
    // #[serde(default, skip_serializing_if = "is_default",  rename = "convertedManaCost")]
    // pub converted_mana_cost: f64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub defense: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "edhrecRank")]
    pub edhrec_rank: Option<f64>,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "edhrecSaltiness"
    )]
    pub edhrec_saltiness: Option<f64>,
    // #[serde(default, skip_serializing_if = "is_default",  rename = "faceConvertedManaCost")]
    // pub face_converted_mana_cost: f64,
    #[serde(default, skip_serializing_if = "is_default", rename = "faceManaValue")]
    pub face_mana_value: f64,
    #[serde(default, skip_serializing_if = "is_default", rename = "faceName")]
    pub face_name: String,
    // #[serde(default, skip_serializing_if = "is_default",  rename = "firstPrinting")]
    // pub first_printing: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "foreignData")]
    pub foreign_data: Vec<ForeignData>,
    // #[serde(default)]
    // pub hand: String,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "hasAlternativeDeckLimit"
    )]
    pub has_alternative_deck_limit: bool,
    // #[serde(default)]
    // pub identifiers: Identifiers,
    #[serde(default, skip_serializing_if = "is_default", rename = "isFunny")]
    pub is_funny: bool,
    // #[serde(default, skip_serializing_if = "is_default",  rename = "isReserved")]
    // pub is_reserved: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub keywords: IndexSet<String>,
    pub layout: CardLayout,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "leadershipSkills"
    )]
    pub leadership_skills: LeadershipSkills,
    #[serde(default, skip_serializing_if = "is_default")]
    pub legalities: Legalities,
    // #[serde(default, skip_serializing_if = "is_default")]
    // pub life: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub loyalty: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "manaCost")]
    pub mana_cost: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "manaValue")]
    pub mana_value: f64,
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub power: String,
    // #[serde(default)]
    // pub printings: Vec<String>,
    // #[serde(default, skip_serializing_if = "is_default",  rename = "purchaseUrls")]
    // pub purchase_urls: PurchaseUrls,
    #[serde(default, skip_serializing_if = "is_default", rename = "relatedCards")]
    pub related_cards: RelatedCards,
    // #[serde(default, skip_serializing_if = "is_default")]
    // pub rulings: Vec<Ruling>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub side: Side,
    // #[serde(default, skip_serializing_if = "is_default")]
    // pub subsets: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub subtypes: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub supertypes: Vec<Supertype>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub text: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub toughness: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "type")]
    pub type_line: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub types: Vec<Type>,
}

impl Card {
    pub fn is_land(&self) -> bool {
        self.is_type(Type::Land)
    }
    pub fn is_basic(&self) -> bool {
        self.is_supertype(Supertype::Basic) && self.is_land()
    }
    pub fn is_spell(&self) -> bool {
        !self.is_type(Type::Land) && self.layout != CardLayout::Token
    }
    pub fn is_permanent(&self) -> bool {
        !self.is_instant() && !self.is_sorcery()
    }
    pub fn is_instant(&self) -> bool {
        self.is_type(Type::Instant)
    }
    pub fn is_sorcery(&self) -> bool {
        self.is_type(Type::Sorcery)
    }
    pub fn is_type(&self, t: Type) -> bool {
        self.types.contains(&t)
    }
    pub fn is_supertype(&self, t: Supertype) -> bool {
        self.supertypes.contains(&t)
    }
    pub fn is_subtype(&self, t: &str) -> bool {
        self.subtypes.iter().any(|s| s == t)
    }

    pub fn face_layout(&self) -> FaceLayout {
        match &self.layout {
            CardLayout::Adventure => FaceLayout::Omenventure,
            CardLayout::Aftermath => FaceLayout::Aftermath,
            CardLayout::Case => FaceLayout::Case,
            CardLayout::Class => FaceLayout::Class,
            CardLayout::Flip => FaceLayout::Flip,
            CardLayout::Leveler => FaceLayout::Leveler,
            CardLayout::Meld => self.guess_face_layout(),
            CardLayout::ModalDfc => self.guess_face_layout(),
            CardLayout::Mutate => FaceLayout::Mutate,
            CardLayout::Normal => self.guess_face_layout(),
            CardLayout::Prototype => FaceLayout::Prototype,
            CardLayout::ReversibleCard => self.guess_face_layout(),
            CardLayout::Saga => FaceLayout::Saga,
            CardLayout::Split if self.text.contains("Fuse") => FaceLayout::Fuse,
            CardLayout::Split => FaceLayout::Split,
            CardLayout::Token => self.guess_face_layout(),
            CardLayout::Transform => self.guess_face_layout(),
            CardLayout::Unsupported => FaceLayout::Unsupported,
        }
    }

    pub fn guess_face_layout(&self) -> FaceLayout {
        if self.is_basic() {
            FaceLayout::Basic
        } else if self.is_type(Type::Creature) {
            FaceLayout::Creature
        } else if self.is_subtype("Saga") {
            FaceLayout::Saga
        } else if self.is_type(Type::Planeswalker) {
            FaceLayout::Planeswalker
        } else if self.is_type(Type::Battle) {
            FaceLayout::Battle
        } else {
            FaceLayout::Unadorned
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = &self.face_name;
        if name.is_empty() {
            name = &self.name;
        }
        write!(f, "{} {}", &name, &self.mana_cost)?;
        write!(f, "\n({}) {}", WUBRG::render(&self.colors), self.type_line)?;
        for line in self.text.lines() {
            write!(f, "\n{}", line)?;
        }
        if self.types.contains(&Type::Planeswalker) {
            write!(f, "\n[{}]", self.loyalty)?;
        }
        if self.types.contains(&Type::Battle) {
            write!(f, "\n<{}>", self.defense)?;
        }
        if self.types.contains(&Type::Creature) {
            write!(f, "\n{}/{}", self.power, self.toughness)?;
        }
        Ok(())
    }
}
