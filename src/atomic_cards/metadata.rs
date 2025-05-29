use std::default;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub date: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RelatedCards {
    #[serde(default, rename = "reverseRelated")]
    pub reverse_related: Vec<String>,
    #[serde(default)]
    pub spellbook: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PurchaseUrls {
    #[serde(default, rename = "cardKingdom")]
    pub card_kingdom: String,
    #[serde(default, rename = "cardKingdomEtched")]
    pub card_kingdom_etched: String,
    #[serde(default, rename = "cardKingdomFoil")]
    pub card_kingdom_foil: String,
    #[serde(default)]
    pub cardmarket: String,
    #[serde(default)]
    pub tcgplayer: String,
    #[serde(default, rename = "tcgplayerEtched")]
    pub tcgplayer_etched: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ruling {
    pub date: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForeignData {
    #[serde(default, rename = "faceName")]
    pub face_name: String,
    #[serde(default, rename = "flavorText")]
    pub flavor_text: String,
    pub language: String,
    #[serde(default, rename = "multiverseId")]
    pub multiverse_id: f64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub text: String,
    #[serde(default, rename = "type")]
    pub type_line: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Identifiers {
    #[serde(default, rename = "cardKingdomEtchedId")]
    pub card_kingdom_etched_id: String,
    #[serde(default, rename = "cardKingdomFoilId")]
    pub card_kingdom_foil_id: String,
    #[serde(default, rename = "cardKingdomId")]
    pub card_kingdom_id: String,
    #[serde(default, rename = "cardsphereId")]
    pub cardsphere_id: String,
    #[serde(default, rename = "mcmId")]
    pub mcm_id: String,
    #[serde(default, rename = "mcmMetaId")]
    pub mcm_meta_id: String,
    #[serde(default, rename = "mtgArenaId")]
    pub mtg_arena_id: String,
    #[serde(default, rename = "mtgjsonFoilVersionId")]
    pub mtgjson_foil_version_id: String,
    #[serde(default, rename = "mtgjsonNonFoilVersionId")]
    pub mtgjson_non_foil_version_id: String,
    #[serde(default, rename = "mtgjsonV4Id")]
    pub mtgjson_v4_id: String,
    #[serde(default, rename = "mtgoFoilId")]
    pub mtgo_foil_id: String,
    #[serde(default, rename = "mtgoId")]
    pub mtgo_id: String,
    #[serde(default, rename = "multiverseId")]
    pub multiverse_id: String,
    #[serde(default, rename = "scryfallId")]
    pub scryfall_id: String,
    #[serde(default, rename = "scryfallOracleId")]
    pub scryfall_oracle_id: String,
    #[serde(default, rename = "scryfallIllustrationId")]
    pub scryfall_illustration_id: String,
    #[serde(default, rename = "tcgplayerProductId")]
    pub tcgplayer_product_id: String,
    #[serde(default, rename = "tcgplayerEtchedProductId")]
    pub tcgplayer_etched_product_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Legalities {
    #[serde(default)]
    pub alchemy: String,
    #[serde(default)]
    pub brawl: String,
    #[serde(default)]
    pub commander: String,
    #[serde(default)]
    pub duel: String,
    #[serde(default)]
    pub explorer: String,
    #[serde(default)]
    pub future: String,
    #[serde(default)]
    pub gladiator: String,
    #[serde(default)]
    pub historic: String,
    #[serde(default)]
    pub historicbrawl: String,
    #[serde(default)]
    pub legacy: String,
    #[serde(default)]
    pub modern: String,
    #[serde(default)]
    pub oldschool: String,
    #[serde(default)]
    pub pauper: String,
    #[serde(default)]
    pub penny: String,
    #[serde(default)]
    pub pioneer: String,
    #[serde(default)]
    pub predh: String,
    #[serde(default)]
    pub premodern: String,
    #[serde(default)]
    pub standard: String,
    #[serde(default)]
    pub vintage: String,
}
