#![warn(unused)]

mod atomic_cards;
mod command;
mod html;
mod proxy;
mod rendering;
mod scryfall;
mod utils;

use std::path::Path;

use atomic_cards::AtomicCardsFile;
use clap::Parser;
use command::Command;

use crate::proxy::decklists::DeckList;

fn main() -> anyhow::Result<()> {
    let command = Command::parse();

    let atomic_cards = AtomicCardsFile::load_json(command.verbose)?;
    //atomic_cards.validate()?;

    let decklist_file = command.subcommand.decklist_file();
    let mut decklist = if decklist_file == Path::new("") {
        DeckList::new()
    } else {
        DeckList::load(decklist_file, &atomic_cards)?
    };

    command.subcommand.dispatch(&atomic_cards, &mut decklist)?;

    Ok(())
}
