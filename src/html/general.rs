use std::collections::BTreeSet;

use build_html::{HtmlChild, HtmlContainer, HtmlElement, HtmlTag};

use crate::{
    atomic_cards::{cards::Card, types::*},
    proxy::Proxy,
    utils::iter::IterExt,
};

use super::utils::HtmlExt;

pub fn empty_card(card: &Card) -> HtmlElement {
    HtmlElement::new(HtmlTag::Div).with_classes(card_css_class(card))
}

pub fn title_bar_div(name: &str, cost: &str) -> HtmlElement {
    HtmlElement::new(HtmlTag::Div)
        .with_classes(["title bar"])
        .with_element(card_name_span(name))
        .with_element(mana_cost_span(cost))
}

pub fn mana_cost_span(mana_cost: &str) -> HtmlElement {
    HtmlElement::new(HtmlTag::Span)
        .with_classes(["name"])
        .with_child(HtmlChild::Raw(mana_cost.to_string()))
}

pub fn card_name_span(name: &str) -> HtmlElement {
    HtmlElement::new(HtmlTag::Span)
        .with_classes(["name"])
        .with_child(HtmlChild::Raw(name.to_string()))
}

pub fn card_art_img(proxy: &Proxy) -> HtmlElement {
    HtmlElement::new(HtmlTag::Image)
        .with_classes(["art"])
        .with_attribute("src", "")
}

pub fn type_line_div(card: &Card) -> HtmlElement {
    HtmlElement::new(HtmlTag::Div)
        .with_classes(["type-line", "bar"])
        .with_element(type_line_span(card))
}

pub fn type_line_span(card: &Card) -> HtmlElement {
    HtmlElement::new(HtmlTag::Span)
        .with_classes(["type"])
        .with_child(HtmlChild::Raw(card.type_line.clone()))
}

pub fn rules_text_div(card: &Card) -> HtmlElement {
    let mut res = HtmlElement::new(HtmlTag::Div).with_classes(["text-box"]);

    for line in card.text.lines() {
        res.add_paragraph(line);
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
