use regex::Regex;

use crate::{
    html::{Element, Node, Tag},
    utils::symbolics::SymbolReplacer,
};

#[derive(Default, Clone, Copy)]
pub struct ReminderText;

impl SymbolReplacer for ReminderText {
    fn matcher(&self) -> regex::Regex {
        Regex::new(r"\(.*?\)").unwrap()
    }

    const WRAPPER: bool = true;

    fn wrap_symbol(&self, matched: Vec<Node>) -> Vec<Node> {
        vec![Element::new(Tag::span)
            .class(["reminder"])
            .nodes(matched)
            .into()]
    }
}

#[derive(Default, Clone, Copy)]
pub struct NoReminderText;

impl SymbolReplacer for NoReminderText {
    fn matcher(&self) -> regex::Regex {
        Regex::new(r"\(.*?\)").unwrap()
    }

    const WRAPPER: bool = false;

    fn map_symbol(&self, _: &str) -> Vec<Node> {
        vec![]
    }
}

#[derive(Default, Clone, Copy)]
pub struct PowerToughnessNobreak;

impl SymbolReplacer for PowerToughnessNobreak {
    fn matcher(&self) -> regex::Regex {
        Regex::new(r"([+-]?)(\d+|[XYZ\*])/([+-]?)(\d+|[XYZ\*])").unwrap()
    }

    const WRAPPER: bool = false;

    fn map_symbol(&self, matched: &str) -> Vec<Node> {
        let c = self.matcher().captures(matched).unwrap();
        let [c1, c2, c3, c4] = [1, 2, 3, 4].map(|n| c.get(n).unwrap().as_str());
        let mut res = vec![];
        if !c1.is_empty() {
            res.push(c1);
        }
        res.push(c2);
        res.push("/");
        if !c3.is_empty() {
            res.push(c3);
        }
        res.push(c4);
        vec![res.join("&NoBreak;").into()]
    }
}
