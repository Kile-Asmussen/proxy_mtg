use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    sync::atomic,
};

use rand::rand_core::block;
use serde::{Deserialize, Serialize};

use crate::atomic_cards::{cards::Cardoid, types::*, AtomicCardsFile};

use super::Proxy;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckList(Vec<Proxy>);

impl DeckList {
    pub fn load(path: &Path, atomics: &AtomicCardsFile) -> anyhow::Result<DeckList> {
        let decklist_file = std::fs::read_to_string(path)?;
        let decklist_structure: BTreeMap<String, Vec<Proxy>> =
            serde_json::from_str(&decklist_file)?;
        let mut res = Self(vec![]);
        let mut failed_to_find = vec![];

        for (mut category, mut vec) in decklist_structure {
            vec.sort_by_key(|a| a.name.clone());
            for mut artoid in vec {
                artoid.category = category.clone();
                if let Some(cardoid) = atomics.data.get(&artoid.name).map(Clone::clone) {
                    artoid.cardoid = cardoid;
                    res.0.push(artoid);
                } else {
                    failed_to_find.push(artoid.name);
                }
            }
        }

        if failed_to_find.is_empty() {
            Ok(res)
        } else {
            Err(DeckListBuildError(failed_to_find).into())
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
            for card in &artoid.cardoid {
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
            for card in &artoid.cardoid {
                res.append(&mut card.color_identity.clone())
            }
        }

        res
    }

    pub fn curve(&self) -> BTreeMap<usize, usize> {
        let mut res = BTreeMap::new();

        for artoid in &self.0 {
            for card in &artoid.cardoid {
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
            for card in &artoid.cardoid {
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
