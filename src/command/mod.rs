pub mod build;
pub mod list;
pub mod search;

use std::{fmt::Debug, path::PathBuf};

use clap::{builder::Str, Parser, Subcommand};
use rand::seq::SliceRandom;
use regex::Regex;

use crate::{atomic_cards::AtomicCardsFile, proxy::decklist::DeckList};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[arg(value_name = "FILE")]
    pub decklist: PathBuf,
    #[command(subcommand)]
    pub subcommand: ListBuildSearch,
}

#[derive(Subcommand, Debug)]
pub enum ListBuildSearch {
    List(list::List),
    Build(build::Build),
    Search(search::Search),
}

impl ListBuildSearch {
    pub fn dispatch(&self, atomics: &AtomicCardsFile, decklist: &DeckList) {
        match self {
            Self::List(l) => l.dispatch(decklist),
            Self::Build(b) => b.dispatch(decklist),
            Self::Search(s) => s.dispatch(atomics, decklist),
        }
    }
}
