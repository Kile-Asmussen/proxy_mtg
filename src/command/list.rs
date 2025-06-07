use clap::Parser;
use rand::{seq::SliceRandom, SeedableRng};
use regex::Regex;

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use crate::{
    atomic_cards::types::*,
    proxy::{decklists::DeckList, Proxy},
    utils::iter::*,
};

#[derive(Parser, Debug, Clone)]
pub struct List {
    #[arg(value_name = "FILE")]
    pub decklist: PathBuf,
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
    pub sideboard: bool,
    #[arg(long)]
    pub tokens: bool,
    #[arg(long)]
    pub lands: bool,
    #[arg(long)]
    pub pips: bool,
}

impl List {
    pub fn decklist_file(&self) -> &Path {
        &self.decklist
    }

    pub fn dispatch(&self, decklist: &DeckList) -> anyhow::Result<()> {
        if decklist.is_empty() {}

        if self.id {
            println!();
            Self::print_color_id(decklist);
        }

        if self.cards {
            println!();
            Self::print_cards(decklist, "Cards", |p| p.in_deck());
        }

        if self.sideboard {
            println!();
            Self::print_cards(decklist, "Sideboard", |p| p.sideboard);
        }

        if self.tokens {
            println!();
            Self::print_cards(decklist, "Tokens", |p| p.layout() == &CardLayout::Token);
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

        if self.pips {
            println!();
            Self::print_pips(decklist);
        }

        println!();

        Ok(())
    }

    pub fn print_cards<F>(list: &DeckList, listing: &str, filter: F)
    where
        F: Fn(&Proxy) -> bool,
    {
        let cats = list.categories(&filter);
        let mut cards = list.card_names(&filter);

        println!("{} ({}):", listing, list.count_cards(&filter));

        for (cat, names) in &cats {
            let count: usize = names.iter().map(|s| cards.get(s).unwrap_or(&0usize)).sum();
            println!("  {} ({}):", cat, count);
            for name in names {
                let n = *cards.get(name).unwrap_or(&0usize);
                if n == 1 {
                    println!("    {}", name);
                } else {
                    println!("    {} x {}", n, name);
                }
                cards.remove(name);
            }
        }
    }

    pub fn print_creatures(decklist: &DeckList) {
        let names = decklist.card_names(Proxy::in_deck);
        let mut creatures = BTreeMap::new();
        for proxy in decklist {
            for card in &proxy.cardoid {
                if !card.types.contains(&Type::Creature) {
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

    pub fn print_power_curve(decklist: &DeckList, power: bool) {
        let mut pt_count = BTreeMap::new();
        for proxy in decklist {
            for card in &proxy.cardoid {
                if !card.types.contains(&Type::Creature) {
                    continue;
                }
                let pt = if power {
                    (card.power.clone(), card.toughness.clone())
                } else {
                    (card.toughness.clone(), card.power.clone())
                };
                *pt_count.entry(pt).or_insert(0) += proxy.repeats;
            }
        }

        println!("P/T curve:");
        Self::print_histo(if power {
            pt_count
                .into_iter()
                .map(|((p, t), n)| (format!("{p}/{t}"), n))
                .collvect()
        } else {
            pt_count
                .into_iter()
                .map(|((t, p), n)| (format!("{p}/{t}"), n))
                .collvect()
        });
    }

    pub fn print_colors(decklist: &DeckList) {
        println!("Colors:");
        for (color, n) in decklist.color_hist() {
            print!("  {} x {}", n, WUBRG::render(&color))
        }
    }

    pub fn print_color_id(decklist: &DeckList) {
        println!("Color Identity: {}", WUBRG::render(&decklist.color_id()))
    }

    pub fn print_color_hist(decklist: &DeckList) {
        println!("Color Histogram:");
        Self::print_histo(
            decklist
                .color_hist()
                .into_iter()
                .map(|s| (WUBRG::render(&s.0), s.1))
                .collvect(),
        );
    }

    pub fn print_mana_curve(decklist: &DeckList) {
        println!("Mana Curve:");
        let curve = decklist.curve();
        let Some(max) = curve.keys().max() else {
            println!("  no curve");
            return;
        };
        Self::print_histo(
            (0..=*max)
                .into_iter()
                .map(|n| (n.to_string(), curve[&n]))
                .collvect(),
        );
    }

    pub fn print_tag_hist(decklist: &DeckList) {
        println!("Tags:");
        Self::print_histo(decklist.tag_hist().into_iter().collvect());
    }

    pub fn print_type_hist(decklist: &DeckList) {
        println!("Card Types:");
        Self::print_histo(decklist.type_hist().into_iter().collvect());
    }

    pub fn print_example_hand(decklist: &DeckList) {
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
            println!("  {}. {n}", i + 1);
        }
    }

    pub fn print_lands(decklist: &DeckList) {
        println!("Land base:");
        let mut basic = 0usize;
        let mut basic_names = BTreeSet::new();
        let mut tapland = 0usize;
        let mut tapland_names = BTreeSet::new();
        let mut nonmana = 0usize;
        let mut nonmana_names = BTreeSet::new();
        let mut total = 0usize;
        for proxy in decklist {
            for land in &proxy.cardoid {
                if !land.types.contains(&Type::Land) {
                    continue;
                }

                total += proxy.repeats;

                if land.text.contains("enters tapped") {
                    tapland += proxy.repeats;
                    tapland_names.insert(land.name.clone());
                }

                if land.supertypes.contains(&Supertype::Basic) {
                    basic += proxy.repeats;
                    basic_names.insert(land.name.clone());
                }

                if !land.text.contains("{T}: Add") {
                    nonmana += proxy.repeats;
                    nonmana_names.insert(land.name.clone());
                }
            }
        }

        println!("  {} x total", total);
        println!("  {} x basic", basic);
        for name in &basic_names {
            println!("    {name}");
        }
        println!("  {} x tapland", tapland);
        for name in &tapland_names {
            println!("    {name}");
        }
        println!("  {} x nonmana land", nonmana);
        for name in &nonmana_names {
            println!("    {name}");
        }
    }

    pub fn print_creature_types(decklist: &DeckList) {
        let mut types = BTreeMap::new();
        for proxy in decklist {
            for card in &proxy.cardoid {
                if !card.types.contains(&Type::Creature) {
                    continue;
                }
                for subtypes in &card.subtypes {
                    *types.entry(subtypes.clone()).or_insert(0) += proxy.repeats;
                }
            }
        }

        println!("Creature types:");
        Self::print_histo(types.into_iter().collvect())
    }

    pub fn print_pips(decklist: &DeckList) {
        let mut res = BTreeMap::new();
        let re = Regex::new(r"\{.*?\}").unwrap();
        for proxy in decklist {
            if !proxy.in_deck() {
                continue;
            }
            for card in &proxy.cardoid {
                for pip in re.find_iter(&card.mana_cost) {
                    *res.entry(pip.as_str().to_string()).or_insert(0) += proxy.repeats;
                }
            }
        }

        let mut res = res.into_iter().collvect();
        res.sort_by_key(|x| -(x.1 as isize));
        println!("Mana Symbols:");
        Self::print_histo(res);
    }

    fn print_histo(things: Vec<(String, usize)>) {
        let width = things.iter().map(|s| s.0.len()).max().unwrap_or(0) + 1;

        for (mut thing, n) in things {
            thing += &vec![" "; width - thing.len()].join("");
            println!(
                "  {thing}{}{}",
                vec!["*"; n].join(""),
                if n > 7 {
                    format!(" ({n})")
                } else {
                    "".to_string()
                }
            );
        }
    }
}
