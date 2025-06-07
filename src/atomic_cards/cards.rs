use serde::{Deserialize, Serialize};

use super::{
    metadata::{ForeignData, Identifiers, Legalities, PurchaseUrls, RelatedCards, Ruling},
    types::{CardLayout, FaceLayout, LeadershipSkills, Side, Supertype, Type, WUBRG},
};

use std::collections::BTreeSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Card {
    #[serde(default, rename = "asciiName")]
    pub ascii_name: String,
    #[serde(default, rename = "attractionLights")]
    pub attraction_lights: Vec<String>,
    #[serde(default, rename = "colorIdentity")]
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
    #[serde(default)]
    pub identifiers: Identifiers,
    #[serde(default, rename = "isFunny")]
    pub is_funny: bool,
    #[serde(default, rename = "isReserved")]
    pub is_reserved: bool,
    #[serde(default)]
    pub keywords: BTreeSet<String>,
    pub layout: CardLayout,
    #[serde(default, rename = "leadershipSkills")]
    pub leadership_skills: LeadershipSkills,
    #[serde(default)]
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
    #[serde(default, rename = "purchaseUrls")]
    pub purchase_urls: PurchaseUrls,
    #[serde(default, rename = "relatedCards")]
    pub related_cards: RelatedCards,
    #[serde(default)]
    pub rulings: Vec<Ruling>,
    #[serde(default)]
    pub side: Side,
    #[serde(default)]
    pub subsets: Vec<String>,
    #[serde(default)]
    pub subtypes: Vec<String>,
    #[serde(default)]
    pub supertypes: Vec<Supertype>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub toughness: String,
    #[serde(rename = "type")]
    pub type_line: String,
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
            CardLayout::Saga if self.is_type(Type::Creature) => FaceLayout::SagaCreature,
            CardLayout::Saga => FaceLayout::Saga,
            CardLayout::Split if self.text.contains("Fuse") => FaceLayout::Fuse,
            CardLayout::Split => FaceLayout::Split,
            CardLayout::Token => self.guess_face_layout(),
            CardLayout::Transform => self.guess_face_layout(),
            CardLayout::Other(_) => FaceLayout::Unsupported,
        }
    }

    pub fn guess_face_layout(&self) -> FaceLayout {
        if self.is_basic() {
            FaceLayout::Basic
        } else if self.is_type(Type::Creature) && self.is_subtype("Saga") {
            FaceLayout::SagaCreature
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

    pub fn translation() {}
}
