use std::fmt::Debug;
use std::iter;
use std::path::{Display, Path, PathBuf};

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic;

use clap::builder::Str;
use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use regex::Regex;

use crate::atomic_cards::CardType::{self, Land};
use crate::atomic_cards::{Cardoid, Supertype, WUBRG};
use crate::decklist::{self, Artoid};
use crate::vec_entry::IterExt;
use crate::{atomic_cards::AtomicCards, decklist::DeckList};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[arg(value_name = "FILE")]
    pub decklist: PathBuf,
    #[command(subcommand)]
    pub subcommand: ListBuildSearch,
}

#[derive(Subcommand, Debug)]
pub enum ListBuildSearch {
    List(List),
    Build(Build),
    Search(Search),
}

impl ListBuildSearch {
    pub fn dispatch(&self, atomics: &AtomicCards, decklist: &DeckList) {
        match self {
            Self::List(l) => l.dispatch(decklist),
            Self::Build(b) => b.dispatch(decklist),
            Self::Search(s) => s.dispatch(atomics, decklist),
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub struct List {
    #[arg(long)]
    pub id: bool,
    #[arg(long)]
    pub colors: bool,
    #[arg(long)]
    pub types: bool,
    #[arg(long)]
    pub tags: bool,
    #[arg(long)]
    pub curve: bool,
    #[arg(long)]
    pub creatures: bool,
    #[arg(long)]
    pub creature_types: bool,
    #[arg(long)]
    pub p_t: bool,
    #[arg(long)]
    pub t_p: bool,
    #[arg(long)]
    pub hand: bool,
    #[arg(long)]
    pub cards: bool,
    #[arg(long)]
    pub lands: bool,
}

impl List {
    fn dispatch(&self, decklist: &DeckList) {
        if self.id {
            println!();
            Self::print_color_id(decklist);
        }

        if self.cards {
            println!();
            Self::print_cards(decklist);
        }

        if self.colors {
            println!();
            Self::print_color_hist(decklist);
        }

        if self.curve {
            println!();
            Self::print_mana_curve(decklist);
        }

        if self.types {
            println!();
            Self::print_type_hist(decklist);
        }

        if self.tags {
            println!();
            Self::print_tag_hist(decklist);
        }

        if self.creatures {
            println!();
            Self::print_creatures(decklist);
        }

        if self.creature_types {
            println!();
            Self::print_creature_types(decklist);
        }

        if self.p_t {
            println!();
            Self::print_power_curve(decklist, true);
        }

        if self.t_p {
            println!();
            Self::print_power_curve(decklist, false);
        }

        if self.hand {
            println!();
            Self::print_example_hand(decklist);
        }

        if self.lands {
            println!();
            Self::print_lands(decklist);
        }

        println!();
    }

    fn print_cards(list: &DeckList) {
        let cats = list.categories();
        let mut cards = list.card_names();

        println!("Cards ({}):", list.count_cards());
        for (cat, names) in &cats {
            let count: usize = names.iter().map(|s| cards.get(s).unwrap_or(&0usize)).sum();
            println!("  {} ({}):", cat, count);
            for name in names {
                println!("    {} x {}", *cards.get(name).unwrap_or(&0usize), name);
                cards.remove(name);
            }
        }
    }

    fn print_creatures(decklist: &DeckList) {
        let names = decklist.card_names();
        let mut creatures = BTreeMap::new();
        for artoid in decklist {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };
            for card in cardoid {
                if !card.types.contains(&CardType::Creature) {
                    continue;
                }
                let pt = format!("{}/{}", &card.power, &card.toughness);
                creatures.insert(card.name.clone(), pt.clone());
            }
        }

        println!("Creatures:");
        for (critter, pt) in creatures {
            println!(
                "  {} x {} {}",
                names.get(&critter).unwrap_or(&0),
                critter,
                pt
            );
        }
    }

    fn print_power_curve(decklist: &DeckList, power: bool) {
        let mut pt_count = BTreeMap::new();
        for artoid in decklist {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };
            for card in cardoid {
                if !card.types.contains(&CardType::Creature) {
                    continue;
                }
                let pt = if power {
                    (card.power.clone(), card.toughness.clone())
                } else {
                    (card.toughness.clone(), card.power.clone())
                };
                *pt_count.entry(pt).or_insert(0) += artoid.repeats;
            }
        }

        println!("P/T curve:");
        for (pt, c) in pt_count {
            if power {
                println!("  {}/{} {}", pt.0, pt.1, vec!["*"; c].join(""));
            } else {
                println!("  {}/{} {}", pt.1, pt.0, vec!["*"; c].join(""));
            }
        }
    }

    fn print_colors(decklist: &DeckList) {
        println!("Colors:");
        for (color, n) in decklist.color_hist() {
            print!("  {} x {}", n, WUBRG::wubrg(&color))
        }
    }

    fn print_color_id(decklist: &DeckList) {
        println!("Color Identity: {}", WUBRG::wubrg(&decklist.color_id()))
    }

    fn print_color_hist(decklist: &DeckList) {
        println!("Color Histogram:");
        for (color, n) in decklist.color_hist() {
            println!("  {} x {}", n, WUBRG::wubrg(&color));
        }
    }

    fn print_mana_curve(decklist: &DeckList) {
        println!("Mana Curve:");
        let curve = decklist.curve();
        let Some(max) = curve.keys().max() else {
            println!("  no curve");
            return;
        };
        for mv in 0..=*max {
            let n = *curve.get(&mv).unwrap_or(&0);
            println!("  {} {}", mv, vec!["*"; n].join(""))
        }
        for (cmc, n) in decklist.curve() {}
    }

    fn print_tag_hist(decklist: &DeckList) {
        println!("Tags:");
        for (tag, n) in decklist.tag_hist() {
            println!("  {} x {}", n, tag);
        }
    }

    fn print_type_hist(decklist: &DeckList) {
        println!("Card Types:");
        for (types, n) in decklist.type_hist() {
            println!("  {} x {}", n, types);
        }
    }

    fn print_example_hand(decklist: &DeckList) {
        let mut names = decklist
            .iter()
            .flat_map(|x| vec![x.name.clone(); x.repeats])
            .collvect();

        let mut rng = rand::rngs::SmallRng::from_os_rng();
        names.shuffle(&mut rng);

        let (hand, _) = names.split_at(7);
        let mut hand = hand.to_owned();
        hand.sort();
        println!("Example Hand:");
        for (i, n) in hand.iter().enumerate() {
            println!("  {}. {}", i + 1, n);
        }
    }

    fn print_lands(decklist: &DeckList) {
        println!("Land base:");
        let mut basic = 0usize;
        let mut basic_names = BTreeSet::new();
        let mut tapland = 0usize;
        let mut tapland_names = BTreeSet::new();
        let mut nonmana = 0usize;
        let mut nonmana_names = BTreeSet::new();
        let mut total = 0usize;
        for artoid in decklist {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };

            for land in cardoid {
                if !land.types.contains(&Land) {
                    continue;
                }

                total += artoid.repeats;

                if land.text.contains("enters tapped") {
                    tapland += artoid.repeats;
                    tapland_names.insert(land.name.clone());
                }

                if land.supertypes.contains(&Supertype::Basic) {
                    basic += artoid.repeats;
                    basic_names.insert(land.name.clone());
                }

                if !land.text.contains("{T}: Add") {
                    nonmana += artoid.repeats;
                    nonmana_names.insert(land.name.clone());
                }
            }
        }

        println!("  {} x total", total);
        println!("  {} x basic", basic);
        for name in &basic_names {
            println!("    {}", name);
        }
        println!("  {} x tapland", tapland);
        for name in &tapland_names {
            println!("    {}", name);
        }
        println!("  {} x nonmana land", nonmana);
        for name in &nonmana_names {
            println!("    {}", name);
        }
    }

