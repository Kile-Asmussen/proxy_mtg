use crate::proxy_builder::{ProxyBuilder, ProxyBuilderNormal};
use lazy_regex::regex;
use regex::Captures;

#[derive(Clone, Default, Debug)]
struct Bucket {
    name: String,
    type_line: String,
    mana_cost: String,
    art_filename: String,
    art_credits: String,
}

macro_rules! delegate_to_bucket {
    ($($function:ident),+) => {
        $(
            fn $function(&mut self, arg: &str) -> &mut Self {
                self.bucket.$function = arg.into();
                self
            }
        )+
    };
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
        if self.bucket.art_filename.is_empty() {
            r#"<div class="art-placeholder"/>"#.into()
        } else {
            format!(r#"<img src="{}" />"#, self.bucket.art_filename)
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
            res += &self.rules_text.lines().collect::<Vec<_>>().join("<br />\n");
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
}

impl ProxyBuilder for NormalHtmlBuilder {
    type Output = String;

    fn build(&mut self) -> Self::Output {
        format!(
            r#"
        <div class="card normal">
        <div class="title-bar">
            <span class="name">{card_name}</span>
            <span class="mana-cost">{mana_cost}</span>
        </div>
        {image_tag}
        <div class="type-bar">
            <span>{type_line}</span>
        </div>
        <div class="text-box">
            {text_box}
        </div>
        {corner_bubble}
        <span class="art-credits">{art_credits}</span>
    </div>
        "#,
            card_name = self.bucket.name,
            mana_cost = replace_mana_symbols(&self.bucket.mana_cost),
            image_tag = self.image_tag(),
            type_line = self.bucket.type_line,
            text_box = self.text_box_content(),
            corner_bubble = self.corner_bubble_tag(),
            art_credits = self.bucket.art_credits,
        )
    }

    delegate_to_bucket!(name, type_line, mana_cost, art_filename, art_credits);
}

impl ProxyBuilderNormal for NormalHtmlBuilder {
    delegate_to_field!(rules_text, flavor_text, corner_bubble);
}

fn replace_mana_symbols(mana_notation: &str) -> String {
    let mana_sym = regex!(r"\{([WUBRGC]|[0-9]|1[0-9]|20|[XYZ])\}");

    return mana_sym.replace_all(mana_notation, replacer).into();

    fn replacer(cap: &Captures<'_>) -> String {
        match cap.get(1).unwrap().as_str() {
            "W" => r#"<img class="pip" src="./svg/white-mana-pip.svg">"#,
            "U" => r#"<img class="pip" src="./svg/blue-mana-pip.svg">"#,
            "B" => r#"<img class="pip" src="./svg/black-mana-pip.svg">"#,
            "R" => r#"<img class="pip" src="./svg/red-mana-pip.svg">"#,
            "G" => r#"<img class="pip" src="./svg/green-mana-pip.svg">"#,
            s if i32::from_str_radix(s, 10).is_ok() => {
                return format!(r#"<img class="pip" src="./svg/generic-{s}-pip.svg">"#);
            }
            s => return format!("({s})"),
        }
        .into()
    }
}
