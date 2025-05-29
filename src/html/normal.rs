use std::{collections::BTreeSet, ops::Div};

use build_html::{Html, HtmlChild, HtmlElement, HtmlTag};

use crate::{
    atomic_cards::{
        cards::{Card, Cardoid},
        types::Layout,
    },
    proxy::Proxy,
};

use super::{general::*, utils::HtmlExt, RenderSettings};

fn normal_card(proxy: &Proxy, settings: &RenderSettings) -> HtmlElement {
    let Some(cardoid) = proxy.cardoid else {
        return empty_card(BTreeSet::new(), &[]);
    };

    let card = cardoid.front();
}
