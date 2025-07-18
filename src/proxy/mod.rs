pub mod decklists;
pub mod deserializers;

use std::fmt::Display;

use deserializers::OneOrMany;
use indexmap::IndexSet;
use itertools::{EitherOrBoth, Itertools};

use serde::{Deserialize, Serialize};

use crate::{
    atomic_cards::{
        cardoids::Cardoid,
        types::{CardLayout, WUBRG},
    },
    scryfall::api::ScryfallCard,
    utils::ToS,
};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(default, deserialize_with = "OneOrMany::<Art>::one_or_many")]
    pub arts: Vec<Art>,
    #[serde(default)]
    pub copies: usize,
    #[serde(rename = "reminderText", default = "Proxy::reminder_text_default")]
    pub reminder_text: bool,
    #[serde(default = "Proxy::repeats_default")]
    pub repeats: usize,
    #[serde(default)]
    pub sideboard: bool,
    #[serde(default)]
    pub tags: IndexSet<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default, deserialize_with = "OneOrMany::<Customization>::one_or_many")]
    pub customize: Vec<Customization>,
    #[serde(default, deserialize_with = "Cardoid::one_or_many")]
    pub cardoid: Cardoid,
}

impl Proxy {
    pub fn layout(&self) -> &CardLayout {
        (&self.cardoid).layout()
    }

    pub fn in_deck(&self) -> bool {
        !(self.sideboard || self.layout() == &CardLayout::Token)
    }

    fn repeats_default() -> usize {
        1
    }

    fn reminder_text_default() -> bool {
        true
    }

    pub fn category(&self) -> Option<String> {
        self.tags.get_index(0).map(Clone::clone)
    }

    pub fn uncategorized(&self) -> String {
        for t in &self.cardoid.face().types {
            if let Some(s) = t.plural() {
                return s;
            }
        }
        return "Uncategorized".s();
    }

    pub fn set_scryfall_arts<F>(&mut self, mut scryfall: F) -> anyhow::Result<()>
    where
        F: FnMut() -> anyhow::Result<ScryfallCard>,
    {
        if self.arts.len() < self.cardoid.printed_cards() || self.arts.iter().any(|a| a.scryfall) {
            self.arts = self
                .arts
                .iter_mut()
                .zip_longest(scryfall()?.arts())
                .map(|x| match x {
                    EitherOrBoth::Both(a, b) => a.copy_from(&b).clone(),
                    EitherOrBoth::Left(a) => a.clone(),
                    EitherOrBoth::Right(a) => a,
                })
                .collect_vec();
        }
        Ok(())
    }

    pub fn add_scryfall_arts<F>(&mut self, mut scryfall: F) -> anyhow::Result<()>
    where
        F: FnMut() -> anyhow::Result<ScryfallCard>,
    {
        if self.arts.iter().any(|a| a.scryfall) {
            self.arts
                .iter_mut()
                .zip(scryfall()?.arts())
                .for_each(|(a, b)| {
                    a.copy_from(&b);
                });
        }
        Ok(())
    }
}

impl Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cardoid.fmt(f)?;

        write!(f, "\n###")?;

        if !self.tags.is_empty() {
            write!(
                f,
                "\ntags: {}",
                &self.tags.iter().map(Clone::clone).collect_vec().join(", ")
            )?;
        }
        if self.repeats > 1 {
            write!(f, "\ncopies: {}", self.repeats)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Art {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub credit: String,
    #[serde(default)]
    pub full: bool,
    #[serde(default)]
    pub scryfall: bool,
}

impl Art {
    pub fn copy_from(&mut self, other: &Art) -> &mut Self {
        if self.url.is_empty() {
            self.url = other.url.to_string();
            self.full |= other.full;
            self.credit = other.credit.to_string();
            self.scryfall = other.scryfall;
        }
        self
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Customization {
    #[serde(default, rename = "flavorText")]
    pub flavor_text: Option<String>,
    #[serde(default)]
    pub colored: Option<WUBRG>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default, rename = "type")]
    pub type_line: Option<String>,
    #[serde(
        default,
        rename = "textStyle",
        deserialize_with = "OneOrMany::<TextStyle>::none_or_one_or_many"
    )]
    pub text_style: Option<Vec<TextStyle>>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum TextStyle {
    #[serde(rename = "no-line-spacing")]
    NoLineSpacing,
    #[serde(rename = "indented-paragraphs")]
    IndentedParagraphs,
    #[serde(rename = "centered-text")]
    CenteredText,
    #[serde(rename = "text-size-120")]
    TextSize120,
    #[serde(rename = "text-size-115")]
    TextSize115,
    #[serde(rename = "text-size-110")]
    TextSize110,
    #[serde(rename = "text-size-105")]
    TextSize105,
    #[serde(rename = "text-size-95")]
    TextSize95,
    #[serde(rename = "text-size-90")]
    TextSize90,
    #[serde(rename = "text-size-85")]
    TextSize85,
    #[serde(rename = "text-size-80")]
    TextSize80,
}

impl Display for TextStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_value(self).unwrap().as_str().unwrap())
    }
}
