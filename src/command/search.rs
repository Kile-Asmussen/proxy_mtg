use clap::Parser;
use regex::Regex;

use crate::{atomic_cards::AtomicCardsFile, proxy::decklist::DeckList};

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
    pub fn dispatch(&self, atomics: &AtomicCardsFile, decklist: &DeckList) {
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
