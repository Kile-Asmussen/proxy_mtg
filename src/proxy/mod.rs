pub mod decklists;
pub mod deserializers;

use deserializers::OneOrMany;
use itertools::{EitherOrBoth, Itertools};
use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{
    atomic_cards::{cardoids::Cardoid, metadata::ForeignData, types::CardLayout},
    scryfall::ScryfallCard,
    utils::iter::IterExt,
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
    pub category: String,
    #[serde(default)]
    pub tags: BTreeSet<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default, deserialize_with = "OneOrMany::<ForeignData>::one_or_many")]
    pub customize: Vec<ForeignData>,
    #[serde(default)]
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
