use std::{
    collections::{
        hash_map::{self, Entry},
        BTreeMap, BTreeSet,
    },
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::atomic_cards::{AtomicCards, Cardoid};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Artoid {
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
    pub tags: BTreeSet<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default, skip_serializing, skip_deserializing)]
    pub cardoid: Option<Cardoid>,
}

fn repeats_default() -> usize {
    1
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckList(pub BTreeMap<String, Vec<Artoid>>);

impl DeckList {
    fn artoid_iter(&self) -> impl Iterator<Item = &Artoid> {
        self.0.iter().map(|(_, s)| s.iter()).flatten()
    }

    pub fn tag_histogram(&self) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();

        for ao in self.artoid_iter() {
            for tag in ao.tags.iter() {
                let count = res.entry(tag.clone()).or_insert(0);
                *count += 1;
            }
        }

        return res;
    }

    pub fn build(&mut self, atomics: &AtomicCards) -> Result<(), Vec<String>> {
        let mut failed_to_find = vec![];
        for (_, artoids) in self.0.iter_mut() {
            for artoid in artoids.iter_mut() {
                if let Some(cardoid) = atomics.data.get(&artoid.name) {
                    artoid.cardoid = Some(cardoid.clone())
                } else {
                    failed_to_find.push(artoid.name.clone())
                }
            }
        }

        if failed_to_find.is_empty() {
            Ok(())
        } else {
            Err(failed_to_find)
        }
    }
}
