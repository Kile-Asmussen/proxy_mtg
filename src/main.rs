#![allow(unused)]

mod atomic_cards;
mod command;
mod decklist;

use std::{error::Error, ffi::OsStr, time::Instant};

use atomic_cards::*;
use clap::Parser;
use command::*;

use crate::decklist::DeckList;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let command = Command::parse();

    println!("Loading cards...");
    let start = Instant::now();
    let atomic_cards = AtomicCards::load()?;
    println!(
        "Read {} atomic cards in {} milliseconds",
        atomic_cards.data.len(),
        start.elapsed().as_millis()
    );
    let start = Instant::now();
    let decklist = DeckList::load(command.decklist_file(), &atomic_cards)?;
    let filename = command
        .decklist_file()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    println!(
        "Read {} card decklist `{}' in {} milliseconds",
        decklist.num_cards(),
        filename,
        start.elapsed().as_millis()
    );

    command.dispatch(&decklist);

    Ok(())
}
