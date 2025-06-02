use std::collections::BTreeSet;

use lazy_regex::regex;

use crate::{
    atomic_cards::{cards::Card, types::*},
    html::*,
    proxy::Proxy,
    rendering::manafont::ManaFontSymbolics,
    utils::{iter::IterExt, symbolics::replace_symbols},
};

use super::RenderSettings;

pub fn empty_card(card: &Card) -> Element {
    Element::new(Tag::div).class(card_css_class(card))
}

pub fn title_bar_div(name: &str, cost: &str) -> Element {
    Element::new(Tag::div)
        .class(["title bar"])
        .elem(card_name_span(name))
        .elem(mana_cost_span(cost))
}

pub fn mana_cost_span(mana_cost: &str) -> Element {
    Element::new(Tag::span)
        .class(["cost"])
        .nodes(replace_symbols(&ManaFontSymbolics, mana_cost))
}

pub fn card_name_span(name: &str) -> Element {
    Element::new(Tag::span).class(["name"]).text(name)
}

pub fn card_art_img(proxy: &Proxy, side: Side) -> Vec<Node> {
    let art = match side {
        Side::A => proxy.art_urls.get(0),
        Side::B => proxy.art_urls.get(1),
        _ => None,
    };

    let credit = match side {
        Side::A => proxy.art_credits.get(0),
        Side::B => proxy.art_credits.get(1),
        _ => None,
    };

    if let (Some(art), Some(credit)) = (art, credit) {
        vec![
            Node::Element(Element::new(Tag::img).class(["art"]).attr("src", art)),
            Node::Element(Element::new(Tag::span).class(["art-credits"]).text(credit)),
        ]
    } else {
        vec![Node::Element(Element::new(Tag::img).class(["art"]))]
    }
}

pub fn type_line_div(card: &Card) -> Element {
    Element::new(Tag::div)
        .class(["type-line", "bar"])
        .elem(type_line_span(card))
}

pub fn type_line_span(card: &Card) -> Element {
    Element::new(Tag::span)
        .class(["type"])
        .text(&card.type_line)
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

pub fn format_rules_text(text: String, settings: &RenderSettings) -> String {
    if settings.reminder_text {
        text
    } else {
        regex!(r"\([^\n]+?\)").replace_all(&text, "").into_owned()
    }
}
