use std::ops::Div;

use build_html::{HtmlChild, HtmlElement, HtmlTag};

use crate::{atomic_cards::Cardoid, decklist::Artoid};

use super::{
    utils::{card_css_class, HtmlElementExt},
    RenderSettings,
};

fn normal_card(artoid: &Artoid, settings: &RenderSettings) -> HtmlElement {
    let card = artoid.cardoid.as_ref().unwrap().front();
    let mut res = HtmlElement::new(HtmlTag::Div);

    res.add_classes(card_css_class(card));
    res.add_child(HtmlChild::Element());

    res
}

fn title_bar(card: &Card) -> HtmlElement {
    let mut res = HtmlElement::new(HtmlTag::Div);
    res.add_attribute(k, v);
}
