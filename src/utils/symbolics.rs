use std::collections::BTreeSet;

use regex::Regex;

use crate::atomic_cards::types::WUBRG;

pub fn replace_symbols<R>(replacer: &R, mut haystack: &str) -> Vec<R::Item>
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

pub trait RulesTextSymbolReplacer {
    type Item;

    fn matcher(&self) -> Regex {
        Regex::new(r"\{.*?\}").unwrap()
    }

    fn map_symbol(&self, matched: &str) -> Self::Item;

    fn intermediate_text(&self, non_matched: &str) -> Self::Item;

    fn joiner(&self) -> Option<Self::Item>;

    fn indicator(&self, indicate: &BTreeSet<WUBRG>) -> Self::Item;
}

#[derive(Default, Clone, Copy)]
pub struct NothingReplacer;

impl RulesTextSymbolReplacer for NothingReplacer {
    type Item = String;

    fn map_symbol(&self, matched: &str) -> Self::Item {
        matched.to_string()
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        non_matched.to_string()
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }

    fn indicator(&self, indicate: &BTreeSet<WUBRG>) -> Self::Item {
        WUBRG::render(indicate)
    }
}

#[derive(Default, Clone, Copy)]
pub struct DiscordEmoji;

impl RulesTextSymbolReplacer for DiscordEmoji {
    type Item = String;

    fn map_symbol(&self, matched: &str) -> Self::Item {
        Self::symbols(matched).to_string()
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        non_matched.to_string()
    }

    fn joiner(&self) -> Option<Self::Item> {
        Some(" ".to_string())
    }

    fn indicator(&self, indicate: &BTreeSet<WUBRG>) -> Self::Item {
        let mut res = "".to_string();

        for c in indicate {
            res += match c {
                WUBRG::W => ":yellow_circle:",
                WUBRG::U => ":blue_circle:",
                WUBRG::B => ":black_circle:",
                WUBRG::R => ":red_circle:",
                WUBRG::G => ":green_circle:",
            }
        }

        res
    }
}

impl DiscordEmoji {
    fn symbols(matched: &str) -> &str {
        match matched {
            "{C}" => ":diamond_shape_with_a_dot_inside:",
            "{W}" => ":sunny:",
            "{U}" => ":droplet:",
            "{B}" => ":skull:",
            "{R}" => ":fire:",
            "{G}" => ":deciduous_tree:",
            "{S}" => ":snowflake:",
            "{1}" => ":one:",
            "{2}" => ":two:",
            "{3}" => ":three:",
            "{4}" => ":four:",
            "{5}" => ":five:",
            "{6}" => ":six:",
            "{7}" => ":seven:",
            "{8}" => ":eight:",
            "{9}" => ":nine:",
            "{10}" => ":keycap_ten:",
            "{T}" => ":arrow_heading_down:",
            "{Q}" => ":arrow_heading_up:",
            "{W/P}" => "(:sunny:/:drop_of_blood:)",
            "{U/P}" => "(:droplet:/:drop_of_blood:)",
            "{B/P}" => "(:skull:/:drop_of_blood:)",
            "{R/P}" => "(:fire:/:drop_of_blood:)",
            "{G/P}" => "(:deciduous_tree:/:drop_of_blood:)",
            "{C/P}" => "(:diamond_shape_with_a_dot_inside:/:drop_of_blood:)",
            "{2/W}" => "(:two:/:sunny:)",
            "{2/U}" => "(:two:/:droplet:)",
            "{2/B}" => "(:two:/:skull:)",
            "{2/R}" => "(:two:/:fire:)",
            "{2/G}" => "(:two:/:deciduous_tree:)",
            "{C/W}" => "(:diamond_shape_with_a_dot_inside:/:sunny:)",
            "{C/U}" => "(:diamond_shape_with_a_dot_inside:/:droplet:)",
            "{C/B}" => "(:diamond_shape_with_a_dot_inside:/:skull:)",
            "{C/R}" => "(:diamond_shape_with_a_dot_inside:/:fire:)",
            "{C/G}" => "(:diamond_shape_with_a_dot_inside:/:deciduous_tree:)",
            s => s,
        }
    }
}
