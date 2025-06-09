use crate::{
    atomic_cards::{
        cards::Card,
        types::{FaceLayout, Side, Supertype, WUBRG},
    },
    html::{Element, Tag},
    proxy::Proxy,
    rendering::{
        general::{anchor_words, flavor_text_paragraph, rules_text_paragraph},
        reminders::{NoReminderText, ReminderText},
    },
    utils::{
        iter::IterExt,
        printers::{TextPrinter, ToText},
        symbolics::{replace_symbols, NothingReplacer},
    },
};

use super::general::{card_art_img, empty_card, flavor_text, get_side, type_line_div};

pub fn normal_layout_card(proxy: &Proxy) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Basic => basic_land(proxy),
        FaceLayout::Creature => creature_card(card, proxy),
        FaceLayout::Planeswalker => raw_card(card, proxy),
        FaceLayout::Unadorned => unadorned_card(card, proxy),
        _ => raw_card(card, proxy),
    }]
}

pub fn raw_card(card: &Card, proxy: &Proxy) -> Element {
    empty_card(card, proxy)
        .node(type_line_div(card, proxy))
        .nodes(card_art_img(card, proxy))
        .node(type_line_div(card, proxy))
}

pub fn basic_land(proxy: &Proxy) -> Element {
    let card = proxy.cardoid.face();
    raw_card(card, proxy).node(rules_text_basic_div(card, proxy))
}

pub fn unadorned_card(card: &Card, proxy: &Proxy) -> Element {
    raw_card(card, proxy).node(rules_text_div(card, proxy))
}

pub fn creature_card(card: &Card, proxy: &Proxy) -> Element {
    println!("{}", TextPrinter(&NothingReplacer, ToText::Card(card)));
    raw_card(card, proxy)
        .node(rules_text_div(card, proxy))
        .node(power_toughness(card))
}

pub fn power_toughness(card: &Card) -> Element {
    Element::new(Tag::div)
        .class(["bar", "corner-bubble"])
        .node(Element::new(Tag::span).node(format!("{}/{}", card.power, card.toughness)))
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

    let reminders = if proxy.reminder_text {
        anchor_words::<ReminderText>
    } else {
        anchor_words::<NoReminderText>
    };

    let mut paragraphs = text
        .lines()
        .map(|line| reminders(line))
        .filter(|line| !line.is_empty())
        .map(rules_text_paragraph)
        .collvect();

    if let Some(t) = flavor_text {
        paragraphs.push(Element::new(Tag::hr));
        for line in t.lines() {
            paragraphs.push(flavor_text_paragraph([line]));
        }
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

pub fn rules_text_basic_div(card: &Card, proxy: &Proxy) -> Element {
    let mut big_symbol = rules_text_paragraph([big_mana_glyph(
        format!("ms-{}", WUBRG::render(&card.color_identity)).to_lowercase(),
    )]);

    if card.is_supertype(Supertype::Snow) {
        big_symbol = big_symbol.node(big_mana_glyph("ms-s"));
    }

    let mut text = vec![big_symbol.into()];

    if proxy.reminder_text {
        text.push(rules_text_paragraph(
            replace_symbols(&ReminderText, &card.text).concat(),
        ))
    }

    text.append(
        &mut flavor_text(card, proxy)
            .into_iter()
            .map(Into::into)
            .collvect(),
    );

    return Element::new(Tag::div)
        .class(["text-box", "sparse"])
        .nodes(text);

    pub fn big_mana_glyph<S>(class: S) -> Element
    where
        S: AsRef<str>,
    {
        Element::new(Tag::i).class(["ms", class.as_ref(), "ms-4x"])
    }
}

pub fn rules_text_planeswalker_div(card: &Card, proxy: &Proxy) -> Element {
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
        anchor_words::<ReminderText>
    } else {
        anchor_words::<NoReminderText>
    };

    let mut paragraphs = vec![];

    for line in text.lines() {
        let line = reminders(line);
        if line.is_empty() {
            continue;
        }
    }

    if let Some(t) = flavor_text {
        paragraphs.push(Element::new(Tag::hr));
        for line in t.lines() {
            paragraphs.push(flavor_text_paragraph([line]));
        }
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
