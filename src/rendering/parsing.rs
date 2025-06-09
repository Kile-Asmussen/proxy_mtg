use std::str::FromStr;

use regex::Regex;

use crate::{
    html::Element,
    rendering::manafont::{
        ms_cost_shadow, ms_loyalty_down, ms_loyalty_up, ms_loyalty_zero, ms_saga,
    },
};

pub fn split_line_starter<'a>(rx: &Regex, text: &'a str) -> (&'a str, &'a str) {
    let Some(c) = rx.captures(&text) else {
        return ("", text);
    };
    let Some(m) = c.get(1) else {
        return ("", text);
    };
    (m.as_str(), &text[c.get(0).unwrap().end()..])
}

pub fn split_anchor_word(text: &str) -> (&str, &str) {
    let flavor_word = Regex::new(r"^((?:[\w•]+\s*?)+)\s+—\s+").unwrap();

    let (word, rest) = split_line_starter(&flavor_word, text);
    if word == "Companion" {
        ("", text)
    } else {
        (word, rest)
    }
}

pub fn split_loyalty_ability(text: &str) -> (&str, &str) {
    let loyalty_ability = Regex::new(r"^(\[[+−]?\d+\]):").unwrap();

    split_line_starter(&loyalty_ability, text)
}

pub fn split_chapter_abilities(mut text: &str) -> (Vec<&str>, &str) {
    let numeral = Regex::new(r"\<(?:I|II|III|IV|V|VI)\>").unwrap();
    let chapter = Regex::new(r"^(.*?)\s+—\s+").unwrap();

    let (numerals, rest) = split_line_starter(&chapter, text);

    let mut res = vec![];
    for num in numeral.find_iter(numerals) {
        res.push(num.as_str())
    }
    if !res.is_empty() {
        text = rest;
    }
    (res, text)
}

pub fn chapter_symbol(m: &str) -> Option<Element> {
    Some(ms_saga(match m {
        "I" => "1",
        "II" => "2",
        "III" => "3",
        "IV" => "4",
        "V" => "5",
        "VI" => "6",
        "VII" => "7",
        "VIII" => "8",
        "IX" => "9",
        "X" => "10",
        _ => return None,
    }))
}

pub fn loyalty_symbol(m: &str) -> Option<Element> {
    Some(match m {
        "[+X]" => ms_loyalty_up("x"),
        "[−X]" => ms_loyalty_down("x"),
        "[0]" => ms_loyalty_zero("0"),
        _ => {
            let n = <isize as FromStr>::from_str(
                &m.replace("[", "").replace("]", "").replace("−", "-"),
            )
            .ok()?;
            match n {
                -25 => ms_loyalty_down("25"),
                -20..=-1 => ms_loyalty_down(&n.to_string()),
                1..=20 => ms_loyalty_up(&n.to_string()),
                25 => ms_loyalty_up("25"),
                _ => return None,
            }
        }
    })
}

pub fn colored_mana(m: &str) -> Option<Element> {
    Some(match m {
        "{W}" => ms_cost_shadow("w"),
        "{U}" => ms_cost_shadow("u"),
        "{B}" => ms_cost_shadow("b"),
        "{R}" => ms_cost_shadow("r"),
        "{G}" => ms_cost_shadow("g"),
        "{C}" => ms_cost_shadow("c"),
        _ => return None,
    })
}

pub fn tap_untap(m: &str) -> Option<Element> {
    Some(match m {
        "{T}" => ms_cost_shadow("tap"),
        "{Q}" => ms_cost_shadow("untap"),
        _ => return None,
    })
}

pub fn hybrid_mana(_: &str) -> Option<Element> {
    None
}

pub fn generic_mana(m: &str) -> Option<Element> {
    let n = <usize as FromStr>::from_str(&m.replace("{", "").replace("}", "")).ok()?;
    if n <= 20 {
        Some(ms_cost_shadow(&n.to_string()))
    } else {
        None
    }
}
