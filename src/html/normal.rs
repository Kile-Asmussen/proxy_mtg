use std::{collections::BTreeSet, ops::Div};

use build_html::{Html, HtmlChild, HtmlElement, HtmlTag};

use crate::{
    atomic_cards::{cards::*, types::*},
    proxy::Proxy,
};

use super::{general::*, utils::HtmlExt, RenderSettings};

fn normal_card(proxy: &Proxy, settings: &RenderSettings) -> HtmlElement {
    let card = proxy.cardoid.face();

    empty_card(card)
        .with_element(type_line_div(card))
        .with_element(card_art_img(proxy))
        .with_element(type_line_div(card))
        .with_element(rules_text_div(card))
}
