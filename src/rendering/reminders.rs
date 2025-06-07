use regex::Regex;

use crate::{
    atomic_cards::types::WUBRG,
    html::{Element, Node, Tag},
    rendering::manafont::ManaFontSymbolics,
    utils::symbolics::{replace_symbols, RulesTextSymbolReplacer},
};

pub struct ReminderText;

impl RulesTextSymbolReplacer for ReminderText {
    type Item = Vec<Node>;

    fn matcher(&self) -> regex::Regex {
        Regex::new(r"\(.*?\)").unwrap()
    }

    fn map_symbol(&self, matched: &str) -> Self::Item {
        vec![Node::Element(
            Element::new(Tag::span)
                .class(["reminder"])
                .nodes(replace_symbols(&ManaFontSymbolics, matched)),
        )]
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        replace_symbols(&ManaFontSymbolics, non_matched)
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }

    fn indicator(&self, indicate: &std::collections::BTreeSet<WUBRG>) -> Self::Item {
        vec![ManaFontSymbolics.indicator(indicate)]
    }
}

pub struct NoReminderText;

impl RulesTextSymbolReplacer for NoReminderText {
    type Item = Vec<Node>;

    fn matcher(&self) -> regex::Regex {
        Regex::new(r"\(.*?\)").unwrap()
    }

    fn map_symbol(&self, _: &str) -> Self::Item {
        vec![]
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        replace_symbols(&ManaFontSymbolics, non_matched)
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }

    fn indicator(&self, indicate: &std::collections::BTreeSet<WUBRG>) -> Self::Item {
        vec![ManaFontSymbolics.indicator(indicate)]
    }
}
