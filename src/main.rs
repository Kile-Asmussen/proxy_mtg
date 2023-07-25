mod cards;
mod proxy;

use cards::*;
use proxy::*;

fn main() {
    print!("Loading... ");
    let atomic_cards = AtomicCards::load().unwrap();
    println!("Done!");

    let deck = &[
        "Lightning Bolt",
        "One with Nothing",
        "Gifts Ungiven",
        "Avacyn, Angel of Hope",
        "Llanowar Elves",
        "Emrakul, the Aeons Torn",
    ][..];

    for name in deck {
        println!(
            "{}",
            DiscordTemplate
                .proxy(name, &atomic_cards)
                .unwrap_or_else(|| format!("--- {} not found ---", name))
        );
        println!()
    }
}
