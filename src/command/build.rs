use clap::Parser;
use std::path::PathBuf;

use crate::decklist::DeckList;

#[derive(Parser, Debug, Clone)]
pub struct Build {
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}

impl Build {
    pub fn dispatch(&self, decklist: &DeckList) {}
}
