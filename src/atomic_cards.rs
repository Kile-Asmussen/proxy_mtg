use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::BufReader,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtomicCards {
    pub meta: MetaData,
    pub data: HashMap<String, Cardoid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Cardoid(pub Vec<Card>);

impl AtomicCards {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let atomic_cards_file = BufReader::new(File::open("AtomicCards.json")?);

        let mut atomic_cards_deserializer =
            serde_json::Deserializer::from_reader(atomic_cards_file);

        let atomic_cards = AtomicCards::deserialize(&mut atomic_cards_deserializer)?;

        return Ok(atomic_cards);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub date: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    #[serde(default, rename = "asciiName")]
    pub ascii_name: String,
    #[serde(default, rename = "attractionLights")]
    pub attraction_lights: Vec<String>,
    #[serde(rename = "colorIdentity")]
    pub color_identity: HashSet<String>,
    #[serde(default, rename = "colorIndicator")]
    pub color_indicator: HashSet<String>,
    pub colors: HashSet<String>,
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
    pub types: Vec<CardType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,

    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardType {
    Kindred,

    Instant,
    Sorcery,

    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Land,
    Battle,

    #[serde(untagged)]
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RelatedCards {
    #[serde(default, rename = "reverseRelated")]
    pub reverse_related: Vec<String>,
    #[serde(default)]
    pub spellbook: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
