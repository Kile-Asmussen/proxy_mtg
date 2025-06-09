use regex::Regex;

use crate::utils::ToS;

pub fn replace_symbols_with<R>(replacer: &R, mut haystack: &str) -> Vec<R::Item>
where
    R: RulesTextSymbolReplacer,
{
    let matcher = replacer.matcher();

    let mut vec = vec![];

    while !haystack.is_empty() {
        if let Some(matched) = matcher.find(haystack) {
            if matched.start() != 0 {
                vec.push(replacer.intermediate_text(&haystack[..matched.start()]));
                replacer.joiner().map(|i| vec.push(i));
            }
            vec.push(replacer.map_symbol(matched.as_str()));
            haystack = &haystack[matched.end()..];
            replacer.joiner().map(|i| vec.push(i));
        } else {
            vec.push(replacer.intermediate_text(haystack));
            haystack = "";
        }
    }

    vec
}

pub fn replace_symbols<R>(haystack: &str) -> Vec<R::Item>
where
    R: RulesTextSymbolReplacer + Default,
{
    replace_symbols_with::<R>(&Default::default(), haystack)
}

pub trait RulesTextSymbolReplacer {
    type Item;

    fn matcher(&self) -> Regex;

    fn map_symbol(&self, matched: &str) -> Self::Item;

    fn intermediate_text(&self, non_matched: &str) -> Self::Item;

    fn joiner(&self) -> Option<Self::Item>;
}

#[derive(Default, Clone, Copy)]
pub struct NothingReplacer;

impl RulesTextSymbolReplacer for NothingReplacer {
    type Item = String;

    fn matcher(&self) -> Regex {
        Regex::new("\u{10FFFF}").unwrap()
    }

    fn map_symbol(&self, matched: &str) -> Self::Item {
        matched.s()
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        non_matched.s()
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }
}
