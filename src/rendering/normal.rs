use lazy_regex::regex;
use std::{collections::BTreeSet, ops::Div};

use crate::{
    atomic_cards::{cards::*, types::*},
    html::*,
    proxy::Proxy,
    rendering::manafont::ManaFontSymbolics,
    utils::{iter::IterExt, symbolics::replace_symbols},
};

use super::{general::*, RenderSettings};

pub fn normal_card(proxy: &Proxy, settings: &RenderSettings) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layouts() {
        FaceLayout::Basic => basic_land(proxy, settings),
        FaceLayout::Creature => creature_card(proxy, settings),
        FaceLayout::Planeswalker => raw_card(proxy, settings),
        FaceLayout::Unadorned => unadorned_card(proxy, settings),
        _ => raw_card(proxy, settings),
    }]
}

pub fn raw_card(proxy: &Proxy, settings: &RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    empty_card(card)
        .elem(title_bar_div(&card.name, &card.mana_cost))
        .elem(type_line_div(card))
        .elem(card_art_img(proxy))
        .elem(type_line_div(card))
}

pub fn basic_land(proxy: &Proxy, settings: &RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy, settings).elem(rules_text_basic_div(card))
}

pub fn unadorned_card(proxy: &Proxy, settings: &RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy, settings).elem(rules_text_div(card, settings))
}

pub fn creature_card(proxy: &Proxy, settings: &RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy, settings)
        .elem(rules_text_div(card, settings))
        .elem(power_toughness(card))
}

pub fn power_toughness(card: &Card) -> Element {
    Element::new(Tag::div)
        .class(["bar", "corner-bubble"])
        .elem(Element::new(Tag::span).text(format!("{}/{}", card.power, card.toughness)))
}

pub fn rules_text_basic_div(card: &Card) -> Element {
    Element::new(Tag::div).class(["text-box", "sparse"]).elem(
        Element::new(Tag::p)
            .class(["rules-text"])
            .elem(Element::new(Tag::i).class([
                format!("ms"),
                format!("ms-{}", WUBRG::render(&card.color_identity).to_lowercase()),
                format!("ms-6x"),
            ])),
    )
}

pub fn rules_text_div(card: &Card, settings: &RenderSettings) -> Element {
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
                .nodes(replace_symbols(&ManaFontSymbolics, line)),
        );
    }

    res
}
