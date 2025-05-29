use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    sync::atomic,
};

use rand::rand_core::block;
use serde::{Deserialize, Serialize};

use crate::atomic_cards::{
    cards::Cardoid,
    types::{Type, WUBRG},
    AtomicCardsFile,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(default, rename = "artFile")]
    pub art_file: Vec<PathBuf>,
    #[serde(default, rename = "artCredit")]
    pub art_credit: Vec<String>,
    #[serde(default, rename = "fullArt")]
    pub full_art: bool,
    #[serde(default, rename = "flavorText")]
    pub flavor_text: Vec<String>,
    #[serde(default)]
    pub copies: usize,
    #[serde(default, rename = "reminderText")]
    pub reminder_text: bool,
    #[serde(default = "repeats_default")]
    pub repeats: usize,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub tags: BTreeSet<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default, skip_serializing, skip_deserializing)]
    pub cardoid: Option<Cardoid>,
}

fn repeats_default() -> usize {
    1
}

impl Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(cardoid) = &self.cardoid else {
            return f.write_str("> ERROR: no such card");
        };

        cardoid.fmt(f);

        f.write_str("\n> # # #")?;

        f.write_fmt(format_args!("\n> category: {}", self.category))?;
        if !self.tags.is_empty() {
            f.write_str("\n> tags: ");
            f.write_str(
                &self
                    .tags
                    .iter()
                    .map(Clone::clone)
                    .collect::<Vec<_>>()
                    .join(", "),
            )?;
        }

        if self.repeats > 1 {
            f.write_fmt(format_args!("> copies: {}", self.repeats))?;
        }

        return Ok(());
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckList(Vec<Proxy>);

impl DeckList {
    pub fn load(path: &Path, atomics: &AtomicCardsFile) -> Result<DeckList, Box<dyn Error>> {
        let decklist_file = std::fs::read_to_string(path)?;
        let decklist_structure: BTreeMap<String, Vec<Proxy>> =
            serde_json::from_str(&decklist_file)?;
        let mut res = Self(vec![]);

        for (mut category, mut vec) in decklist_structure.into_iter() {
            vec.sort_by_key(|a| a.name.clone());
            for mut artoid in vec {
                artoid.category = category.clone();
                res.0.push(artoid);
            }
        }

        res.build(&atomics).map_err(|nf| DeckListBuildError(nf))?;

        Ok(res)
    }

    fn build(&mut self, atomics: &AtomicCardsFile) -> Result<(), Vec<String>> {
        let mut failed_to_find = vec![];
        for artoid in &mut self.0 {
            if let Some(cardoid) = atomics.data.get(&artoid.name) {
                artoid.cardoid = Some(cardoid.clone())
            } else {
                failed_to_find.push(artoid.name.clone())
            }
        }

        if failed_to_find.is_empty() {
            Ok(())
        } else {
            Err(failed_to_find)
        }
    }

    pub fn card_names(&self) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();

        for artoid in self {
            *res.entry(artoid.name.clone()).or_insert(0) += artoid.repeats;
        }

        res
    }

    pub fn count_cards(&self) -> usize {
        Self::count_cards_raw(self)
    }

    pub fn count_cards_raw<'a, I>(artoids: I) -> usize
    where
        I: IntoIterator<Item = &'a Proxy>,
    {
        artoids.into_iter().map(|a| a.repeats).sum()
    }

    pub fn categories(&self) -> BTreeMap<String, BTreeSet<String>> {
        let mut res = BTreeMap::new();

        for artoid in &self.0 {
            res.entry(artoid.category.clone())
                .or_insert_with(BTreeSet::new)
                .insert(artoid.name.clone());
        }

        res
    }

    pub fn color_hist(&self) -> BTreeMap<BTreeSet<WUBRG>, usize> {
        let mut res = BTreeMap::new();

        for artoid in &self.0 {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };
            for card in cardoid {
                if card.types.contains(&Type::Land) {
                    continue;
                }
                *res.entry(card.colors.clone()).or_insert(0) += artoid.repeats;
            }
        }

        res
    }

    pub fn color_id(&self) -> BTreeSet<WUBRG> {
        let mut res = BTreeSet::new();

        for artoid in &self.0 {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };
            for card in cardoid {
                res.append(&mut card.color_identity.clone())
            }
        }

        res
    }

    pub fn curve(&self) -> BTreeMap<usize, usize> {
        let mut res = BTreeMap::new();

        for artoid in &self.0 {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };
            for card in cardoid {
                if card.types.contains(&Type::Land) {
                    continue;
                }
                *res.entry(card.mana_value as usize).or_insert(0) += artoid.repeats;
            }
        }

        res
    }

    pub fn tag_hist(&self) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();

        for artoid in &self.0 {
            for tag in &artoid.tags {
                *res.entry(tag.clone()).or_insert(0) += artoid.repeats;
            }
        }

        return res;
    }

    pub fn type_hist(&self) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();

        for artoid in &self.0 {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };
            for card in cardoid {
                let typeline = card
                    .supertypes
                    .iter()
                    .map(|t| format!("{}", t))
                    .chain(card.types.iter().map(|t| format!("{}", t)))
                    .collect::<Vec<_>>()
                    .join(" ");

                let count = res.entry(typeline).or_insert(0);
                *count += artoid.repeats;
            }
        }

        return res;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Proxy> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Proxy> {
        self.into_iter()
    }
}

impl IntoIterator for DeckList {
    type Item = Proxy;

    type IntoIter = <Vec<Proxy> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a DeckList {
    type Item = &'a Proxy;

    type IntoIter = <&'a Vec<Proxy> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> IntoIterator for &'a mut DeckList {
    type Item = &'a mut Proxy;

    type IntoIter = <&'a mut Vec<Proxy> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

#[derive(Debug)]
pub struct DeckListBuildError(pub Vec<String>);

impl Display for DeckListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("The following cards were not found:\n")?;

        for name in &self.0 {
            f.write_fmt(format_args!("  {}\n", name))?;
        }

        Ok(())
    }
}

impl Error for DeckListBuildError {}
