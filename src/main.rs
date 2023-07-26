mod cards;
mod html_proxies;
mod proxy_builder;
mod simple_proxy;

use cards::*;
use simple_proxy::*;

use crate::{
    html_proxies::NormalHtmlBuilder,
    proxy_builder::{ProxyBuilder, ProxyBuilderNormal},
};

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

    let mut builder = NormalHtmlBuilder::new();

    builder
        .name("One with Nothing")
        .mana_cost("{B}")
        .art_filename("../art/one-with-nothing.webp")
        .type_line("Instant")
        .rules_text("Discard your hand.")
        .flavor_text("When nothing remains, everything is equally possible.")
        .art_credits("John Doe");

    println!("{}", builder.build());
}
