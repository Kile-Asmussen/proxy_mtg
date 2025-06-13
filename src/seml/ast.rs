use bigdecimal::{num_bigint::BigInt, BigDecimal};

use crate::{
    seml::cst::{
        SemlAtom, SemlCompound, SemlDecimal, SemlInt, SemlList, SemlMap, SemlString, SemlToken,
    },
    utils::iter::IterExt,
};

pub enum SemlValue {
    Nil,
    True,
    False,
    Int(BigInt),
    Decimal(BigDecimal),
    String(String),
    List(Vec<SemlValue>),
    Map(Vec<(String, SemlValue)>),
}

impl From<SemlToken> for SemlValue {
    fn from(value: SemlToken) -> Self {
        match value {
            SemlToken::Atom(seml_atom) => seml_atom.into(),
            SemlToken::Compound(seml_compound) => seml_compound.into(),
        }
    }
}

impl From<SemlAtom> for SemlValue {
    fn from(value: SemlAtom) -> Self {
        match value {
            SemlAtom::String(seml_string) => seml_string.into(),
            SemlAtom::Int(seml_int) => seml_int.into(),
            SemlAtom::Decimal(seml_decimal) => seml_decimal.into(),
            SemlAtom::True(_) => Self::True,
            SemlAtom::False(_) => Self::False,
            SemlAtom::Nil(_) => Self::Nil,
        }
    }
}

impl From<SemlInt> for SemlValue {
    fn from(value: SemlInt) -> Self {
        Self::Int(value.value)
    }
}

impl From<SemlDecimal> for SemlValue {
    fn from(value: SemlDecimal) -> Self {
        Self::Decimal(value.value)
    }
}

impl From<SemlString> for SemlValue {
    fn from(value: SemlString) -> Self {
        Self::String(value.value)
    }
}

impl From<SemlCompound> for SemlValue {
    fn from(value: SemlCompound) -> Self {
        match value {
            SemlCompound::Map(seml_map) => seml_map.into(),
            SemlCompound::List(seml_list) => seml_list.into(),
        }
    }
}

impl From<SemlMap> for SemlValue {
    fn from(value: SemlMap) -> Self {
        Self::Map(
            value
                .value
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collvect(),
        )
    }
}

impl From<SemlList> for SemlValue {
    fn from(value: SemlList) -> Self {
        Self::List(value.value.into_iter().map(|v| v.into()).collvect())
    }
}
