use regex::Regex;

use crate::html::Node;

pub fn replace_symbols_with<R>(replacer: &R, mut haystack: &str) -> Vec<Node>
where
    R: SymbolReplacer,
{
    let matcher = replacer.matcher();

    let mut vec = vec![];

    while !haystack.is_empty() {
        if let Some(matched) = matcher.find(haystack) {
            if matched.start() != 0 {
                vec.append(&mut replacer.intermediate_text(&haystack[..matched.start()]));
                vec.append(&mut replacer.joiner());
            }
            vec.append(&mut replacer.map_symbol(matched.as_str()));
            haystack = &haystack[matched.end()..];
            vec.append(&mut replacer.joiner());
        } else {
            vec.append(&mut replacer.intermediate_text(haystack));
            haystack = "";
        }
    }

    vec
}

pub fn replace_symbols<R>(haystack: &str) -> Vec<Node>
where
    R: SymbolReplacer + Default,
{
    replace_symbols_with::<R>(&Default::default(), haystack)
}

pub trait SymbolReplacer {
    fn matcher(&self) -> Regex;

    fn map_symbol(&self, matched: &str) -> Vec<Node> {
        self.wrap_symbol(vec![matched.into()])
    }

    const WRAPPER: bool;
    fn wrap_symbol(&self, matched: Vec<Node>) -> Vec<Node> {
        matched
    }

    fn intermediate_text(&self, non_matched: &str) -> Vec<Node> {
        vec![non_matched.into()]
    }

    fn joiner(&self) -> Vec<Node> {
        vec![]
    }
}

#[derive(Default, Clone, Copy)]
pub struct Symchain<R0, R1>(R0, R1)
where
    R0: SymbolReplacer,
    R1: SymbolReplacer;

impl<R0, R1> SymbolReplacer for Symchain<R0, R1>
where
    R0: SymbolReplacer,
    R1: SymbolReplacer,
{
    fn matcher(&self) -> Regex {
        self.0.matcher()
    }

    const WRAPPER: bool = R1::WRAPPER;

    fn wrap_symbol(&self, matched: Vec<Node>) -> Vec<Node> {
        self.0.wrap_symbol(self.1.wrap_symbol(matched))
    }

    fn map_symbol(&self, matched: &str) -> Vec<Node> {
        if R0::WRAPPER {
            self.0.wrap_symbol(replace_symbols_with(&self.1, matched))
        } else {
            self.0.map_symbol(matched)
        }
    }

    fn intermediate_text(&self, non_matched: &str) -> Vec<Node> {
        replace_symbols_with(&self.1, non_matched)
    }

    fn joiner(&self) -> Vec<Node> {
        self.0.joiner()
    }
}
