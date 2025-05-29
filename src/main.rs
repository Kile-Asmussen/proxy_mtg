#![allow(unused)]

mod atomic_cards;
mod command;
mod html;
mod proxy;
mod vec_entry;

use std::{error::Error, ffi::OsStr, time::Instant};

use atomic_cards::*;
use clap::Parser;
use command::*;

use crate::proxy::DeckList;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let command = Command::parse();

    println!("Loading cards...");
    let start = Instant::now();
    let atomic_cards = AtomicCardsFile::load()?;
    println!(
        "Read {} atomic cards in {} milliseconds",
        atomic_cards.data.len(),
        start.elapsed().as_millis()
    );
    let decklist = DeckList::load(&command.decklist, &atomic_cards)?;
    let filename = command.decklist.file_name().unwrap().to_str().unwrap();

    command.subcommand.dispatch(&atomic_cards, &decklist);

    Ok(())
}
