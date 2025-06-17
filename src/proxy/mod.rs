pub mod decklists;
pub mod deserializers;

use std::fmt::Display;

use deserializers::OneOrMany;
use indexmap::IndexSet;
use itertools::{EitherOrBoth, Itertools};

use serde::{Deserialize, Serialize};

use crate::{
    atomic_cards::{cardoids::Cardoid, metadata::ForeignData, types::CardLayout},
    scryfall::api::ScryfallCard,
    utils::{iter::IterExt, ToS},
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
    #[serde(default, deserialize_with = "OneOrMany::<ForeignData>::one_or_many")]
    pub customize: Vec<ForeignData>,
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
                .collvect();
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
                &self.tags.iter().map(Clone::clone).collvect().join(", ")
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
    #[serde(default, rename = "centerText")]
    pub center_text: bool,
    #[serde(default)]
    pub scryfall: bool,
    #[serde(default, rename = "textStyle")]
    pub text_style: Vec<String>,
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

#[derive(Deserialize, Serialize)]
pub enum TextStyle {
    #[serde(rename = "slim-margins")]
    SlimMargins,
    #[serde(rename = "no-line-spacing")]
    NoLineSpacing,
    #[serde(rename = "centered-text")]
    CenteredText,
    #[serde(rename = "bigger-text")]
    BigText,
    #[serde(rename = "small-text")]
    SmallText,
    #[serde(rename = "smaller-text")]
    SmallerText,
    #[serde(rename = "smallest-text")]
    SmallestText,
}

impl Display for TextStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_value(self).unwrap().as_str().unwrap())
    }
}
