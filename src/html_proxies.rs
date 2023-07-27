use std::path::{Path, PathBuf};

use crate::proxy_builder::{ProxyBuilder, ProxyBuilderNormal};
use lazy_regex::regex;
use regex::Captures;

#[derive(Clone, Default, Debug)]
struct Bucket {
    name: String,
    type_line: String,
    mana_cost: String,
    art_filename: PathBuf,
    art_credits: String,
    color_indicator: Vec<String>,
    color_identity: Vec<String>,
    set_legendary: bool,
}

macro_rules! delegate_to_bucket {
    ($( $function:ident : $type:ty ),+) => {
        $(
            fn $function(&mut self, arg: &$type) -> &mut Self {
                self.bucket.$function = arg.into();
                self
            }
        )+
    };
    () => {
        delegate_to_bucket!(
            name : str,
            type_line : str,
            mana_cost : str,
            art_filename : Path,
            art_credits : str,
            color_indicator : [String],
            color_identity : [String]
        );
        fn set_legendary(&mut self, arg: bool) -> &mut Self {
            self.bucket.set_legendary = arg;
            self
        }
    }
}

macro_rules! delegate_to_field {
    ($($function:ident),+) => {
        $(
            fn $function(&mut self, arg: &str) -> &mut Self {
                self.$function = arg.into();
                self
            }
        )+
    };
}

#[derive(Clone, Default, Debug)]
pub struct NormalHtmlBuilder {
    bucket: Bucket,
    rules_text: String,
    flavor_text: String,
    corner_bubble: String,
}

impl NormalHtmlBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    fn image_tag(&self) -> String {
        if self.bucket.art_filename == PathBuf::default() {
            r#"<div class="art-placeholder"></div>"#.into()
        } else {
            format!(
                r#"<img src="{}" />"#,
                self.bucket.art_filename.to_string_lossy()
            )
        }
    }

    fn corner_bubble_tag(&self) -> String {
        if self.corner_bubble.is_empty() {
            "".into()
        } else {
            format!(r#"<div class="corner-bubble">{}</div>"#, self.corner_bubble)
        }
    }

    fn text_box_content(&self) -> String {
        let mut res = String::new();

        if !self.rules_text.is_empty() {
            res += r#"<p class="rules-text">"#;
            res += &self
                .rules_text
                .lines()
                .map(replace_pip_symbols)
                .collect::<Vec<_>>()
                .join("<br />\n");
            res += "</p>\n"
        }

        if !self.flavor_text.is_empty() {
            if !res.is_empty() {
                res += "<hr />";
            }
            res += r#"<p class="flavor-text">"#;
            res += &self.flavor_text;
            res += "</p>";
        }

        res
    }

    fn legendary_status(&self) -> &'static str {
        if self.bucket.set_legendary {
            "legendary"
        } else {
            ""
        }
    }

    fn type_bar(&self) -> String {
        let mut res = String::new();

        let mut pips = String::new();
        for sym in &self.bucket.color_indicator {
            pips += "{";
            pips += &sym;
            pips += "}";
        }

        if !pips.is_empty() {
            res += r#"<span class="color-indicator">"#;
            res += &replace_pip_symbols(&pips);
            res += "</span>";
        }

        res += "<span>";
        res += &self.bucket.type_line;
        res += "</span>";

        res
    }
}

impl ProxyBuilder for NormalHtmlBuilder {
    type Output = String;

    fn build(&mut self) -> Self::Output {
        format!(
            r#"
<div class="card normal">
    <div class="title-bar {is_legendary}">
        <span class="name">{card_name}</span>
        <span class="mana-cost">{mana_cost}</span>
    </div>
    {image_tag}
    <div class="type-bar">
        {type_line}
    </div>
    <div class="text-box">
        {text_box}
    </div>
    {corner_bubble}
    <span class="art-credits">{art_credits}</span>
</div>
        "#,
            card_name = self.bucket.name,
            is_legendary = self.legendary_status(),
            mana_cost = replace_pip_symbols(&self.bucket.mana_cost),
            image_tag = self.image_tag(),
            type_line = self.type_bar(),
            text_box = self.text_box_content(),
            corner_bubble = self.corner_bubble_tag(),
            art_credits = self.bucket.art_credits,
        )
    }

    delegate_to_bucket!();
}

impl ProxyBuilderNormal for NormalHtmlBuilder {
    delegate_to_field!(rules_text, flavor_text, corner_bubble);
}

fn replace_pip_symbols(mana_notation: &str) -> String {
    let mana_sym = regex!(r"\{[WUBRGCP0-9/]+\}");

    mana_sym
        .replace_all(mana_notation, |cap: &Captures<'_>| {
            let m = cap.get(0).unwrap().as_str();
            let p: PathBuf = format!("./svg/{}.svg", m.replace('/', "|")).into();
            if p.exists() {
                format!(
                    r#"<img class="pip" src="{}"/>"#,
                    p.to_string_lossy().into_owned()
                )
            } else {
                m.into()
            }
        })
        .into_owned()
}
