use clap::Parser;
use rand::{seq::SliceRandom, SeedableRng};

use std::collections::{BTreeMap, BTreeSet};

use crate::{atomic_cards::types::*, proxy::decklist::DeckList, utils::iter::*};

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
    pub fn dispatch(&self, decklist: &DeckList) -> anyhow::Result<()> {
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

        Ok(())
    }

    pub fn print_cards(list: &DeckList) {
        let cats = list.categories();
        let mut cards = list.card_names(true);

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

    pub fn print_creatures(decklist: &DeckList) {
        let names = decklist.card_names(false);
        let mut creatures = BTreeMap::new();
        for artoid in decklist {
            for card in &artoid.cardoid {
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
        for artoid in decklist {
            for card in &artoid.cardoid {
                if !card.types.contains(&Type::Creature) {
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

    pub fn print_colors(decklist: &DeckList) {
        println!("Colors:");
        for (color, n) in decklist.color_hist() {
            print!("  {} x {}", n, WUBRG::wubrg(&color))
        }
    }

    pub fn print_color_id(decklist: &DeckList) {
        println!("Color Identity: {}", WUBRG::wubrg(&decklist.color_id()))
    }

    pub fn print_color_hist(decklist: &DeckList) {
        println!("Color Histogram:");
        for (color, n) in decklist.color_hist() {
            println!("  {} x {}", n, WUBRG::wubrg(&color));
        }
    }

    pub fn print_mana_curve(decklist: &DeckList) {
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

    pub fn print_tag_hist(decklist: &DeckList) {
        println!("Tags:");
        for (tag, n) in decklist.tag_hist() {
            println!("  {} x {}", n, tag);
        }
    }

    pub fn print_type_hist(decklist: &DeckList) {
        println!("Card Types:");
        for (types, n) in decklist.type_hist() {
            println!("  {} x {}", n, types);
        }
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
            println!("  {}. {}", i + 1, n);
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
        for artoid in decklist {
            for land in &artoid.cardoid {
                if !land.types.contains(&Type::Land) {
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

    pub fn print_creature_types(decklist: &DeckList) {
        let mut types = BTreeMap::new();
        for artoid in decklist {
            for card in &artoid.cardoid {
                if !card.types.contains(&Type::Creature) {
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
