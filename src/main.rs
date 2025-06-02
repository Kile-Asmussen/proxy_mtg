#![allow(unused)]

mod atomic_cards;
mod command;
mod html;
mod proxy;
mod rendering;
mod utils;

use std::{error::Error, ffi::OsStr, path::Path, time::Instant};

use atomic_cards::*;
use clap::Parser;
use command::*;

use crate::proxy::decklists::DeckList;

fn main() -> anyhow::Result<()> {
    let command = Command::parse();

    let atomic_cards = AtomicCardsFile::load(command.verbose)?;

    let decklist_file = command.subcommand.decklist_file();
    let decklist = if decklist_file == Path::new("") {
        DeckList::new()
    } else {
        DeckList::load(decklist_file, &atomic_cards)?
    };

    command.subcommand.dispatch(&atomic_cards, &decklist);

    Ok(())
}
