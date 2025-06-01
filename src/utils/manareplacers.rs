use std::collections::BTreeSet;

use lazy_regex::regex;
use regex::{Match, Regex};

use crate::atomic_cards::types::WUBRG;

use super::iter::IterExt;

pub trait ManaReplacer {
    type Item;
    fn dispatch(&self, matched: &str) -> Self::Item;
    fn scratch(&self, non_matched: &str) -> Self::Item;

    fn replace(&self, mut haystack: &str) -> Vec<Self::Item> {
        let matcher = regex!(r"\{.*?\}");

        let mut vec = vec![];

        while !haystack.is_empty() {
            if let Some(matched) = matcher.find(haystack) {
                if matched.start() != 0 {
                    vec.push(self.scratch(&haystack[..matched.start()]));
                }
                vec.push(self.dispatch(matched.as_str()));
                haystack = &haystack[matched.end()..];
            } else {
                vec.push(self.scratch(haystack));
                haystack = "";
            }
        }

        vec
    }
}

pub struct DiscordEmoji;

impl ManaReplacer for DiscordEmoji {
    type Item = String;

    fn dispatch(&self, matched: &str) -> Self::Item {
        Self::symbols(matched).to_string()
    }

    fn scratch(&self, non_matched: &str) -> Self::Item {
        non_matched.to_string()
    }
}

impl DiscordEmoji {
    pub fn colored_circles(colors: &BTreeSet<WUBRG>) -> String {
        let mut res = "".to_string();

        for c in colors {
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

    fn symbols(matched: &str) -> &str {
        match matched {
            "{C}" => ":diamond_shape_with_a_dot_inside:",
            "{W}" => ":sunny:",
            "{U}" => ":droplet:",
            "{B}" => ":skull:",
            "{R}" => ":fire:",
            "{G}" => ":deciduous_tree:",
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
            "{U}" => ":arrow_heading_up:",
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
