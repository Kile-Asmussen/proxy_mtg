use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
};

use clap::Parser;
use regex::Regex;

use anyhow::anyhow;

use crate::{
    atomic_cards::{
        cardoids::Cardoid,
        cards::Card,
        types::{CardLayout, Side, WUBRG},
        AtomicCardsFile,
    },
    proxy::{decklists::DeckList, Proxy},
    utils::{
        iter::IterExt,
        printers::{TextPrinter, ToText},
        symbolics::{DiscordEmoji, NothingReplacer},
    },
};

#[derive(Parser, Debug, Clone)]
pub struct Search {
    #[arg(long, short)]
    pub tag: Vec<String>,
    #[arg(long = "name")]
    pub name: Vec<String>,
    #[arg(long)]
    pub color: Option<String>,
    #[arg(long)]
    pub commander: Option<String>,
    #[arg(long)]
    pub vname: Vec<String>,
    #[arg(long)]
    pub r#type: Vec<String>,
    #[arg(long)]
    pub vtype: Vec<String>,
    #[arg(long)]
    pub grep: Vec<String>,
    #[arg(long)]
    pub vgrep: Vec<String>,
    #[arg(long)]
    pub discord: bool,
    #[arg(short, long)]
    pub case: bool,
    #[arg(value_name = "OFILE")]
    pub decklist: Option<PathBuf>,
}

impl Search {
    pub fn decklist_file(&self) -> &Path {
        self.decklist
            .as_ref()
            .map(|p| p.as_ref())
            .unwrap_or(Path::new(""))
    }

    pub fn dispatch(self, atomics: &AtomicCardsFile, decklist: &DeckList) -> anyhow::Result<()> {
        let searcher = Searcher::new(self)?;

        if decklist.is_empty() {
            let mut hits = atomics
                .data
                .values()
                .filter(|c| searcher.matches_cardoid(c))
                .collvect();
            hits.sort_by_key(|c| c.name());
            hits.iter().for_each(|c| searcher.print_cardoid(*c));
        } else {
            let mut hits = decklist
                .iter()
                .filter(|p| searcher.matches_proxy(p))
                .collvect();
            hits.sort_by_key(|p| &p.name);
            hits.iter().for_each(|p| searcher.print_proxy(*p));
        }

        Ok(())
    }
}

struct Searcher {
    tags: BTreeSet<String>,
    color: BTreeSet<WUBRG>,
    commander: BTreeSet<WUBRG>,
    name: Vec<Regex>,
    vname: Vec<Regex>,
    r#type: Vec<Regex>,
    vtype: Vec<Regex>,
    grep: Vec<Regex>,
    vgrep: Vec<Regex>,
    discord: bool,
}

impl Searcher {
    fn new(it: Search) -> anyhow::Result<Self> {
        Ok(Self {
            tags: BTreeSet::from_iter(it.tag.into_iter()),
            name: Self::build_regexes(it.case, it.name)?,
            vname: Self::build_regexes(it.case, it.vname)?,
            color: Self::build_color(it.color, WUBRG::colorless())?,
            commander: Self::build_color(it.commander, WUBRG::wubrg())?,
            r#type: Self::build_regexes(it.case, it.r#type)?,
            vtype: Self::build_regexes(it.case, it.vtype)?,
            grep: Self::build_regexes(it.case, it.grep)?,
            vgrep: Self::build_regexes(it.case, it.vgrep)?,
            discord: it.discord,
        })
    }

    fn print_cardoid(&self, c: &Cardoid) {
        println!();
        if self.discord {
            println!("{}", TextPrinter(&DiscordEmoji, ToText::Cardoid(c)))
        } else {
            println!("{}", TextPrinter(&NothingReplacer, ToText::Cardoid(c)))
        }
    }

    fn print_proxy(&self, p: &Proxy) {
        println!();
        if self.discord {
            println!("{}", TextPrinter(&DiscordEmoji, ToText::Proxy(p)))
        } else {
            println!("{}", TextPrinter(&NothingReplacer, ToText::Proxy(p)))
        }
    }

    fn matches_proxy(&self, proxy: &Proxy) -> bool {
        &self.tags < &proxy.tags && self.matches_cardoid(&proxy.cardoid)
    }

    fn matches_cardoid(&self, cardoid: &Cardoid) -> bool {
        cardoid.color_identity() < &self.commander
            && Self::regex_match(&self.name, &self.vname, cardoid.name())
            && cardoid.iter().any(|card| self.matches_card(card))
    }

    fn matches_card(&self, card: &Card) -> bool {
        self.color < card.colors
            && Self::regex_match(&self.r#type, &self.vtype, &card.type_line)
            && Self::regex_match(&self.grep, &self.vgrep, &card.text)
    }

    fn build_color(it: Option<String>, or: BTreeSet<WUBRG>) -> anyhow::Result<BTreeSet<WUBRG>> {
        let Some(it) = it else {
            return Ok(or);
        };
        if it == "C" || it == "c" {
            return Ok(BTreeSet::new());
        }
        let mut res = BTreeSet::new();
        for c in it.chars() {
            res.insert(match c {
                'W' | 'w' => WUBRG::W,
                'U' | 'u' => WUBRG::U,
                'B' | 'b' => WUBRG::B,
                'R' | 'r' => WUBRG::R,
                'G' | 'g' => WUBRG::R,
                c => return Err(anyhow!("{} is not a color", c)),
            });
        }
        return Ok(res);
    }

    fn build_regexes(case: bool, it: Vec<String>) -> anyhow::Result<Vec<Regex>> {
        let mut res = vec![];
        for s in it {
            let pref = if case {
                "".to_string()
            } else {
                "(?i)".to_string()
            };
            res.push(Regex::new(&(pref + &s))?)
        }
        Ok(res)
    }

    fn regex_match(pos: &[Regex], neg: &[Regex], data: &str) -> bool {
        pos.iter().all(|r| r.is_match(data)) && !neg.iter().any(|r| r.is_match(data))
    }
}
