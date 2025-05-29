use std::collections::BTreeSet;

use build_html::{HtmlChild, HtmlElement, HtmlTag};

use crate::{
    atomic_cards::{cards::Card, types::*},
    utils::iter::IterExt,
};

use super::utils::HtmlExt;

pub fn title_bar_div(name: &str, cost: &str) -> HtmlElement {
    HtmlElement::new(HtmlTag::Div)
        .with_classes(["title bar"])
        .with_child_element(card_name_span(name))
        .with_child_element(mana_cost_span(cost))
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
