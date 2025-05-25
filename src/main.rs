mod atomic_cards;
mod command;
mod decklist;
mod proxy_cards;

use std::{collections::HashSet, fs::File, time::Instant};

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

    for (section, artoids) in &decklist.0 {
        println!("  {}:", section);
        for artoid in artoids {
            println!("    {} x {}", artoid.repeats, artoid.name);
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

    let mut color_id = HashSet::<String>::new();
    for (section, artoids) in &decklist.0 {
        for artoid in artoids {
            if let Some(cardoid) = &artoid.cardoid {
                for card in &cardoid.0 {
                    for color in &card.color_identity {
                        color_id.insert(color.clone());
                    }
                }
            }
        }
    }
    let mut color_id = color_id.into_iter().collect::<Vec<_>>();

    color_id.sort_by_key(|c| match &c[..] {
        "W" => 1,
        "U" => 2,
        "B" => 3,
        "R" => 4,
        "G" => 5,
        _ => 10,
    });

    println!("Color identity: {}", color_id.join(""))
}
