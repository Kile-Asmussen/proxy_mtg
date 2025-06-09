use regex::Regex;

use crate::{
    atomic_cards::{
        cards::Card,
        types::{FaceLayout, Side},
    },
    html::{Element, Tag},
    proxy::Proxy,
    rendering::{
        general::{
            card_art_img, empty_card, flavor_text_paragraph, get_side, raw_card, rules_text_line,
            rules_text_paragraph, type_line_div,
        },
        reminders::{NoReminderText, ReminderText},
    },
};

pub fn class_layout_card(proxy: &Proxy) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Class => class_card(card, proxy),
        _ => empty_card(card, proxy),
    }]
}

pub fn class_card(card: &Card, proxy: &Proxy) -> Element {
    raw_card(card, proxy).node(vertical_rules_text_div(card, proxy))
}

fn vertical_rules_text_div(card: &Card, proxy: &Proxy) -> Element {
    match card.face_layout() {
        FaceLayout::Class => class_rules_text_div(card, proxy),
        _ => Element::new(Tag::div).class(["rules-text"]),
    }
}

fn class_rules_text_div(card: &Card, proxy: &Proxy) -> Element {
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

    let reminders = if proxy.reminder_text {
        rules_text_line::<ReminderText>
    } else {
        rules_text_line::<NoReminderText>
    };

    let level_up = Regex::new(r"^.*?:\s+Level\s+\d\s*$").unwrap();

    let mut paragraphs = vec![];
    for line in text.lines() {
        if level_up.is_match(line) {
            paragraphs.push(Element::new(Tag::hr));
        }
        let rem = reminders(line);
        if !rem.is_empty() {
            paragraphs.push(rules_text_paragraph(reminders(line)))
        }
    }

    if let Some(t) = flavor_text {
        for line in t.lines() {
            paragraphs.push(flavor_text_paragraph([line]));
        }
    }

    Element::new(Tag::div)
        .class(["text-box", "dense", "vertical"])
        .nodes(paragraphs)
}
