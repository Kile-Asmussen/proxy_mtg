use std::collections::BTreeSet;

use lazy_regex::regex;
use serde_json::ser::CharEscape;

use crate::{
    atomic_cards::{cards::Card, types::*},
    html::*,
    proxy::{Art, Proxy},
    rendering::{self, manafont::ManaFontSymbolics},
    utils::{iter::IterExt, symbolics::replace_symbols},
};

use super::RenderSettings;

pub fn empty_card(card: &Card) -> Element {
    Element::new(Tag::div).class(card_css_class(card))
}

pub fn title_bar_div(card: &Card, proxy: &Proxy) -> Element {
    Element::new(Tag::div)
        .class(["title", "bar"])
        .elem(card_name_span(card, proxy))
        .elem(mana_cost_span(card))
}

pub fn mana_cost_span(card: &Card) -> Element {
    Element::new(Tag::span)
        .class(["cost"])
        .nodes(replace_symbols(&ManaFontSymbolics, &card.mana_cost))
}

pub fn card_name_span(card: &Card, proxy: &Proxy) -> Element {
    let mut name = card.name.clone();

    if let Some(c) = proxy.customize.get(0) {
        if !c.name.is_empty() {
            name = c.name.clone();
        } else if !c.face_name.is_empty() {
            name = c.name.clone();
        }
    }

    Element::new(Tag::span).class(["name"]).text(name)
}

pub fn card_art_img(proxy: &Proxy, side: Side) -> Vec<Node> {
    if let Some(art) = get_side(side, &proxy.arts) {
        vec![
            Node::Element(
                Element::new(Tag::img)
                    .class(if art.full {
                        vec!["art", "full-art"]
                    } else {
                        vec!["art"]
                    })
                    .attr("src", &art.url),
            ),
            Node::Element(
                Element::new(Tag::span)
                    .class(["art-credits"])
                    .text(&art.credit),
            ),
        ]
    } else {
        vec![]
    }
}

pub fn type_line_div(card: &Card, side: Side, proxy: &Proxy) -> Element {
    let on_bottom = match card.face_layout() {
        FaceLayout::SagaCreature | FaceLayout::Saga | FaceLayout::Case | FaceLayout::Class => true,
        _ => false,
    } || get_side(side, &proxy.arts).map(|a| a.full).unwrap_or(false);

    Element::new(Tag::div)
        .class(if on_bottom {
            vec!["type-line", "bar", "bottom"]
        } else {
            vec!["type-line", "bar"]
        })
        .elem(type_line_span(card, side, proxy))
}

pub fn type_line_span(card: &Card, side: Side, proxy: &Proxy) -> Element {
    let mut type_line = &card.type_line;

    if let Some(c) = get_side(side, &proxy.customize) {
        if !c.type_line.is_empty() {
            type_line = &c.type_line;
        }
    }

    Element::new(Tag::span).class(["type"]).text(type_line)
}

pub fn card_css_class(card: &Card) -> Vec<&'static str> {
    let (colors, extra) = if card.types.contains(&Type::Land) {
        (&card.color_identity, vec!["colorless", "card"])
    } else {
        (&card.colors, vec!["card"])
    };

    return colors
        .iter()
        .map(WUBRG::name)
        .chain(extra.into_iter())
        .collvect();
}

pub fn get_side<T>(side: Side, v: &Vec<T>) -> Option<&T> {
    match side {
        Side::A => v.get(0),
        Side::B => v.get(1),
        _ => None,
    }
}
