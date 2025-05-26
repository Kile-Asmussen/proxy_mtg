use std::fmt::Debug;
use std::iter;
use std::path::{Display, Path, PathBuf};

use std::collections::{BTreeMap, BTreeSet};

use clap::{Parser, Subcommand};

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
    pub color_id: bool,
    #[arg(long)]
    pub colors: bool,
    #[arg(long)]
    pub type_hist: bool,
    #[arg(long)]
    pub tag_hist: bool,
    #[arg(long)]
    pub mana_curve: bool,
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
                color_id: true,
                colors: true,
                type_hist: true,
                tag_hist: true,
                mana_curve: true,
                cards: true,
            }
        }

        if this.color_id {
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

        if this.mana_curve {
            println!();
            Self::print_curve(decklist);
        }

        if this.type_hist {
            println!();
            Self::print_type_hist(decklist);
        }

        if this.tag_hist {
            println!();
            Self::print_tag_hist(decklist);
        }

        println!();
    }

    fn print_cards(list: &DeckList) {
        let cats: BTreeMap<String, Vec<Artoid>> = list.categories();

        println!("Cards:");
        for (cat, artoids) in &cats {
            println!("  {}:", cat);
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
