pub mod build;
pub mod list;
pub mod search;
pub mod setup;

use std::{fmt::Debug, path::Path};

use clap::{Parser, Subcommand};

use crate::{atomic_cards::AtomicCardsFile, proxy::decklists::DeckList};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub subcommand: ListBuildSearch,
    #[arg(short, long)]
    pub verbose: bool,
}

pub struct Context {
    pub atomics: AtomicCardsFile,
    pub decklist: DeckList,
}

impl Context {
    pub fn load_atomics(&mut self) -> anyhow::Result<()> {}
    pub fn load_decklist(&mut self, path: &Path) -> anyhow::Result<()> {}
}

#[derive(Subcommand, Debug)]
pub enum ListBuildSearch {
    List(list::List),
    Build(build::Build),
    Search(search::Search),
    Setup(setup::Setup),
}

impl ListBuildSearch {
    pub fn decklist_file(&self) -> &Path {
        match self {
            ListBuildSearch::List(list) => list.decklist_file(),
            ListBuildSearch::Build(build) => build.decklist_file(),
            ListBuildSearch::Search(search) => search.decklist_file(),
            ListBuildSearch::Setup(setup) => setup.decklist_file(),
        }
    }

    pub fn dispatch(
        self,
        atomics: &AtomicCardsFile,
        decklist: &mut DeckList,
    ) -> anyhow::Result<()> {
        match self {
            Self::List(l) => l.dispatch(decklist),
            Self::Build(b) => b.dispatch(decklist),
            Self::Search(s) => s.dispatch(atomics, decklist),
            Self::Setup(s) => s.dispatch(),
        }
    }
}
