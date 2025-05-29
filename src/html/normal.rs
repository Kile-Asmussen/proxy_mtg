use std::ops::Div;

use build_html::{Html, HtmlChild, HtmlElement, HtmlTag};

use crate::{
    atomic_cards::cards::{Card, Cardoid},
    decklist::Proxy,
};

use super::{
    utils::{card_css_class, HtmlElementExt},
    RenderSettings,
};

fn normal_card(proxy: &Proxy, settings: &RenderSettings) -> HtmlElement {
    let card = proxy.cardoid.as_ref().unwrap().front();
    let mut res = HtmlElement::new(HtmlTag::Div);

    res.add_classes(card_css_class(card));
    res.add_child(HtmlChild::Element(title_bar(card)));

    res
}

fn title_bar(card: &Card) -> HtmlElement {
    let mut res = HtmlElement::new(HtmlTag::Div);
    res.add_classes([""]);
    res
}
