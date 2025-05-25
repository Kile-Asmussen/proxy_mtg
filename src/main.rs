mod atomic_cards;
mod decklist;
mod proxy_cards;

use std::{fs::File, time::Instant};

use atomic_cards::*;
use decklist::*;
use serde::Deserialize;

fn main() {
    println!("Loading atomic cards...");
    let start = Instant::now();
    let atomic_cards = AtomicCards::load().unwrap();

    println!(
        "Read {} atomic cards in {} seconds",
        atomic_cards.data.len(),
        start.elapsed().as_secs()
    );

    let decklist_file = File::open("decklists/oketra.json").unwrap();

    let mut decklist_deserializer = serde_json::Deserializer::from_reader(decklist_file);

    let mut decklist = DeckList::deserialize(&mut decklist_deserializer).unwrap();

    println!("Read decklist:");

    for (section, cards) in &decklist.0 {
        println!("  {}:", section);
        for card in cards {
            println!("    {} x {}", card.repeats, card.name);
        }
    }

    let tag_histogram = decklist.tag_histogram();

    if tag_histogram.is_empty() {
        println!("No tags.")
    } else {
        println!("Tags:");
        for (tag, count) in decklist.tag_histogram() {
            println!("    {} x {}", count, tag)
        }
    }

    if let Err(misses) = decklist.build(&atomic_cards) {
        println!("Following cards were not found:");
        for card in misses {
            println!("  {}", card);
        }
    } else {
        println!("All cards successfully loaded from database.")
    }
}
