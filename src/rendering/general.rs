use std::collections::BTreeSet;

use lazy_regex::regex;

use crate::{
    atomic_cards::{cards::Card, types::*},
    html::{Element, Tag},
    proxy::Proxy,
    utils::iter::IterExt,
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
    Element::new(Tag::span).class(["name"]).text(mana_cost)
}

pub fn card_name_span(name: &str) -> Element {
    Element::new(Tag::span).class(["name"]).text(name)
}

pub fn card_art_img(proxy: &Proxy) -> Element {
    Element::new(Tag::img).class(["art"]).attr("src", "")
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

pub fn rules_text_div(card: &Card, settings: &RenderSettings) -> Element {
    let mut text = card.text.clone();

    if !settings.reminder_text {
        text = regex!(r"\([^\n]+\)").replace_all(&text, "").into_owned();
    }

    let text_len = text.len();

    let paragraphs = text.lines().map(ToOwned::to_owned).collvect();

    let class: &[&str] = if paragraphs.len() == 1 && text_len < 50 {
        &["text-box", "sparse"]
    } else if paragraphs.len() >= 3 || text_len >= 200 {
        &["text-box", "dense"]
    } else {
        &["text-box"]
    };

    let mut res = Element::new(Tag::div).class(class);

    if text.is_empty()
        && card.supertypes.contains(&Supertype::Basic)
        && card.types.contains(&Type::Land)
    {
        res = res.elem(
            Element::new(Tag::p)
                .class(["rules-text"])
                .text(WUBRG::wubrg(&card.color_identity)),
        );
    } else {
        for line in text.lines() {
            res = res.elem(
                Element::new(Tag::p)
                    .class(["rules-text"])
                    .text(line.to_string()),
            );
        }
    }

    res
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