    fn print_creature_types(decklist: &DeckList) {
        let mut types = BTreeMap::new();
        for artoid in decklist {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };
            for card in cardoid {
                if !card.types.contains(&CardType::Creature) {
                    continue;
                }
                for subtypes in &card.subtypes {
                    *types.entry(subtypes.clone()).or_insert(0) += artoid.repeats;
                }
            }
        }

        println!("Creature types:");
        for (subtype, count) in types {
            println!("  {} {}", vec!["*"; count].join(""), subtype);
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub struct Build {
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}

impl Build {
    fn dispatch(&self, decklist: &DeckList) {}
}

#[derive(Parser, Debug, Clone)]
pub struct Search {
    #[arg(value_name = "CARDS", num_args = 1..)]
    pub cards: Vec<String>,
    #[arg(long, short)]
    pub search_all: bool,
    #[arg(long, short)]
    pub tag: Vec<String>,
    #[arg(long, short)]
    pub exp: Vec<String>,
    #[arg(long, short)]
    pub nexp: Vec<String>,
}

impl Search {
    pub fn dispatch(&self, atomics: &AtomicCards, decklist: &DeckList) {
        let mut exp = vec![];
        for pat in &self.exp {
            let Ok(pat) = Regex::new(pat) else {
                println!("Invalid regex: {}", pat);
                return;
            };
            exp.push(pat);
        }

        let mut nexp = vec![];
        for pat in &self.nexp {
            let Ok(pat) = Regex::new(pat) else {
                println!("Invalid regex: {}", pat);
                return;
            };
            nexp.push(pat);
        }

        let mut cards = self.cards.clone();
        if cards.is_empty() && self.search_all {
            cards.append(&mut atomics.data.keys().map(Clone::clone).collect());
        } else if cards.is_empty() {
            cards.append(&mut decklist.card_names().keys().map(Clone::clone).collect());
        }

        for card in &cards {
            let artoid = decklist.iter().find(|x| &x.name == card);

            if let Some(artoid) = artoid {
                if self.tag.iter().any(|t| !artoid.tags.contains(t)) {
                    continue;
                }
            }

            let cardoid = artoid
                .and_then(|a| a.cardoid.as_ref())
                .or_else(|| atomics.data.get(card));

            if let Some(cardoid) = cardoid {
                let res = format!("{}", cardoid);
                if exp.iter().all(|pat| pat.is_match(&res))
                    && nexp.iter().all(|pat| !pat.is_match(&res))
                {
                    println!();
                    if let Some(artoid) = artoid {
                        println!("{}", artoid);
                    } else {
                        println!("{}", cardoid);
                    }
                }
            } else {
                println!("No such card as `{}'", card);
            }
        }
    }
}
