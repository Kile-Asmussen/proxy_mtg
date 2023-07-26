use std::{
    cmp::Reverse,
    ops::{Range, RangeInclusive},
    path::Path,
};

use crate::cards::{AtomicCards, Card, Layout, MetaData};

pub trait ProxyBuilder {
    type Output;

    fn build(&self) -> Self::Output;
    fn name(&mut self, name: &str) -> &mut Self;
    fn type_line(&mut self, type_line: &str) -> &mut Self;
    fn mana_cost(&mut self, mana_cost: &str) -> &mut Self;
    fn art_filename(&mut self, art_filename: &Path) -> &mut Self;
    fn art_credits(&mut self, artist: &str) -> &mut Self;
    fn border_color(&mut self, color: BorderColor) -> &mut Self;
    fn legendary_decor(&mut self, is_legedary: bool) -> &mut Self;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BorderColor {
    Mono(Color),
    Gradient(Color, Color),
    Gold,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

pub trait ProxyBuilderNormal {
    fn rules_text(&mut self, name: &str) -> &mut Self;
    fn flavor_text(&mut self, flavor_text: &str) -> &mut Self;
    fn watermark(&mut self, watermark_filename: &Path) -> &mut Self;
}

pub trait ProxyBuilderReversible: ProxyBuilder {
    type Back: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
    fn back(&mut self) -> &mut Self::Back;
}

pub trait ProxyBuilderSaga: ProxyBuilder {
    fn step_text(&mut self, step_text: &str, steps: &[i32]) -> &mut Self;
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

pub trait GeneralProxyBuilder {
    type Metadata;
    type Output;
    type Normal: ProxyBuilderNormal + ProxyBuilder<Output = Self::Output>;
    type Reversible: ProxyBuilderReversible + ProxyBuilder<Output = Self::Output>;
    type Saga: ProxyBuilderSaga + ProxyBuilder<Output = Self::Output>;

    fn new(metadata: MetaData) -> Self;

    fn build_card(card: &[Card]) -> Option<Self::Output>;
}

trait DeckBuilder {
    type Output;

    fn add_card(&mut self, card: &[Card]) -> &mut Self;
    fn unsupported_cards(&self) -> &[Card];

    fn result(&self) -> Self::Output;
}
