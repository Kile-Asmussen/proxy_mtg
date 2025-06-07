use lazy_regex::regex;
use reqwest::blocking::get;
use std::{
    collections::BTreeSet,
    mem::replace,
    ops::{Div, Rem},
};

use crate::{
    atomic_cards::{cards::*, types::*},
    html::*,
    proxy::Proxy,
    rendering::{
        manafont::ManaFontSymbolics,
        reminders::{NoReminderText, ReminderText},
    },
    utils::{
        iter::IterExt,
        symbolics::{replace_symbols, RulesTextSymbolReplacer},
    },
};

use super::{general::*, RenderSettings};

pub fn normal_card(proxy: &Proxy, settings: RenderSettings) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Basic => basic_land(proxy, settings),
        FaceLayout::Creature => creature_card(proxy, settings),
        FaceLayout::Planeswalker => raw_card(proxy, settings),
        FaceLayout::Unadorned => unadorned_card(proxy, settings),
        _ => raw_card(proxy, settings),
    }]
}

pub fn raw_card(proxy: &Proxy, settings: RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    let mut name = card.name.clone();

    if let Some(c) = proxy.customize.get(0) {
        if !c.name.is_empty() {
            name = c.name.clone();
        }
    }

    empty_card(card)
        .elem(title_bar_div(card, proxy))
        .elem(type_line_div(card, Side::A, proxy))
        .nodes(card_art_img(proxy, Side::A))
        .elem(type_line_div(card, Side::A, proxy))
}

pub fn basic_land(proxy: &Proxy, settings: RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy, settings).elem(rules_text_basic_div(card, proxy))
}

pub fn unadorned_card(proxy: &Proxy, settings: RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy, settings).elem(rules_text_div(card, proxy, settings))
}

pub fn creature_card(proxy: &Proxy, settings: RenderSettings) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy, settings)
        .elem(rules_text_div(card, proxy, settings))
        .elem(power_toughness(card))
}

pub fn power_toughness(card: &Card) -> Element {
    Element::new(Tag::div)
        .class(["bar", "corner-bubble"])
        .elem(Element::new(Tag::span).text(format!("{}/{}", card.power, card.toughness)))
}

pub fn rules_text_basic_div(card: &Card, proxy: &Proxy) -> Element {
    let mut text = Element::new(Tag::p)
        .class(["rules-text"])
        .elem(Element::new(Tag::i).class([
            format!("ms"),
            format!("ms-{}", WUBRG::render(&card.color_identity).to_lowercase()),
            format!("ms-4x"),
        ]));

    if card.is_supertype(Supertype::Snow) {
        text = text.elem(Element::new(Tag::i).class(["ms", "ms-s", "ms-4x"]));
    }

    Element::new(Tag::div)
        .class(["text-box", "sparse"])
        .elem(text)
}

pub fn rules_text_div(card: &Card, proxy: &Proxy, settings: RenderSettings) -> Element {
    let mut text = card.text.clone();
    let mut flavor_text = None;

    if let Some(c) = get_side(Side::A, &proxy.customize) {
        if !c.text.is_empty() {
            text = c.text.clone();
        }
        if !c.flavor_text.is_empty() {
            flavor_text = Some(c.flavor_text.clone());
        }
    }

    fn with_reminders(text: &str) -> Vec<Node> {
        replace_symbols(&ReminderText, text).concat()
    }

    fn without_reminders(text: &str) -> Vec<Node> {
        replace_symbols(&NoReminderText, text).concat()
    }

    let reminders = if proxy.reminder_text {
        with_reminders
    } else {
        without_reminders
    };

    let mut paragraphs = text
        .lines()
        .map(|line| {
            Element::new(Tag::p)
                .class(["rules-text"])
                .nodes(reminders(line))
        })
        .collvect();

    if let Some(t) = flavor_text {
        paragraphs.push(Element::new(Tag::p).class(["flavor-text"]).text(t));
    }

    let text_len: usize = paragraphs.iter().map(|n| n.text_len()).sum();

    let class: &[&str] = if paragraphs.len() == 1 && text_len < 50 {
        &["text-box", "sparse"]
    } else if paragraphs.len() >= 3 || text_len >= 200 {
        &["text-box", "dense"]
    } else {
        &["text-box"]
    };

    Element::new(Tag::div)
        .class(class)
        .nodes(paragraphs.into_iter().map(Node::Element))
}
