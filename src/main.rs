#![allow(unused)]

mod atomic_cards;
mod command;
mod html;
mod proxy;
mod rendering;
mod utils;

use std::{error::Error, ffi::OsStr, time::Instant};

use atomic_cards::*;
use clap::Parser;
use command::*;

use crate::proxy::decklist::DeckList;

fn main() -> anyhow::Result<()> {
    let command = Command::parse();

    let atomic_cards = AtomicCardsFile::load()?;

    let decklist = DeckList::load(&command.decklist, &atomic_cards)?;

    command.subcommand.dispatch(&atomic_cards, &decklist);

    Ok(())
}
