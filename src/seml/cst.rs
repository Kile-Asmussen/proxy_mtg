use bigdecimal::{num_bigint::BigInt, BigDecimal};

use crate::seml::parsing::ParserState;

#[derive(Clone)]
pub enum SemlToken {
    Atom(SemlAtom),
    Compound(SemlCompound),
}

#[derive(Clone)]
pub enum SemlAtom {
    String(SemlString),
    Int(SemlInt),
    Decimal(SemlDecimal),
    True(ParserState),
    False(ParserState),
    Nil(ParserState),
}

#[derive(Clone)]
pub struct SemlString {
    pub format: QuoteFormat,
    pub value: String,
}

#[derive(Clone)]
pub struct SemlInt {
    pub format: IntFormat,
    pub value: BigInt,
}

#[derive(Clone)]
pub struct SemlDecimal {
    pub state: ParserState,
    pub format: RealFormat,
    pub value: BigDecimal,
}

#[derive(Clone)]
pub enum SemlCompound {
    Map(SemlMap),
    List(SemlList),
}

#[derive(Clone)]
pub struct SemlList {
    pub state: ParserState,
    pub format: ObjFormat,
    pub value: Vec<SemlToken>,
}

#[derive(Clone)]
pub struct SemlMap {
    pub state: ParserState,
    pub format: ObjFormat,
    pub value: Vec<(String, SemlToken)>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IntFormat {
    Dec,
    Hex,
    Bin,
    Oct,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RealFormat {
    Dec,
    Exp,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QuoteFormat {
    Single,
    Double,
    Preformatted,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjFormat {
    Inline,
    Multiline,
}
