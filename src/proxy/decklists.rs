use crate::utils::iter::IterExt;

use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckList(Vec<Proxy>);

impl DeckList {
    pub fn load(path: &Path, atomics: &AtomicCardsFile) -> anyhow::Result<DeckList> {
        let decklist_file = std::fs::read_to_string(path)?;
        let decklist_structure: BTreeMap<String, Vec<Proxy>> =
            serde_json::from_str(&decklist_file)?;
        let mut res = Self(vec![]);
        let mut errors = vec![];

        for (mut category, mut vec) in decklist_structure {
            vec.sort_by_key(|a| a.name.clone());
            for mut proxy in vec {
                proxy.category = category.clone();

                if proxy.cardoid.is_empty() {
                    let Some(cardoid) = atomics.data.get(&proxy.name).map(Clone::clone) else {
                        errors.push("Failed to find: ".to_string() + &proxy.name);
                        continue;
                    };
                    proxy.cardoid = cardoid;
                }

                let num_arts = proxy.art_urls.len();
                let num_credits = proxy.art_urls.len();

                if num_arts != num_credits {
                    errors.push("Missing art credits: ".to_string() + &proxy.name);
                    continue;
                }

                res.0.push(proxy);
            }
        }

        if errors.is_empty() {
            Ok(res)
        } else {
            Err(DeckListBuildError(errors).into())
        }
    }

    pub fn card_names(&self) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();

        for proxy in self {
            if proxy.in_deck() {
                *res.entry(proxy.name.clone()).or_insert(0) += proxy.repeats;
            }
        }

        res
    }

    pub fn extras(&self) -> Vec<&Proxy> {
        self.iter().filter(|p| !p.in_deck()).collvect()
    }

    pub fn count_cards(&self) -> usize {
        self.iter()
            .map(|p| if p.in_deck() { p.repeats } else { 0 })
            .sum()
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
        f.write_str("The following errors occurred while reading decklist:\n")?;

        for name in &self.0 {
            f.write_fmt(format_args!("  {}\n", name))?;
        }

        Ok(())
    }
}

impl Error for DeckListBuildError {}
