use regex::Regex;

use crate::{
    html::{Element, Node, Tag},
    rendering::manafont::ManaFontSymbolics,
    utils::symbolics::{replace_symbols_with, RulesTextSymbolReplacer},
};

#[derive(Default, Clone, Copy)]
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
                .nodes(replace_symbols_with(&ManaFontSymbolics, matched)),
        )]
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        replace_symbols_with(&ManaFontSymbolics, non_matched)
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }
}

#[derive(Default, Clone, Copy)]
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
        replace_symbols_with(&ManaFontSymbolics, non_matched)
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }
}
