use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

use crate::proxy_builder::{
    BasicLand, BasicLandProxyBuilder, DeckBuilder, ProxyBuilder, ProxyBuilderNormal,
    ProxyBuilderSaga,
};
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

impl Bucket {
    fn new() -> Self {
        Default::default()
    }
    fn image_tag(&self) -> String {
        if self.art_filename != PathBuf::new() {
            format!(r#"<img src="{}" />"#, self.art_filename.to_string_lossy())
        } else {
            r#"<div class="art-placeholder"></div>"#.into()
        }
    }

    fn art_credits_tag(&self) -> String {
        if self.art_credits.is_empty() {
            "".into()
        } else {
            format!(r#"<span class="art-credits">{}</span>"#, self.art_credits)
        }
    }

    fn title_bar_tag(&self) -> String {
        let mut res = format!(r#"<span class="name">{}</span>"#, &self.name);

        if !self.mana_cost.is_empty() {
            res += r#"<span class="mana-cost">"#;
            res += &replace_pip_symbols(&self.mana_cost);
            res += "</span>";
        }

        format!(
            r#"<div class="title-bar{}">{}</div>"#,
            if self.set_legendary { " legendary" } else { "" },
            res
        )
    }

    fn type_bar_tag(&self) -> String {
        let mut pips = String::new();
        for sym in &self.color_indicator {
            pips += "{";
            pips += &sym;
            pips += "}";
        }

        let ind;
        if !pips.is_empty() {
            ind = format!(
                r#"<span class="color-indicator">{}</span>"#,
                &replace_pip_symbols(&pips)
            );
        } else {
            ind = "".into();
        }

        format!(
            r#"<div class="type-bar">{}<span>{}</span></div>"#,
            &ind, &self.type_line
        )
    }
}

macro_rules! delegate_to_bucket {
    ($( $function:ident : $type:ty ),+) => {
        $(
            fn $function(&mut self, arg: $type) -> &mut Self {
                self.bucket.$function = arg.into();
                self
            }
        )+
    };
    () => {
        delegate_to_bucket!(
            name : &str,
            type_line : &str,
            mana_cost : &str,
            art_filename : &Path,
            art_credits : &str,
            color_indicator : &[String],
            color_identity : &[String],
            set_legendary : bool
        );
    }
}

macro_rules! delegate_to_field {
    ($($function:ident : $type:ty),+) => {
        $(
            fn $function(&mut self, arg: $type) -> &mut Self {
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

    fn corner_bubble_tag(&self) -> String {
        if self.corner_bubble.is_empty() {
            "".into()
        } else {
            format!(r#"<div class="corner-bubble">{}</div>"#, self.corner_bubble)
        }
    }

    fn text_box_tag(&self) -> String {
        let mut res: String = if self.rules_text.len() + self.flavor_text.len() > 240 {
            r#"<div class="text-box dense">"#
        } else {
            r#"<div class="text-box">"#
        }
        .into();

        if !self.rules_text.is_empty() {
            let rules = &regex!(r"\((.*)\)").replace_all(&self.rules_text, |cap: &Captures<'_>| {
                format!(
                    r#"<span class="reminder-text">{}</span>"#,
                    cap.get(1).unwrap().as_str()
                )
            });

            for line in rules.lines() {
                res += r#"<p class="rules-text">"#;
                res += &replace_pip_symbols(line);
                res += "</p>";
            }
        }

        if !self.flavor_text.is_empty() {
            if !res.is_empty() {
                res += "<hr />";
            }
            res += r#"<p class="flavor-text">"#;
            res += &self.flavor_text;
            res += "</p>";
        }

        res += "</div>";

        res
    }
}

impl ProxyBuilder for NormalHtmlBuilder {
    type Output = String;

    fn build(&mut self) -> Self::Output {
        format!(
            r#"
<div class="card normal">
    {title_bar_tag}
    {image_tag}
    {type_bar_tag}
    {text_box_tag}
    {corner_bubble}
    {art_credits_tag}
</div>
        "#,
            title_bar_tag = self.bucket.title_bar_tag(),
            image_tag = self.bucket.image_tag(),
            type_bar_tag = self.bucket.type_bar_tag(),
            text_box_tag = self.text_box_tag(),
            corner_bubble = self.corner_bubble_tag(),
            art_credits_tag = self.bucket.art_credits_tag(),
        )
    }

    delegate_to_bucket!();
}

impl ProxyBuilderNormal for NormalHtmlBuilder {
    delegate_to_field!(rules_text : &str, flavor_text : &str, corner_bubble : &str);
}

#[derive(Debug, Clone, Default)]
pub struct SagaHtmlBuilder {
    bucket: Bucket,
    steps: Vec<(String, Vec<u32>)>,
    include_reminder: bool,
    flavor_text: String,
}

impl SagaHtmlBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    const ROMAN_NUMERAL_ONE: char = '\u{2160}';

    fn lore_char(step: u32) -> char {
        char::from_u32(Self::ROMAN_NUMERAL_ONE as u32 - 1 + step).unwrap()
    }

    fn max_lore(&self) -> u32 {
        *self
            .steps
            .iter()
            .flat_map(|(_, x)| &x[..])
            .max()
            .unwrap_or(&0)
    }

    fn text_box_content(&self) -> String {
        let mut res = Vec::new();

        if self.include_reminder {
            res.push(format!(
                r#"<p class="reminder-text">As this Saga enters and after your draw step, add a lore counter.
                Sacrifice after {}</p>"#,
                Self::lore_char(self.max_lore())
            ));
        }

        for (step_text, lore) in &self.steps {
            res.push(format!(
                r#"<p class="step"><span class="hex">{}</span> {}</p>"#,
                String::from_iter(lore.iter().map(|s| Self::lore_char(*s))),
                &step_text[..]
            ));
        }

        if !self.flavor_text.is_empty() {
            res.push(format!(
                r#"<p class="flavor-text">{}</p>"#,
                &self.flavor_text
            ));
        }

        res.join("<hr />")
    }
}

impl ProxyBuilder for SagaHtmlBuilder {
    type Output = String;

    fn build(&mut self) -> Self::Output {
        format!(
            r#"
<div class="card saga">
    {title_bar_tag}
    <div class="text-and-art">
        <div class="text-box dense">
            {text_box_content}
        </div>
        {image_tag}
    </div>
    {type_bar_tag}
    {art_credits_tag}
</div>
            "#,
            title_bar_tag = self.bucket.title_bar_tag(),
            text_box_content = self.text_box_content(),
            image_tag = self.bucket.image_tag(),
            type_bar_tag = self.bucket.type_bar_tag(),
            art_credits_tag = self.bucket.art_credits_tag()
        )
    }

    delegate_to_bucket!();
}

impl ProxyBuilderSaga for SagaHtmlBuilder {
    fn step_text(&mut self, steps: &[u32], step_text: &str) -> &mut Self {
        self.steps.push((step_text.into(), steps.into()));
        self
    }

    delegate_to_field!(include_reminder : bool, flavor_text : &str);
}

fn replace_pip_symbols(mana_notation: &str) -> String {
    let mana_sym = regex!(r"\{[WUBRGCP0-9/]+\}");

    mana_sym
        .replace_all(mana_notation, |cap: &Captures<'_>| {
            let m = cap.get(0).unwrap().as_str();
            let p: PathBuf = format!("../svg/{}.svg", m.replace('/', "|")).into();
            format!(
                r#"<img class="pip" src="{}"/> "#,
                p.to_string_lossy().into_owned()
            )
        })
        .into_owned()
}

#[derive(Clone, Default, Debug)]
pub struct BasicLandHtmlBuilder {
    cycle: HashMap<BasicLand, usize>,
    arts: HashMap<BasicLand, Vec<(PathBuf, String)>>,
}

impl BasicLandHtmlBuilder {
    pub fn new() -> Self {
        let mut arts = HashMap::new();
        let mut cycle = HashMap::new();

        for land in BasicLand::ALL {
            let land = *land;
            arts.insert(land, Vec::new());
            cycle.insert(land, 0);
        }

        Self { arts, cycle }
    }
}

impl BasicLandProxyBuilder for BasicLandHtmlBuilder {
    type Output = String;

    fn build(&mut self, land: crate::proxy_builder::BasicLand) -> Self::Output {
        let mut cycle = self.cycle[&land];
        let mut bucket = Bucket::new();

        if let Some((art_filename, art_credits)) = self.arts[&land].get(cycle) {
            cycle += 1;
            self.cycle.insert(land, cycle);

            bucket.art_credits = art_credits.clone();
            bucket.art_filename = art_filename.clone();
        } else {
            cycle = 0;
            self.cycle.insert(land, cycle);
        }

        bucket.name = format!("{}", land);
        bucket.type_line = match land {
            BasicLand::Base(l) => format!("Basic Land &mdash; {}", l),
            BasicLand::Snow(l) => format!("Basic Snow Land &mdash; {}", l),
            BasicLand::Wastes => "Basic Land".into(),
        };

        "".into()
    }

    fn art(&mut self, land: BasicLand, art_filename: &Path, artist: &str) -> &mut Self {
        match self.arts.entry(land) {
            std::collections::hash_map::Entry::Occupied(mut oe) => {
                oe.get_mut().push((art_filename.into(), artist.into()))
            }
            std::collections::hash_map::Entry::Vacant(_) => panic!(),
        }
        self
    }
}

#[derive(Clone, Default, Debug)]
pub struct FirefoxFriendlyHtmlDeckList {
    pages: Vec<String>,
    page: Vec<String>,
    row: Vec<String>,
}

impl FirefoxFriendlyHtmlDeckList {
    pub fn new() -> Self {
        Default::default()
    }
}

impl DeckBuilder for FirefoxFriendlyHtmlDeckList {
    type Input = String;
    type Output = Box<dyn Write>;
    type Result = std::io::Result<()>;
    fn add_card(&mut self, card: Self::Input) -> &mut Self {
        self.row.push(card);

        if self.row.len() >= 3 {
            self.page.push(format!(
                r#"<div class="card-row">{}</div>"#,
                self.row.join("\n")
            ));
            self.row.clear();
        }

        if self.page.len() >= 3 {
            self.pages
                .push(format!("<page>{}</page>\n", self.page.join("\n")));
            self.page.clear();
        }

        self
    }

    fn build(&self, out: &mut Self::Output) -> Self::Result {
        let out = &mut **out;

        out.write_fmt(format_args!(
            r#"
        <html><head>
        <link rel="stylesheet" href="../css/page.css" />
        <link rel="stylesheet" href="../css/card.css" />
        <link rel="stylesheet" href="../css/normal-card.css" />
        <link rel="stylesheet" href="../css/saga-card.css" />
        </head><body>
        "#
        ))?;

        for page in &self.pages {
            let page = &page[..];
            out.write_fmt(format_args!("{}", page))?;
        }

        out.write_fmt(format_args!(r#"</body></html>"#))?;

        Ok(())
    }
}
