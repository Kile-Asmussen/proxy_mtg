use clap::Parser;
use regex::Regex;

use anyhow::anyhow;

use crate::{atomic_cards::AtomicCardsFile, proxy::decklists::DeckList};

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
    pub fn dispatch(&self, atomics: &AtomicCardsFile, decklist: &DeckList) -> anyhow::Result<()> {
        let mut exp = vec![];
        for pat in &self.exp {
            exp.push(Regex::new(&("(?i)".to_string() + pat))?);
        }

        let mut nexp = vec![];
        for pat in &self.nexp {
            nexp.push(Regex::new(&("(?i)".to_string() + pat))?);
        }

        let mut cards = self.cards.clone();
        if cards.is_empty() && self.search_all {
            cards.append(&mut atomics.data.keys().map(Clone::clone).collect());
        } else if cards.is_empty() {
            for proxy in decklist {
                if !proxy.token {
                    cards.push(proxy.name.clone());
                }
            }
        }

        for card in &cards {
            let artoid = decklist.iter().find(|x| &x.name == card);

            if let Some(artoid) = artoid {
                if self.tag.iter().any(|t| !artoid.tags.contains(t)) {
                    continue;
                }
            }

            let cardoid = artoid
                .map(|a| &a.cardoid)
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
                return Err(anyhow!("No such card as `{}'", card));
            }
        }

        Ok(())
    }
}
