use regex::Regex;

use crate::{
    html::{Element, Node, Tag},
    rendering::parsing::{colored_mana, generic_mana, hybrid_mana, loyalty_symbol, tap_untap},
    utils::{symbolics::SymbolReplacer, ToS},
};

#[derive(Default, Clone, Copy)]
pub struct ManaFontSymbolics;

impl SymbolReplacer for ManaFontSymbolics {
    const WRAPPER: bool = false;

    fn matcher(&self) -> regex::Regex {
        Regex::new(r"\{.*?\}|\[.*?\]").unwrap()
    }

    fn map_symbol(&self, matched: &str) -> Vec<Node> {
        vec![colored_mana(matched)
            .or_else(|| tap_untap(matched))
            .or_else(|| hybrid_mana(matched))
            .or_else(|| generic_mana(matched))
            .or_else(|| loyalty_symbol(matched))
            .map(|s| s.into())
            .unwrap_or_else(|| matched.s().into())]
    }

    fn intermediate_text(&self, non_matched: &str) -> Vec<Node> {
        vec![Node::from(non_matched)]
    }
}

pub fn ms_cost_shadow(c: &str) -> Element {
    Element::new(Tag::i).class(vec![
        "ms".s(),
        "ms-cost".s(),
        "ms-shadow".s(),
        format!("ms-{}", c),
    ])
}

pub fn ms_loyalty_up(c: &str) -> Element {
    Element::new(Tag::i).class(vec![
        "ms".s(),
        "ms-loyalty-up".s(),
        format!("ms-loyalty-{}", c),
    ])
}

pub fn ms_loyalty_down(c: &str) -> Element {
    Element::new(Tag::i).class(vec![
        "ms".s(),
        "ms-loyalty-down".s(),
        format!("ms-loyalty-{}", c),
    ])
}

pub fn ms_loyalty_zero(c: &str) -> Element {
    Element::new(Tag::i).class(vec![
        "ms".s(),
        "ms-loyalty-zero".s(),
        format!("ms-loyalty-{}", c),
    ])
}

pub fn ms_saga(c: &str) -> Element {
    Element::new(Tag::i).class(vec!["ms".s(), "ms-saga".s(), format!("ms-saga-{}", c)])
}
