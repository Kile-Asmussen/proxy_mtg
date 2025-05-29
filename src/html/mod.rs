use std::collections::{BTreeMap, BTreeSet};

use build_html::HtmlElement;
use general::{empty_card, title_bar_div};
use utils::HtmlExt;

use crate::proxy::Proxy;

pub mod general;
pub mod normal;
pub mod utils;

pub struct RenderSettings {
    pub color: bool,
    pub reminder_text: bool,
}

pub struct RenderContext {
    pub settings: RenderSettings,
    pub cards: Vec<HtmlElement>,
}

impl RenderContext {
    pub fn add_proxy(&mut self, proxy: &Proxy) {
        if proxy.cardoid.is_none() {
            self.cards.push(
                empty_card(BTreeSet::new(), &[]).with_child_element(title_bar_div(&proxy.name, "")),
            );
        }
    }
}
