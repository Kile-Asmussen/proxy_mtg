use std::fmt::Debug;
use std::iter;
use std::path::{Display, Path, PathBuf};

use std::collections::{BTreeMap, BTreeSet};

use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::atomic_cards::WUBRG;
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
    pub identity: bool,
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
}

impl List {
    fn dispatch(&self, decklist: &DeckList) {
        let mut this = self.clone();

        if this.all {
            this = List {
                decklist: this.decklist,
                all: true,
                identity: true,
                colors: true,
                types: true,
                tags: true,
                curve: true,
                cards: true,
                hand: true,
            }
        }

        if this.identity {
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

        println!("Example Hand:");
        for n in &names[0..=7] {
            println!("  {}", n);
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
