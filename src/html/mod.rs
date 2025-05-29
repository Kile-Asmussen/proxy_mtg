use std::collections::{BTreeMap, BTreeSet};

use build_html::HtmlElement;
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
    pub fn add_proxy(&mut self, proxy: &Proxy) {}
}
