use std::{collections::BTreeSet, fmt::Display};

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
    utils::iter::IterExt,
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
}

impl Search {
    pub fn dispatch(self, atomics: &AtomicCardsFile, decklist: &DeckList) -> anyhow::Result<()> {
        let searcher = Searcher::new(self)?;

        if decklist.is_empty() {
            let mut hits = atomics
                .data
                .values()
                .filter(|c| searcher.matches_cardoid(c))
                .collvect();
            hits.sort_by_key(|c| c.name());
            hits.iter().for_each(|c| searcher.print(*c));
        } else {
            let mut hits = decklist
                .iter()
                .filter(|p| searcher.matches_proxy(p))
                .collvect();
            hits.sort_by_key(|p| &p.name);
            hits.iter().for_each(|p| searcher.print(*p));
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
            name: Self::build_regexes(it.name)?,
            vname: Self::build_regexes(it.vname)?,
            color: Self::build_color(it.color, WUBRG::colorless())?,
            commander: Self::build_color(it.commander, WUBRG::wubrg())?,
            r#type: Self::build_regexes(it.r#type)?,
            vtype: Self::build_regexes(it.vtype)?,
            grep: Self::build_regexes(it.grep)?,
            vgrep: Self::build_regexes(it.vgrep)?,
            discord: it.discord,
        })
    }

    fn print<D>(&self, d: &D)
    where
        D: Display,
    {
        if self.discord {
            println!("{d:#}")
        } else {
            println!("{d}")
        }
    }

    fn matches_proxy(&self, proxy: &Proxy) -> bool {
        &self.tags < &proxy.tags && self.matches_cardoid(&proxy.cardoid)
    }

    fn matches_cardoid(&self, cardoid: &Cardoid) -> bool {
        cardoid.color_identity() < &self.commander
            && Self::regex_match(&self.name, &self.vname, cardoid.name())
            && cardoid.iter().any(|card| self.matches_card(card))
            && Self::regex_match(&self.grep, &self.vgrep, &format!("{cardoid}"))
    }

    fn matches_card(&self, card: &Card) -> bool {
        self.color < card.colors && Self::regex_match(&self.r#type, &self.vtype, &card.type_line)
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

    fn build_regexes(it: Vec<String>) -> anyhow::Result<Vec<Regex>> {
        let mut res = vec![];
        for s in it {
            res.push(Regex::new(&s)?)
        }
        Ok(res)
    }

    fn regex_match(pos: &[Regex], neg: &[Regex], data: &str) -> bool {
        pos.iter().all(|r| r.is_match(data)) && !neg.iter().any(|r| r.is_match(data))
    }
}
