use crate::utils::escape_html_text;

use super::is_default;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MetaData {
    pub date: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct RelatedCards {
    #[serde(default, skip_serializing_if = "is_default", rename = "reverseRelated")]
    pub reverse_related: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub spellbook: Vec<String>,
}

// #[derive(Deserialize, Debug, Clone, Default)]
// pub struct PurchaseUrls {
//     #[serde(default, rename = "cardKingdom")]
//     pub card_kingdom: String,
//     #[serde(default, rename = "cardKingdomEtched")]
//     pub card_kingdom_etched: String,
//     #[serde(default, rename = "cardKingdomFoil")]
//     pub card_kingdom_foil: String,
//     #[serde(default)]
//     pub cardmarket: String,
//     #[serde(default)]
//     pub tcgplayer: String,
//     #[serde(default, rename = "tcgplayerEtched")]
//     pub tcgplayer_etched: String,
// }

// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// pub struct Ruling {
//     pub date: String,
//     pub text: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForeignData {
    #[serde(default, rename = "faceName")]
    pub face_name: String,
    #[serde(default, rename = "flavorText")]
    pub flavor_text: String,
    #[serde(default)]
    pub language: String,
    // #[serde(default, rename = "multiverseId")]
    // pub multiverse_id: f64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub text: String,
    #[serde(default, rename = "type")]
    pub type_line: String,
}

impl ForeignData {
    pub fn get_name(&self) -> String {
        escape_html_text(if self.name.is_empty() {
            &self.face_name
        } else {
            &self.name
        })
    }

    pub fn get_text(&self) -> String {
        escape_html_text(&self.text)
    }

    pub fn get_flavor(&self) -> String {
        escape_html_text(&self.flavor_text)
    }

    pub fn get_type_line(&self) -> String {
        escape_html_text(&self.type_line)
    }
}

// #[derive(Deserialize, Debug, Clone, Default)]
// pub struct Identifiers {
//     #[serde(default, rename = "cardKingdomEtchedId")]
//     pub card_kingdom_etched_id: String,
//     #[serde(default, rename = "cardKingdomFoilId")]
//     pub card_kingdom_foil_id: String,
//     #[serde(default, rename = "cardKingdomId")]
//     pub card_kingdom_id: String,
//     #[serde(default, rename = "cardsphereId")]
//     pub cardsphere_id: String,
//     #[serde(default, rename = "mcmId")]
//     pub mcm_id: String,
//     #[serde(default, rename = "mcmMetaId")]
//     pub mcm_meta_id: String,
//     #[serde(default, rename = "mtgArenaId")]
//     pub mtg_arena_id: String,
//     #[serde(default, rename = "mtgjsonFoilVersionId")]
//     pub mtgjson_foil_version_id: String,
//     #[serde(default, rename = "mtgjsonNonFoilVersionId")]
//     pub mtgjson_non_foil_version_id: String,
//     #[serde(default, rename = "mtgjsonV4Id")]
//     pub mtgjson_v4_id: String,
//     #[serde(default, rename = "mtgoFoilId")]
//     pub mtgo_foil_id: String,
//     #[serde(default, rename = "mtgoId")]
//     pub mtgo_id: String,
//     #[serde(default, rename = "multiverseId")]
//     pub multiverse_id: String,
//     #[serde(default, rename = "scryfallId")]
//     pub scryfall_id: String,
//     #[serde(default, rename = "scryfallOracleId")]
//     pub scryfall_oracle_id: String,
//     #[serde(default, rename = "scryfallIllustrationId")]
//     pub scryfall_illustration_id: String,
//     #[serde(default, rename = "tcgplayerProductId")]
//     pub tcgplayer_product_id: String,
//     #[serde(default, rename = "tcgplayerEtchedProductId")]
//     pub tcgplayer_etched_product_id: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Legalities {
    #[serde(default, skip_serializing_if = "is_default")]
    pub alchemy: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub brawl: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub commander: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub duel: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub explorer: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub future: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub gladiator: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub historic: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub historicbrawl: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub legacy: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub modern: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub oldschool: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pauper: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub penny: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pioneer: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub predh: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub premodern: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub standard: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub vintage: String,
}
