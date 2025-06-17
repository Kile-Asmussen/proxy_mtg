use regex::Regex;

use crate::{
    atomic_cards::{
        cards::Card,
        types::{FaceLayout, Side, Type},
    },
    html::{Element, Node, Tag},
    proxy::{Customization, Proxy, TextStyle},
    rendering::{
        general::{
            anchor_words, empty_card, get_side, rules_text_filter, rules_text_paragraph,
            text_style, type_line_div,
        },
        normal::power_toughness,
        parsing::{chapter_symbol, split_anchor_word, split_chapter_abilities},
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

pub fn raw_card(card: &Card, proxy: &Proxy) -> Element {
    empty_card(card, proxy)
        .node(type_line_div(card, proxy))
        .nodes(card_art_img(card, proxy))
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
    let mut text = &card.text;

    if let Some(Customization {
        text: Some(ctext), ..
    }) = get_side(Side::A, &proxy.customize)
    {
        text = ctext;
    }

    let rules_text = rules_text_filter(proxy);

    let level_up = Regex::new(r"^(.*?):\s+(Level\s+\d)\s*$").unwrap();

    let mut paragraphs = vec![];
    for line in text.lines() {
        if let Some(c) = level_up.captures(line) {
            let mana = c.get(1).unwrap().as_str();
            let level = c.get(2).unwrap().as_str();
            paragraphs.push(
                rules_text_paragraph([
                    Element::new(Tag::span).nodes(rules_text(mana)),
                    Element::new(Tag::b).node(level),
                ])
                .class(["level-up"]),
            );
        } else {
            let mut par = vec![];
            let (words, line) = split_anchor_word(line);
            if !words.is_empty() {
                par.append(&mut anchor_words(words));
            }
            par.append(&mut rules_text(line));
            if !par.is_empty() {
                paragraphs.push(rules_text_paragraph(par))
            }
        }
    }

    Element::new(Tag::div)
        .class(["text-box", "dense", "vertical", "right"])
        .class(text_style(card, proxy, vec![TextStyle::SmallText]))
        .nodes(paragraphs)
}

fn saga_rules_text_div(card: &Card, proxy: &Proxy) -> Element {
    let mut text = &card.text;

    if let Some(Customization {
        text: Some(ctext), ..
    }) = get_side(card.side, &proxy.customize)
    {
        text = ctext;
    }

    let rules_text = rules_text_filter(proxy);

    let mut paragraphs = vec![];
    for line in text.lines() {
        let mut par = Vec::<Node>::new();
        let (chapters, line) = split_chapter_abilities(line);

        par.append(&mut rules_text(line));

        if !chapters.is_empty() {
            paragraphs.push(saga_chapter_indicator(chapters));
        }
        paragraphs.push(rules_text_paragraph(par));
    }

    Element::new(Tag::div)
        .class(["text-box", "vertical", "left"])
        .class(text_style(card, proxy, vec![TextStyle::SmallText]))
        .nodes(paragraphs)
}

fn saga_chapter_indicator(chapters: Vec<&str>) -> Element {
    Element::new(Tag::p)
        .class(["saga-chapter"])
        .nodes(chapters.into_iter().map(chapter_symbol).flatten())
}

pub fn card_art_img(card: &Card, proxy: &Proxy) -> Vec<Element> {
    let mut classes = vec!["art", "vertical"];

    let Some(art) = get_side(card.side, &proxy.arts) else {
        return vec![];
    };

    if card.face_layout() == FaceLayout::Saga {
        classes.push("right");
    } else {
        classes.push("left");
    }

    let mut res = vec![];

    if !art.url.is_empty() {
        res.push(Element::new(Tag::img).class(classes).attr("src", &art.url));
    }

    if !art.credit.is_empty() {
        res.push(
            Element::new(Tag::span)
                .class(["art-credits"])
                .node(&art.credit),
        );
    }

    res
}
