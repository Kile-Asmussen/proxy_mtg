use std::fmt::Debug;
use std::iter;
use std::path::{Display, Path, PathBuf};

use std::collections::{BTreeMap, BTreeSet};

use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::atomic_cards::CardType::Land;
use crate::atomic_cards::{Supertype, WUBRG};
use crate::decklist::{self, Artoid};
use crate::{atomic_cards::AtomicCards, decklist::DeckList};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Command {
    List(List),
    Build(Build),
}

impl Command {
    pub fn decklist_file(&self) -> &Path {
        match self {
            Self::List(l) => l.decklist.as_ref(),
            Self::Build(b) => b.decklist.as_ref(),
        }
    }

    pub fn dispatch(&self, decklist: &DeckList) {
        match self {
            Self::List(l) => l.dispatch(decklist),
            Self::Build(b) => b.dispatch(decklist),
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub struct List {
    #[arg(value_name = "FILE")]
    pub decklist: PathBuf,
    #[arg(long)]
    pub all: bool,
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
    pub hand: bool,
    #[arg(long)]
    pub cards: bool,
    #[arg(long)]
    pub lands: bool,
}

impl List {
    fn dispatch(&self, decklist: &DeckList) {
        let mut this = self.clone();

        if this.all {
            this = List {
                decklist: this.decklist,
                all: true,
                id: true,
                colors: true,
                types: true,
                tags: true,
                curve: true,
                hand: true,
                cards: true,
                lands: true,
            }
        }

        if this.id {
            println!();
            Self::print_color_id(decklist);
        }

        if this.cards {
            println!();
            Self::print_cards(decklist);
        }

        if this.colors {
            println!();
            Self::print_color_hist(decklist);
        }

        if this.curve {
            println!();
            Self::print_curve(decklist);
        }

        if this.types {
            println!();
            Self::print_type_hist(decklist);
        }

        if this.tags {
            println!();
            Self::print_tag_hist(decklist);
        }

        if this.hand {
            println!();
            Self::print_example_hand(decklist);
        }

        if this.lands {
            println!();
            Self::print_lands(decklist);
        }

        println!();
    }

    fn print_cards(list: &DeckList) {
        let cats: BTreeMap<String, Vec<Artoid>> = list.categories();

        println!("Cards ({}):", DeckList::count_cards(&list.0));
        for (cat, artoids) in &cats {
            println!("  {} ({}):", cat, DeckList::count_cards(artoids));
            for artoid in artoids {
                println!("    {} x {}", artoid.repeats, artoid.name);
            }
        }
    }

    fn print_colors(decklist: &DeckList) {
        println!("Colors:");
        for (color, n) in decklist.color_hist() {
            print!("  {} x {}", n, Self::wubrg(color))
        }
    }

    fn print_color_id(decklist: &DeckList) {
        println!("Color Identity: {}", Self::wubrg(decklist.color_id()))
    }

    fn print_color_hist(decklist: &DeckList) {
        println!("Color Histogram:");
        for (color, n) in decklist.color_hist() {
            println!("  {} x {}", n, Self::wubrg(color));
        }
    }

    fn print_curve(decklist: &DeckList) {
        println!("Mana Curve:");
        let curve = decklist.curve();
        let Some(max) = curve.keys().max() else {
            println!("  no curve");
            return;
        };
        for mv in 0..=*max {
            let n = *curve.get(&mv).unwrap_or(&0);
            println!("  {} {}", mv, iter::repeat("*").take(n).collect::<String>())
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
            .0
            .iter()
            .flat_map(|x| vec![x.name.clone(); x.repeats])
            .collect::<Vec<_>>();

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
        for artoid in &decklist.0 {
            let Some(cardoid) = &artoid.cardoid else {
                continue;
            };

            if !cardoid.0[0].types.contains(&Land) {
                continue;
            }

            let land = &cardoid.0[0];

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

    fn wubrg(colors: BTreeSet<WUBRG>) -> String {
        let res = colors
            .into_iter()
            .map(|c| format!("{:?}", c))
            .collect::<Vec<_>>()
            .join("");

        if res.is_empty() {
            "C".to_string()
        } else {
            res
        }
    }
}

#[derive(Parser, Debug)]
pub struct Build {
    #[arg(value_name = "FILE")]
    pub decklist: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}

impl Build {
    fn dispatch(&self, decklist: &DeckList) {}
}
