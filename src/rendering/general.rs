use regex::Regex;

use crate::{
    atomic_cards::{cards::Card, metadata::ForeignData, types::*},
    html::*,
    proxy::{Art, Proxy},
    rendering::manafont::ManaFontSymbolics,
    utils::{
        iter::IterExt,
        symbolics::{replace_symbols, RulesTextSymbolReplacer},
    },
};

pub fn blank_card() -> Element {
    Element::new(Tag::div).class(["card"])
}

pub fn empty_card(card: &Card, proxy: &Proxy) -> Element {
    blank_card()
        .class(card_css_class(card))
        .node(title_bar_div(card, proxy))
}

pub fn title_bar_div(card: &Card, proxy: &Proxy) -> Element {
    Element::new(Tag::div)
        .class(["title", "bar"])
        .node(card_name_span(card, proxy))
        .node(mana_cost_span(card))
}

pub fn mana_cost_span(card: &Card) -> Element {
    Element::new(Tag::span)
        .class(["cost"])
        .nodes(replace_symbols(&ManaFontSymbolics, &card.mana_cost))
}

pub fn rules_text_paragraph<NS, N>(text: NS) -> Element
where
    NS: IntoIterator<Item = N>,
    N: Into<Node>,
{
    Element::new(Tag::p).class(["rules-text"]).nodes(text)
}

pub fn flavor_text_paragraph<NS, N>(text: NS) -> Element
where
    NS: IntoIterator<Item = N>,
    N: Into<Node>,
{
    Element::new(Tag::p).class(["flavor-text"]).nodes(text)
}

pub fn card_name_span(card: &Card, proxy: &Proxy) -> Element {
    let mut name = card.face_name.clone();
    if name.is_empty() {
        name = card.name.clone();
    }

    if let Some(c) = get_side(card.side, &proxy.customize) {
        if !c.name.is_empty() {
            name = c.name.clone();
        } else if !c.face_name.is_empty() {
            name = c.name.clone();
        }
    }

    Element::new(Tag::span).class(["name"]).node(name)
}

pub fn card_art_img(card: &Card, proxy: &Proxy) -> Vec<Node> {
    if let Some(art) = get_side(card.side, &proxy.arts) {
        let mut classes = vec!["art"];
        if art.full {
            classes.push("full-art");
        }
        if card.face_layout().is_vertical() {
            classes.push("vertical")
        }

        vec![
            Node::Element(Element::new(Tag::img).class(classes).attr("src", &art.url)),
            Node::Element(
                Element::new(Tag::span)
                    .class(["art-credits"])
                    .node(&art.credit),
            ),
        ]
    } else {
        vec![]
    }
}

pub fn type_line_div(card: &Card, proxy: &Proxy) -> Element {
    let mut classes = vec!["type-line", "bar"];

    if card.face_layout().is_vertical() {
        classes.push("bottom");
    }

    if let Some(Art { full: true, .. }) = get_side(card.side, &proxy.arts) {
        classes.push("bottom");
    }

    Element::new(Tag::div)
        .class(classes)
        .node(type_line_span(card, proxy))
}

pub fn type_line_span(card: &Card, proxy: &Proxy) -> Element {
    let mut type_line = &card.type_line;

    if let Some(c) = get_side(card.side, &proxy.customize) {
        if !c.type_line.is_empty() {
            type_line = &c.type_line;
        }
    }

    Element::new(Tag::span).class(["type"]).node(type_line)
}

pub fn card_css_class(card: &Card) -> Vec<&'static str> {
    if card.types.contains(&Type::Land) {
        card.color_identity
            .iter()
            .map(WUBRG::name)
            .chain(["colorless"])
            .collvect()
    } else {
        card.colors.iter().map(WUBRG::name).collvect()
    }
}

pub fn flavor_text(card: &Card, proxy: &Proxy) -> Vec<Element> {
    let Some(ForeignData { flavor_text, .. }) = get_side(card.side, &proxy.customize) else {
        return vec![];
    };

    flavor_text
        .lines()
        .map(|s| Element::new(Tag::p).class(["flavor-text"]).node(s))
        .collvect()
}

pub fn get_side<T>(side: Side, v: &Vec<T>) -> Option<&T> {
    match side {
        Side::A => v.get(0),
        Side::B => v.get(1),
        _ => None,
    }
}

pub fn cost_symbol<S>(class: S) -> Element
where
    S: AsRef<str>,
{
    Element::new(Tag::i).class(["ms", class.as_ref(), "ms-cost", "ms-shadow"])
}

pub fn anchor_words<RT>(mut text: &str) -> Vec<Node>
where
    RT: RulesTextSymbolReplacer<Item = Vec<Node>> + Default,
{
    let mut res = vec![];
    let flavor_word = Regex::new(r"^(.*)\s+—\s+").unwrap();
    if let Some(m) = flavor_word.captures(text).and_then(|c| c.get(1)) {
        if m.as_str() != "Companion" {
            res.push(Element::new(Tag::em).node(m.as_str()).into());
            text = &text[m.end()..]
        }
    }
    res.append(&mut replace_symbols::<RT>(&Default::default(), text).concat());
    res
}
