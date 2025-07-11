use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    path::Path,
};

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use serde::Deserialize;

use crate::{
    atomic_cards::{
        types::{Type, WUBRG},
        AtomicCardsFile,
    },
    utils::ToS,
};

use super::Proxy;

#[derive(Clone, Debug, Default)]
pub struct DeckList(Vec<Proxy>);

impl DeckList {
    pub fn new() -> DeckList {
        Self(vec![])
    }

    pub fn add_card(&mut self, proxy: Proxy) {
        self.0.push(proxy)
    }

    pub fn load_str(data: &str, atomics: &AtomicCardsFile) -> anyhow::Result<DeckList> {
        let structure: DeckListFile = serde_json::from_str(&data)?;

        Ok(DeckList(structure.build(atomics)?))
    }

    pub fn load(path: &Path, atomics: &AtomicCardsFile) -> anyhow::Result<DeckList> {
        let data = std::fs::read_to_string(path)?;
        Self::load_str(&data, atomics)
    }

    pub fn card_names<F>(&self, filter: F) -> BTreeMap<String, usize>
    where
        F: Fn(&Proxy) -> bool,
    {
        let mut res = BTreeMap::new();

        for proxy in self {
            if filter(proxy) {
                *res.entry(proxy.name.clone()).or_insert(0) += proxy.repeats;
            }
        }

        res
    }

    pub fn extras(&self) -> Vec<&Proxy> {
        self.iter().filter(|p| !p.in_deck()).collect_vec()
    }

    pub fn count_cards<F>(&self, filter: F) -> usize
    where
        F: Fn(&Proxy) -> bool,
    {
        self.iter()
            .map(|p| if filter(p) { p.repeats } else { 0 })
            .sum()
    }

    pub fn categories<F>(&self, filter: F) -> IndexMap<String, IndexSet<String>>
    where
        F: Fn(&Proxy) -> bool,
    {
        let mut res = IndexMap::new();

        for proxy in &self.0 {
            if filter(proxy) {
                res.entry(proxy.category().unwrap_or(proxy.uncategorized()))
                    .or_insert_with(IndexSet::new)
                    .insert(proxy.name.clone());
            }
        }

        res
    }

    pub fn color_hist(&self) -> BTreeMap<WUBRG, usize> {
        let mut res = BTreeMap::new();

        for proxy in &self.0 {
            if !proxy.in_deck() {
                continue;
            }
            for card in &proxy.cardoid {
                if card.types.contains(&Type::Land) {
                    continue;
                }
                *res.entry(card.colors.clone()).or_insert(0) += proxy.repeats;
            }
        }

        res
    }

    pub fn color_id(&self) -> WUBRG {
        let mut res = BTreeSet::new();

        for proxy in &self.0 {
            if !proxy.in_deck() {
                continue;
            }
            for card in &proxy.cardoid {
                res.append(&mut card.color_identity.clone().0)
            }
        }

        WUBRG(res)
    }

    pub fn curve(&self) -> BTreeMap<usize, usize> {
        let mut res = BTreeMap::new();

        for proxy in &self.0 {
            if !proxy.in_deck() {
                continue;
            }
            for card in &proxy.cardoid {
                if card.types.contains(&Type::Land) {
                    continue;
                }
                *res.entry(card.mana_value as usize).or_insert(0) += proxy.repeats;
            }
        }

        res
    }

    pub fn tag_hist(&self) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();

        for proxy in &self.0 {
            if !proxy.in_deck() {
                continue;
            }
            for tag in &proxy.tags {
                *res.entry(tag.clone()).or_insert(0) += proxy.repeats;
            }
        }

        return res;
    }

    pub fn tags(&self) -> IndexMap<String, IndexSet<String>> {
        let mut res = IndexMap::new();

        for proxy in &self.0 {
            if !proxy.in_deck() {
                continue;
            }
            res.entry(proxy.name.clone())
                .or_insert(IndexSet::new())
                .append(&mut proxy.tags.clone())
        }

        return res;
    }

    pub fn type_hist(&self) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();

        for proxy in self {
            for card in &proxy.cardoid {
                let typeline = card
                    .supertypes
                    .iter()
                    .map(|t| format!("{}", t))
                    .chain(card.types.iter().map(|t| format!("{}", t)))
                    .collect::<Vec<_>>()
                    .join(" ");

                let count = res.entry(typeline).or_insert(0);
                *count += proxy.repeats;
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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

#[derive(Deserialize, Clone)]
#[serde(transparent)]
struct DeckListFile(IndexMap<String, Vec<Proxy>>);

impl DeckListFile {
    fn build(self, atomics: &AtomicCardsFile) -> anyhow::Result<Vec<Proxy>> {
        let mut res = vec![];
        let mut errors = vec![];

        Self::build_categorized(self.0, atomics, &mut res, &mut errors);

        if errors.is_empty() {
            Ok(res)
        } else {
            Err(DeckListBuildError(errors).into())
        }
    }

    fn build_categorized(
        categories: IndexMap<String, Vec<Proxy>>,
        atomics: &AtomicCardsFile,
        res: &mut Vec<Proxy>,
        errors: &mut Vec<String>,
    ) {
        for (category, mut vec) in categories {
            if !category.is_empty() {
                vec.iter_mut().for_each(|a| {
                    a.tags.insert_before(0, category.clone());
                });
            }
            Self::build_uncategorized(vec, atomics, res, errors);
        }
    }

    fn build_uncategorized(
        vec: Vec<Proxy>,
        atomics: &AtomicCardsFile,
        res: &mut Vec<Proxy>,
        errors: &mut Vec<String>,
    ) {
        for mut proxy in vec {
            if proxy.cardoid.is_empty() {
                let Some(cardoid) = atomics.data.get(&proxy.name).map(Clone::clone) else {
                    errors.push("Failed to find: ".s() + &proxy.name);
                    continue;
                };
                proxy.cardoid = cardoid;
            }
            res.push(proxy);
        }
    }
}

#[derive(Debug)]
pub struct DeckListBuildError(pub Vec<String>);

impl Display for DeckListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("The following errors occurred while reading decklist:\n")?;

        for name in &self.0 {
            f.write_fmt(format_args!("  {}\n", name))?;
        }

        Ok(())
    }
}

impl Error for DeckListBuildError {}
