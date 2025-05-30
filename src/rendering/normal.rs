use std::{collections::BTreeSet, ops::Div};

use lazy_regex::regex;

use crate::{
    atomic_cards::{cards::*, types::*},
    html::*,
    proxy::Proxy,
    utils::iter::IterExt,
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

pub fn rules_text_div(card: &Card, settings: &RenderSettings) -> Element {
    if card.is_basic() {
        rules_text_basic_div(card, settings)
    } else {
        rules_text_nonland_div(card, settings)
    }
}

pub fn rules_text_basic_div(card: &Card, settings: &RenderSettings) -> Element {
    todo!()
}

pub fn rules_text_nonland_div(card: &Card, settings: &RenderSettings) -> Element {
    let mut text = card.text.clone();

    if !settings.reminder_text {
        text = regex!(r"\([^\n]+\)").replace_all(&text, "").into_owned();
    }

    let text_len = text.len();

    let paragraphs = text.lines().map(ToOwned::to_owned).collvect();

    let class: &[&str] = if paragraphs.len() == 1 && text_len < 50 {
        &["text-box", "sparse"]
    } else if paragraphs.len() >= 3 || text_len >= 200 {
        &["text-box", "dense"]
    } else {
        &["text-box"]
    };

    let mut res = Element::new(Tag::div).class(class);

    for line in text.lines() {
        res = res.elem(
            Element::new(Tag::p)
                .class(["rules-text"])
                .text(line.to_string()),
        );
    }

    res
}
