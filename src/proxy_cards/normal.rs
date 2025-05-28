use build_html::{HtmlElement, HtmlTag};

use crate::{atomic_cards::Cardoid, decklist::Artoid};

use super::{utils::card_css_class, RenderSettings};

fn normal_card(artoid: &Artoid, settings: &RenderSettings) -> HtmlElement {
    let card = artoid.cardoid.as_ref().unwrap().front();
    HtmlElement::new(HtmlTag::Div)
        .with_attribute("class", card_css_class(card))
        .with_child(content)
}
