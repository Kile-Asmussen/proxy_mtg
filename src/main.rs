mod cards;
mod decklist;
mod html_proxies;
mod proxy_builder;
mod simple_proxy;

use std::path::{Path, PathBuf};

use cards::*;
use decklist::{Artoid, DeckEDH, Landoid};
use html_proxies::FirefoxFriendlyHtmlDeckList;
use proxy_builder::{BasicLand, CoreLand, DeckBuilder};

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

    println!("{}", serde_json::to_string_pretty(&deck).unwrap(),);

    //main_2();
}

fn main_2() {
    let mut deck = FirefoxFriendlyHtmlDeckList::new();
    let mut cards = AtomicCards::load().unwrap();

    let raw_henzie = cards
        .data
        .get("Henzie \"Toolbox\" Torre")
        .unwrap()
        .0
        .first()
        .unwrap();

    let mut henzie = NormalHtmlBuilder::new();
    henzie
        .name(&raw_henzie.name)
        .mana_cost(&raw_henzie.mana_cost)
        .set_legendary(raw_henzie.supertypes.contains(&"Legendary".into()))
        .type_line(&raw_henzie.type_line)
        .rules_text(&raw_henzie.text)
        .art_credits("Johannes Voss")
        .art_filename(&Path::new("../art/henzie-toolbox-torre.png"))
        .corner_bubble(&format!("{}/{}", raw_henzie.power, raw_henzie.toughness));

    deck.add_card(henzie.build());

    let mut kiora = SagaHtmlBuilder::new();
    kiora
        .name("Kiora Bests the Sea God")
        .mana_cost("{5}{U}{U}")
        .art_filename(&Path::new("../art/kiora-bests-the-sea-god.png"))
        .art_credits("Victor Adame Minguez")
        .type_line("Enchantment &mdash; Saga")
        .include_reminder(true)
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

    for _ in 2..=9 {
        deck.add_card(kiora.build());
    }

    let mut out_file: Box<dyn std::io::Write> =
        Box::new(std::fs::File::create("./output/card_test.html").unwrap());

    deck.build(&mut out_file).unwrap();
}
