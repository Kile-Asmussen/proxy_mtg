use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use clap::Parser;
use indexmap::IndexSet;
use regex::Regex;

use anyhow::anyhow;

use crate::{
    atomic_cards::{
        cardoids::Cardoid,
        cards::Card,
        types::{CardLayout, WUBRG},
        AtomicCardsFile,
    },
    proxy::{decklists::DeckList, Proxy},
    utils::{iter::IterExt, ToS},
};

#[derive(Parser, Debug, Clone)]
pub struct Search {
    #[arg(long)]
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
    pub text: Vec<String>,
    #[arg(long)]
    pub vtext: Vec<String>,
    #[arg(long)]
    pub grep: Vec<String>,
    #[arg(long)]
    pub vgrep: Vec<String>,
    #[arg(long)]
    pub sideboard: bool,
    #[arg(long)]
    pub funnies: bool,
    #[arg(long)]
    pub debug: bool,
    #[arg(long)]
    pub case_sensitive: bool,
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
            let mut hits = searcher.matches_cardoids(atomics.data.values());
            hits.sort_by_key(|c| c.name());
            hits.iter().for_each(|c| searcher.print_cardoid(*c));
        } else {
            let mut hits = searcher.match_proxies(decklist);
            hits.sort_by_key(|p| (p.category(), &p.name));
            hits.iter().for_each(|p| searcher.print_proxy(*p));
        }

        Ok(())
    }
}

struct Searcher {
    tags: IndexSet<String>,
    color: BTreeSet<WUBRG>,
    commander: BTreeSet<WUBRG>,
    name: Vec<Regex>,
    vname: Vec<Regex>,
    r#type: Vec<Regex>,
    vtype: Vec<Regex>,
    grep: Vec<Regex>,
    vgrep: Vec<Regex>,
    text: Vec<Regex>,
    vtext: Vec<Regex>,
    sideboard: bool,
    funnies: bool,
    debug: bool,
}

impl Searcher {
    fn new(it: Search) -> anyhow::Result<Self> {
        Ok(Self {
            tags: IndexSet::from_iter(it.tag.into_iter()),
            name: Self::build_regexes(it.case_sensitive, it.name)?,
            vname: Self::build_regexes(it.case_sensitive, it.vname)?,
            color: Self::build_color(it.color, WUBRG::colorless())?,
            commander: Self::build_color(it.commander, WUBRG::wubrg())?,
            r#type: Self::build_regexes(it.case_sensitive, it.r#type)?,
            vtype: Self::build_regexes(it.case_sensitive, it.vtype)?,
            grep: Self::build_regexes(it.case_sensitive, it.grep)?,
            vgrep: Self::build_regexes(it.case_sensitive, it.vgrep)?,
            text: Self::build_regexes(it.case_sensitive, it.text)?,
            vtext: Self::build_regexes(it.case_sensitive, it.vtext)?,
            debug: it.debug,
            sideboard: it.sideboard,
            funnies: it.funnies,
        })
    }

    fn match_proxies<'a>(&self, proxies: impl IntoIterator<Item = &'a Proxy>) -> Vec<&'a Proxy> {
        proxies
            .into_iter()
            .filter(|p| p.in_deck() != self.sideboard)
            .filter(|p| self.matches_proxy(p))
            .collvect()
    }

    fn matches_cardoids<'a>(
        &self,
        cardoids: impl IntoIterator<Item = &'a Cardoid>,
    ) -> Vec<&'a Cardoid> {
        cardoids
            .into_iter()
            .filter(|c| c.face().is_funny <= self.funnies)
            .filter(|c| match c.layout() {
                &CardLayout::Unsupported => self.funnies,
                _ => true,
            })
            .filter(|c| self.matches_cardoid(c))
            .collvect()
    }

    fn matches_proxy(&self, proxy: &Proxy) -> bool {
        self.tags.is_subset(&proxy.tags) && self.matches_cardoid(&proxy.cardoid)
    }

    fn matches_cardoid(&self, cardoid: &Cardoid) -> bool {
        cardoid.color_identity().is_subset(&self.commander)
            && Self::regex_match(&self.name, &self.vname, cardoid.name())
            && Self::regex_match(&self.grep, &self.vgrep, &format!("{}", cardoid))
            && cardoid.iter().any(|card| self.matches_card(card))
    }

    fn matches_card(&self, card: &Card) -> bool {
        self.color.is_subset(&card.colors)
            && Self::regex_match(&self.r#type, &self.vtype, &card.type_line)
            && Self::regex_match(&self.text, &self.vtext, &card.text)
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
            let pref = if case { "".s() } else { "(?i)".s() };
            res.push(Regex::new(&(pref + &s))?)
        }
        Ok(res)
    }

    fn regex_match(pos: &[Regex], neg: &[Regex], data: &str) -> bool {
        pos.iter().all(|r| r.is_match(data)) && !neg.iter().any(|r| r.is_match(data))
    }

    fn print_cardoid(&self, c: &Cardoid) {
        println!();
        if self.debug {
            println!("{:?}", c);
        } else {
            println!("{}", c);
        }
    }

    fn print_proxy(&self, p: &Proxy) {
        println!();
        if self.debug {
            println!("{:?}", p);
        } else {
            println!("{}", p);
        }
    }
}
