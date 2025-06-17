use crate::{
    atomic_cards::cards::Card,
    html::Element,
    proxy::Proxy,
    rendering::{
        general::{corner_bubble, empty_card, type_line_div},
        normal::{self},
    },
};

pub fn battle_card(card: &Card, proxy: &Proxy) -> Element {
    empty_card(card, proxy)
        .node(type_line_div(card, proxy))
        .nodes(normal::card_art_img(card, proxy))
        .node(normal::rules_text_normal_div(card, proxy))
        .node(corner_bubble(&card.defense).class(["square"]))
}
