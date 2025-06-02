pub mod build;
pub mod list;
pub mod search;

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use clap::{builder::Str, Parser, Subcommand};
use rand::seq::SliceRandom;
use regex::Regex;

use crate::{atomic_cards::AtomicCardsFile, proxy::decklists::DeckList};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub subcommand: ListBuildSearch,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum ListBuildSearch {
    List(list::List),
    Build(build::Build),
    Search(search::Search),
}

impl ListBuildSearch {
    pub fn decklist_file(&self) -> &Path {
        match self {
            ListBuildSearch::List(list) => list.decklist_file(),
            ListBuildSearch::Build(build) => build.decklist_file(),
            ListBuildSearch::Search(search) => search.decklist_file(),
        }
    }

    pub fn dispatch(self, atomics: &AtomicCardsFile, decklist: &DeckList) -> anyhow::Result<()> {
        match self {
            Self::List(l) => l.dispatch(decklist),
            Self::Build(b) => b.dispatch(decklist),
            Self::Search(s) => s.dispatch(atomics, decklist),
        }
    }
}
