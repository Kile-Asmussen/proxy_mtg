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

pub fn hybrid_mana(m: &str) -> Option<Element> {
    Some(match m {
        "{W/P}" => ms_cost_shadow("wp"),
        "{U/P}" => ms_cost_shadow("up"),
        "{B/P}" => ms_cost_shadow("bp"),
        "{R/P}" => ms_cost_shadow("rp"),
        "{G/P}" => ms_cost_shadow("gp"),

        "{2/W}" => ms_cost_shadow("2w"),
        "{2/U}" => ms_cost_shadow("2u"),
        "{2/B}" => ms_cost_shadow("2b"),
        "{2/R}" => ms_cost_shadow("2r"),
        "{2/G}" => ms_cost_shadow("2g"),

        "{W/U}" => ms_cost_shadow("wu"),
        "{U/B}" => ms_cost_shadow("ub"),
        "{B/R}" => ms_cost_shadow("br"),
        "{R/G}" => ms_cost_shadow("rg"),
        "{G/W}" => ms_cost_shadow("gw"),

        "{W/B}" => ms_cost_shadow("wb"),
        "{B/G}" => ms_cost_shadow("bg"),
        "{G/U}" => ms_cost_shadow("gu"),
        "{U/R}" => ms_cost_shadow("ur"),
        "{R/W}" => ms_cost_shadow("rw"),
        _ => return None,
    })
}

pub fn generic_mana(m: &str) -> Option<Element> {
    if m == "{X}" {
        return Some(ms_cost_shadow("x"));
    }
    let n = <usize as FromStr>::from_str(&m.replace("{", "").replace("}", "")).ok()?;
    if n <= 20 {
        Some(ms_cost_shadow(&n.to_string()))
    } else {
        None
    }
}
