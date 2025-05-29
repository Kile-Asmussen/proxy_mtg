use std::{collections::BTreeSet, ops::Div};

use build_html::{Html, HtmlChild, HtmlElement, HtmlTag};

use crate::{
    atomic_cards::{cards::*, types::*},
    proxy::Proxy,
};

use super::{general::*, utils::HtmlExt, RenderSettings};

fn normal_card(proxy: &Proxy, settings: &RenderSettings) -> HtmlElement {
    let card = proxy.cardoid.face();

    HtmlElement::new(HtmlTag::Div)
        .with_classes(card_css_class(card))
        .with_child_element(title_bar_div(&card.name, &card.mana_cost))
}
