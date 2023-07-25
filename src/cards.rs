use std::{collections::HashMap, error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtomicCards {
    pub meta: MetaData,
    pub data: HashMap<String, Vec<Card>>,
}

impl AtomicCards {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut atomic_cards_file = File::open("./AtomicCards.json")?;
        let mut atomic_cards_string = String::new();
        atomic_cards_file.read_to_string(&mut atomic_cards_string)?;
        Ok(serde_json::from_str(&atomic_cards_string)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub date: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    #[serde(rename = "asciiName")]
    #[serde(default)]
    pub ascii_name: String,
    #[serde(rename = "attractionLights")]
    #[serde(default)]
    pub attraction_lights: Vec<String>,
    #[serde(rename = "colorIdentity")]
    pub color_identity: Vec<String>,
    #[serde(rename = "colorIndicator")]
    #[serde(default)]
    pub color_indicator: Vec<String>,
    pub colors: Vec<String>,
    #[serde(rename = "convertedManaCost")]
    pub converted_mana_cost: f64,
    #[serde(default)]
    pub defense: String,
    #[serde(rename = "edhrecRank")]
    #[serde(default)]
    pub edhrec_rank: Option<f64>,
    #[serde(rename = "edhrecSaltiness")]
    #[serde(default)]
    pub edhrec_saltiness: Option<f64>,
    #[serde(rename = "faceConvertedManaCost")]
    #[serde(default)]
    pub face_converted_mana_cost: f64,
    #[serde(rename = "faceManaValue")]
    #[serde(default)]
    pub face_mana_value: f64,
    #[serde(rename = "faceName")]
    #[serde(default)]
    pub face_name: String,
    #[serde(rename = "firstPrinting")]
    #[serde(default)]
    pub first_printing: String,
    #[serde(rename = "foreignData")]
    #[serde(default)]
    pub foreign_data: Vec<ForeignData>,
    #[serde(default)]
    pub hand: String,
    #[serde(rename = "hasAlternativeDeckLimit")]
    #[serde(default)]
    pub has_alternative_deck_limit: bool,
    pub identifiers: Identifiers,
    #[serde(rename = "isFunny")]
    #[serde(default)]
    pub is_funny: bool,
    #[serde(rename = "isReserved")]
    #[serde(default)]
    pub is_reserved: bool,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub layout: Layout,
    #[serde(rename = "leadershipSkills")]
    #[serde(default)]
    pub leadership_skills: LeadershipSkills,
    pub legalities: Legalities,
    #[serde(default)]
    pub life: String,
    #[serde(default)]
    pub loyalty: String,
    #[serde(rename = "manaCost")]
    #[serde(default)]
    pub mana_cost: String,
    #[serde(rename = "manaValue")]
    pub mana_value: f64,
    pub name: String,
    #[serde(default)]
    pub power: String,
    #[serde(default)]
    pub printings: Vec<String>,
    #[serde(rename = "purchaseUrls")]
    pub purchase_urls: PurchaseUrls,
    #[serde(rename = "relatedCards")]
    #[serde(default)]
    pub related_cards: RelatedCards,
    #[serde(default)]
    pub rulings: Vec<Ruling>,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub subsets: Vec<String>,
    pub subtypes: Vec<String>,
    pub supertypes: Vec<String>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub toughness: String,
    #[serde(rename = "type")]
    pub type_line: String,
    pub types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RelatedCards {
    #[serde(rename = "reverseRelated")]
    #[serde(default)]
    pub reverse_related: Vec<String>,
    #[serde(default)]
    pub spellbook: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PurchaseUrls {
    #[serde(rename = "cardKingdom")]
    #[serde(default)]
    pub card_kingdom: String,
    #[serde(rename = "cardKingdomEtched")]
    #[serde(default)]
    pub card_kingdom_etched: String,
    #[serde(rename = "cardKingdomFoil")]
    #[serde(default)]
    pub card_kingdom_foil: String,
    #[serde(default)]
    pub cardmarket: String,
    #[serde(default)]
    pub tcgplayer: String,
    #[serde(rename = "tcgplayerEtched")]
    #[serde(default)]
    pub tcgplayer_etched: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LeadershipSkills {
    pub brawl: bool,
    pub commander: bool,
    pub oathbreaker: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ruling {
    pub date: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForeignData {
    #[serde(rename = "faceName")]
    #[serde(default)]
    pub face_name: String,
    #[serde(rename = "flavorText")]
    #[serde(default)]
    pub flavor_text: String,
    pub language: String,
    #[serde(rename = "multiverseId")]
    #[serde(default)]
    pub multiverse_id: f64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub text: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_line: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identifiers {
    #[serde(rename = "cardKingdomEtchedId")]
    #[serde(default)]
    pub card_kingdom_etched_id: String,
    #[serde(rename = "cardKingdomFoilId")]
    #[serde(default)]
    pub card_kingdom_foil_id: String,
    #[serde(rename = "cardKingdomId")]
    #[serde(default)]
    pub card_kingdom_id: String,
    #[serde(rename = "cardsphereId")]
    #[serde(default)]
    pub cardsphere_id: String,
    #[serde(rename = "mcmId")]
    #[serde(default)]
    pub mcm_id: String,
    #[serde(rename = "mcmMetaId")]
    #[serde(default)]
    pub mcm_meta_id: String,
    #[serde(rename = "mtgArenaId")]
    #[serde(default)]
    pub mtg_arena_id: String,
    #[serde(rename = "mtgjsonFoilVersionId")]
    #[serde(default)]
    pub mtgjson_foil_version_id: String,
    #[serde(rename = "mtgjsonNonFoilVersionId")]
    #[serde(default)]
    pub mtgjson_non_foil_version_id: String,
    #[serde(rename = "mtgjsonV4Id")]
    #[serde(default)]
    pub mtgjson_v4_id: String,
    #[serde(rename = "mtgoFoilId")]
    #[serde(default)]
    pub mtgo_foil_id: String,
    #[serde(rename = "mtgoId")]
    #[serde(default)]
    pub mtgo_id: String,
    #[serde(rename = "multiverseId")]
    #[serde(default)]
    pub multiverse_id: String,
    #[serde(rename = "scryfallId")]
    #[serde(default)]
    pub scryfall_id: String,
    #[serde(rename = "scryfallOracleId")]
    #[serde(default)]
    pub scryfall_oracle_id: String,
    #[serde(rename = "scryfallIllustrationId")]
    #[serde(default)]
    pub scryfall_illustration_id: String,
    #[serde(rename = "tcgplayerProductId")]
    #[serde(default)]
    pub tcgplayer_product_id: String,
    #[serde(rename = "tcgplayerEtchedProductId")]
    #[serde(default)]
    pub tcgplayer_etched_product_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Legalities {
    #[serde(default)]
    alchemy: String,
    #[serde(default)]
    brawl: String,
    #[serde(default)]
    commander: String,
    #[serde(default)]
    duel: String,
    #[serde(default)]
    explorer: String,
    #[serde(default)]
    future: String,
    #[serde(default)]
    gladiator: String,
    #[serde(default)]
    historic: String,
    #[serde(default)]
    historicbrawl: String,
    #[serde(default)]
    legacy: String,
    #[serde(default)]
    modern: String,
    #[serde(default)]
    oldschool: String,
    #[serde(default)]
    pauper: String,
    #[serde(default)]
    penny: String,
    #[serde(default)]
    pioneer: String,
    #[serde(default)]
    predh: String,
    #[serde(default)]
    premodern: String,
    #[serde(default)]
    standard: String,
    #[serde(default)]
    vintage: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layout {
    #[serde(rename = "adventure")]
    Adventure,
    #[serde(rename = "aftermath")]
    Aftermath,
    #[serde(rename = "augment")]
    Augment,
    #[serde(rename = "class")]
    Class,
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
    #[serde(rename = "transform")]
    Transform,
    #[serde(rename = "vanguard")]
    Vanguard,
}
