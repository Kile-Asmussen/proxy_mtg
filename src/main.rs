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
    println!();
    println!();

    let mut builder = NormalHtmlBuilder::new();

    builder
        .name("Squirrel")
        .type_line("Creature &mdash; Squirrel")
        .color_indicator("{G}}")
        .rules_text("Token.")
        .corner_bubble("1/1")
        .art_credits("John Doe");

    println!("{}", builder.build());
}
