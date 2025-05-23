mod atomic_cards;
mod decklist;
mod proxy_cards;

use std::fs::File;

use atomic_cards::*;
use decklist::*;
use serde::Deserialize;

fn main() {
    let atomic_cards_file = File::open("AtomicCards.pretty.json").unwrap();

    let mut atomic_cards_deserializer = serde_json::Deserializer::from_reader(atomic_cards_file);

    let atomic_cards = AtomicCards::deserialize(&mut atomic_cards_deserializer).unwrap();

    println!("Read {} atomic cards", atomic_cards.data.len());

    let decklist_file = File::open("decklists/oketra.json").unwrap();

    let mut decklist_deserializer = serde_json::Deserializer::from_reader(decklist_file);

    let decklist = DeckList::deserialize(&mut decklist_deserializer).unwrap();

    println!("Read decklist:");

    for (section, cards) in &decklist.0 {
        println!("  Section {}", section);
        for card in cards {
            println!(
                "    {} x {}",
                match card.repeats {
                    0 => 1,
                    x => x,
                },
                card.name
            );
        }
    }
}
