use std::{collections::BTreeSet, ops::Div};

use crate::{
    atomic_cards::{cards::*, types::*},
    html::Element,
    proxy::Proxy,
};

use super::{general::*, RenderSettings};

pub fn normal_card(proxy: &Proxy, settings: &RenderSettings) -> Element {
    let card = proxy.cardoid.face();

    empty_card(card)
        .elem(title_bar_div(&card.name, &card.mana_cost))
        .elem(type_line_div(card))
        .elem(card_art_img(proxy))
        .elem(type_line_div(card))
        .elem(rules_text_div(card, settings))
}
