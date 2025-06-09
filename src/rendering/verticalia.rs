use regex::Regex;

use crate::{
    atomic_cards::{
        cards::Card,
        types::{FaceLayout, Side, Type},
    },
    html::{Element, Node, Tag},
    proxy::Proxy,
    rendering::{
        general::{
            empty_card, flavor_text_paragraphs, get_side, raw_card, rules_text_filter,
            rules_text_paragraph,
        },
        normal::power_toughness,
        parsing::{chapter_symbol, split_chapter_abilities},
    },
};

pub fn class_layout_proxy(proxy: &Proxy) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Class => class_card(card, proxy),
        _ => empty_card(card, proxy),
    }]
}

pub fn saga_layout_proxy(proxy: &Proxy) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Saga => saga_card(card, proxy),
        _ => empty_card(card, proxy),
    }]
}

pub fn class_card(card: &Card, proxy: &Proxy) -> Element {
    raw_card(card, proxy).node(vertical_rules_text_div(card, proxy))
}

pub fn saga_card(card: &Card, proxy: &Proxy) -> Element {
    let mut res = raw_card(card, proxy).node(vertical_rules_text_div(card, proxy));
    if card.types.contains(&Type::Creature) {
        res = res.node(power_toughness(card));
    }
    res
}

fn vertical_rules_text_div(card: &Card, proxy: &Proxy) -> Element {
    match card.face_layout() {
        FaceLayout::Class => class_rules_text_div(card, proxy),
        FaceLayout::Saga => saga_rules_text_div(card, proxy),
        _ => Element::new(Tag::div).class(["rules-text", "vertical"]),
    }
}

fn class_rules_text_div(card: &Card, proxy: &Proxy) -> Element {
    let mut text = card.text.clone();

    if let Some(c) = get_side(Side::A, &proxy.customize) {
        if !c.text.is_empty() {
            text = c.text.clone();
        }
    }

    let rules_text = rules_text_filter(proxy);

    let level_up = Regex::new(r"^.*?:\s+Level\s+\d\s*$").unwrap();

    let mut paragraphs = vec![];
    for line in text.lines() {
        if level_up.is_match(line) {
            if !paragraphs.is_empty() {
                paragraphs.push(Element::new(Tag::hr));
            }
            paragraphs.push(rules_text_paragraph(rules_text(line)).class(["center"]))
        } else {
            let line = rules_text(line);
            if !line.is_empty() {
                paragraphs.push(rules_text_paragraph(line))
            }
        }
    }

    paragraphs.append(&mut flavor_text_paragraphs(card, proxy));

    Element::new(Tag::div)
        .class(["text-box", "dense", "vertical"])
        .nodes(paragraphs)
}

fn saga_rules_text_div(card: &Card, proxy: &Proxy) -> Element {
    let mut text = card.text.clone();

    if let Some(c) = get_side(Side::A, &proxy.customize) {
        if !c.text.is_empty() {
            text = c.text.clone();
        }
    }

    let rules_text = rules_text_filter(proxy);

    let mut paragraphs = vec![];
    for line in text.lines() {
        let mut par = Vec::<Node>::new();
        let (chapters, line) = split_chapter_abilities(line);

        if !chapters.is_empty() {
            par.push(
                Element::new(Tag::p)
                    .class(["center"])
                    .nodes(chapters.into_iter().map(chapter_symbol).flatten())
                    .into(),
            );
        }

        par.append(&mut rules_text(line));

        if !paragraphs.is_empty() {
            paragraphs.push(Element::new(Tag::hr));
        }
        paragraphs.push(rules_text_paragraph(par));
    }

    paragraphs.append(&mut flavor_text_paragraphs(card, proxy));

    Element::new(Tag::div)
        .class(["text-box", "dense", "vertical"])
        .nodes(paragraphs)
}
