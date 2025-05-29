use std::collections::BTreeSet;

use build_html::{HtmlChild, HtmlContainer, HtmlElement, HtmlTag};
use lazy_regex::regex;

use crate::{
    atomic_cards::{cards::Card, types::*},
    proxy::Proxy,
    utils::iter::IterExt,
};

use super::{fragments::HtmlExt, RenderSettings};

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

pub fn rules_text_div(card: &Card, settings: &RenderSettings) -> HtmlElement {
    let mut text = card.text.clone();

    if !settings.reminder_text {
        text = regex!(r"\([^\n]+\)").replace_all(&text, "").into_owned();
    }
    println!("{} {}", text, settings.reminder_text);

    let text_len = text.len();

    let paragraphs = text.lines().map(ToOwned::to_owned).collvect();

    let class: &[&str] = if paragraphs.len() == 1 && text_len < 50 {
        &["text-box", "sparse"]
    } else if paragraphs.len() >= 3 || text_len >= 200 {
        &["text-box", "dense"]
    } else {
        &["text-box"]
    };

    let mut res = HtmlElement::new(HtmlTag::Div).with_classes(class);

    if text.is_empty()
        && card.supertypes.contains(&Supertype::Basic)
        && card.types.contains(&Type::Land)
    {
        res.add_element(HtmlElement::new(HtmlTag::ParagraphText).with_classes(["rules-text"]));
    } else {
        for line in text.lines() {
            res.add_element(
                HtmlElement::new(HtmlTag::ParagraphText)
                    .with_classes(["rules-text"])
                    .with_text(line.to_string()),
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
