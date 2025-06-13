use crate::{
    atomic_cards::{
        cards::Card,
        types::{FaceLayout, Side, Supertype, WUBRG},
    },
    html::{Element, Node, Tag},
    proxy::Proxy,
    rendering::{
        general::{anchor_words, corner_bubble, raw_card, rules_text_filter, rules_text_paragraph},
        parsing::{loyalty_symbol, split_anchor_word, split_loyalty_ability},
        reminders::ReminderText,
    },
    utils::symbolics::replace_symbols,
};

use super::general::{flavor_text_paragraphs, get_side};

pub fn normal_layout_proxy(proxy: &Proxy) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Basic => basic_land(card, proxy),
        FaceLayout::Creature => creature_card(card, proxy),
        FaceLayout::Planeswalker => planeswalker_card(card, proxy),
        FaceLayout::Unadorned => unadorned_card(card, proxy),
        _ => raw_card(card, proxy),
    }]
}

pub fn basic_land(card: &Card, proxy: &Proxy) -> Element {
    raw_card(card, proxy).node(rules_text_basic_div(card, proxy))
}

pub fn unadorned_card(card: &Card, proxy: &Proxy) -> Element {
    raw_card(card, proxy).node(rules_text_normal_div(card, proxy))
}

pub fn creature_card(card: &Card, proxy: &Proxy) -> Element {
    raw_card(card, proxy)
        .node(rules_text_normal_div(card, proxy))
        .node(power_toughness(card))
}

pub fn planeswalker_card(card: &Card, proxy: &Proxy) -> Element {
    raw_card(card, proxy)
        .node(rules_text_planeswalker_div(card, proxy))
        .node(planeswalker_loyalty(card))
}

pub fn rules_text_normal_div(card: &Card, proxy: &Proxy) -> Element {
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
        let (words, line) = split_anchor_word(line);

        if !words.is_empty() {
            par.append(&mut anchor_words(words))
        }

        par.append(&mut rules_text(line));

        paragraphs.push(rules_text_paragraph(par));
    }

    let centered = get_side(card.side, &proxy.arts)
        .map(|a| a.center_text)
        .unwrap_or(false);

    let text_len: usize = paragraphs.iter().map(|n| n.text_len()).sum();
    let num_paragraphs = paragraphs.len();

    let mut flavor = flavor_text_paragraphs(card, proxy);
    if !paragraphs.is_empty() && !flavor.is_empty() {
        paragraphs.push(Element::new(Tag::hr));
    }
    paragraphs.append(&mut flavor);

    let text_class: &[&str] = if text_len >= 350 {
        &["compact"]
    } else if num_paragraphs >= 5 || text_len >= 200 {
        &["dense"]
    } else if centered {
        &["sparse"]
    } else {
        &[]
    };

    Element::new(Tag::div)
        .class(["text-box"])
        .class(text_class)
        .nodes(paragraphs)
}

pub fn rules_text_planeswalker_div(card: &Card, proxy: &Proxy) -> Element {
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
        let (loyalty, line) = split_loyalty_ability(line);

        if let Some(l) = loyalty_symbol(loyalty) {
            par.push(l.into())
        }

        let (words, line) = split_anchor_word(line);

        if !words.is_empty() {
            par.append(&mut anchor_words(words))
        }

        par.append(&mut rules_text(line));

        if !paragraphs.is_empty() {
            paragraphs.push(Element::new(Tag::hr));
        }
        paragraphs.push(rules_text_paragraph(par));
    }

    let text_len: usize = paragraphs.iter().map(|n| n.text_len()).sum();

    let text_class: &[&str] = if text_len >= 350 {
        &["compact"]
    } else if text_len >= 200 {
        &["dense"]
    } else {
        &[]
    };

    Element::new(Tag::div)
        .class(["text-box"])
        .class(text_class)
        .nodes(paragraphs)
}

pub fn rules_text_basic_div(card: &Card, proxy: &Proxy) -> Element {
    let mut big_symbol = rules_text_paragraph([big_mana_glyph(
        format!("ms-{}", WUBRG::render(&card.color_identity)).to_lowercase(),
    )]);

    if card.is_supertype(Supertype::Snow) {
        big_symbol = big_symbol.node(big_mana_glyph("ms-s"));
    }

    let mut paragraphs = vec![big_symbol.into()];

    if proxy.reminder_text {
        paragraphs.push(rules_text_paragraph(
            replace_symbols::<ReminderText>(&card.text).concat(),
        ))
    }

    paragraphs.append(&mut flavor_text_paragraphs(card, proxy));

    return Element::new(Tag::div)
        .class(["text-box", "sparse"])
        .nodes(paragraphs);

    pub fn big_mana_glyph<S>(class: S) -> Element
    where
        S: AsRef<str>,
    {
        Element::new(Tag::i).class(["ms", class.as_ref(), "ms-6x"])
    }
}

pub fn power_toughness(card: &Card) -> Element {
    corner_bubble(format!("{}/{}", card.power, card.toughness))
}

pub fn planeswalker_loyalty(card: &Card) -> Element {
    corner_bubble(&card.loyalty).class(["shield"])
}
