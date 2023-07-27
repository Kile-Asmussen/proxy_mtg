mod cards;
mod decklist;
mod html_proxies;
mod proxy_builder;
mod simple_proxy;

use std::{
    error::Error,
    fmt::Write,
    path::{Path, PathBuf},
};

use cards::*;
use decklist::{Artoid, DeckEDH, Landoid};
use html_proxies::FirefoxFriendlyHtmlDeckList;
use proxy_builder::{BasicLand, CoreLand, DeckBuilder};
use simple_proxy::*;

use crate::{
    html_proxies::{NormalHtmlBuilder, SagaHtmlBuilder},
    proxy_builder::{ProxyBuilder, ProxyBuilderNormal, ProxyBuilderSaga},
};

fn main() {
    let deck = DeckEDH {
        commanders: vec![Artoid {
            name: "Henzie \"Toolbox\" Torre".into(),
            art_file: "./art/henzie-toolbox-torre.png".into(),
            art_credit: "Johannes Voss".into(),
            flavor_text: "".into(),
        }],
        the_99ish: vec![Artoid {
            name: "Lightning Bolt".into(),
            art_file: PathBuf::new(),
            art_credit: String::new(),
            flavor_text: String::new(),
        }],
        basics: vec![Landoid {
            name: BasicLand::Base(CoreLand::Mountain),
            number: 1,
            art_credit: String::new(),
            art_file: PathBuf::new(),
        }],
    };

    println!("{}", serde_json::to_string_pretty(&deck).unwrap(),)
}

fn main_2() {
    let mut deck = FirefoxFriendlyHtmlDeckList::new();

    let mut kiora = SagaHtmlBuilder::new();
    kiora
        .name("Kiora Bests the Sea God")
        .mana_cost("{7}{U}{U}")
        .art_filename(&Path::new("./art/kiora-bests-the-sea-god.png"))
        .art_credits("Victor Adame Minguez")
        .type_line("Enchantment &mdash; Saga")
        .step_text(
            &[1],
            "Create an 8/8 blue Kraken creature
        token with hexproof.",
        )
        .step_text(
            &[2],
            "Tap all nonland permanents target opponent controls. They don't untap during their controller's next untap step.",
        )
        .step_text(&[3], "Gain control of target permanent an opponent controls. Untap it.");

    for _ in 1..=36 {
        deck.add_card(kiora.build());
    }

    let mut out_file: Box<dyn std::io::Write> =
        Box::new(std::fs::File::create("./card_test.html").unwrap());

    deck.build(&mut out_file).unwrap();
}
