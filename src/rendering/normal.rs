use crate::{
    atomic_cards::{
        cards::Card,
        types::{FaceLayout, Side, Supertype, WUBRG},
    },
    html::*,
    proxy::Proxy,
    rendering::reminders::{NoReminderText, ReminderText},
    utils::{iter::IterExt, symbolics::replace_symbols},
};

use super::general::{card_art_img, empty_card, flavor_text, get_side, type_line_div};

pub fn normal_card(proxy: &Proxy) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Basic => basic_land(proxy),
        FaceLayout::Creature => creature_card(proxy),
        FaceLayout::Planeswalker => raw_card(proxy),
        FaceLayout::Unadorned => unadorned_card(proxy),
        _ => raw_card(proxy),
    }]
}

pub fn raw_card(proxy: &Proxy) -> Element {
    let card = proxy.cardoid.face();

    empty_card(card, proxy)
        .node(type_line_div(card, proxy))
        .nodes(card_art_img(card, proxy))
        .node(type_line_div(card, proxy))
}

pub fn basic_land(proxy: &Proxy) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy).node(rules_text_basic_div(card, proxy))
}

pub fn unadorned_card(proxy: &Proxy) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy).node(rules_text_div(card, proxy))
}

pub fn creature_card(proxy: &Proxy) -> Element {
    let card = proxy.cardoid.face();
    raw_card(proxy)
        .node(rules_text_div(card, proxy))
        .node(power_toughness(card))
}

pub fn power_toughness(card: &Card) -> Element {
    Element::new(Tag::div)
        .class(["bar", "corner-bubble"])
        .node(Element::new(Tag::span).node(format!("{}/{}", card.power, card.toughness)))
}

pub fn rules_text_basic_div(card: &Card, proxy: &Proxy) -> Element {
    if proxy.reminder_text {
        return rules_text_div(card, proxy);
    }

    let mut text = Element::new(Tag::p)
        .class(["rules-text"])
        .node(big_mana_glyph(
            format!("ms-{}", WUBRG::render(&card.color_identity)).to_lowercase(),
        ));

    if card.is_supertype(Supertype::Snow) {
        text = text.node(big_mana_glyph("ms-s"));
    }

    let mut text = vec![text];

    text.append(&mut flavor_text(card, proxy));

    Element::new(Tag::div)
        .class(["text-box", "sparse"])
        .nodes(text)
}

pub fn big_mana_glyph<S>(class: S) -> Element
where
    S: AsRef<str>,
{
    Element::new(Tag::i).class(["ms", class.as_ref(), "ms-4x"])
}

pub fn rules_text_div(card: &Card, proxy: &Proxy) -> Element {
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
        paragraphs.push(Element::new(Tag::p).class(["flavor-text"]).node(t));
    }

    let text_len: usize = paragraphs.iter().map(|n| n.text_len()).sum();

    let class: &[&str] = if paragraphs.len() == 1 && text_len < 50 {
        &["text-box", "sparse"]
    } else if paragraphs.len() >= 4 || text_len >= 180 {
        &["text-box", "dense"]
    } else {
        &["text-box"]
    };

    Element::new(Tag::div).class(class).nodes(paragraphs)
}
