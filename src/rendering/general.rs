use crate::{
    atomic_cards::{cards::Card, metadata::ForeignData, types::*},
    html::*,
    proxy::{Art, Proxy},
    rendering::{
        manafont::{ms_cost_shadow, ManaFontSymbolics},
        reminders::{NoReminderText, ReminderText},
    },
    utils::{
        iter::IterExt,
        symbolics::{replace_symbols, replace_symbols_with},
        ToS,
    },
};

pub fn blank_card() -> Element {
    Element::new(Tag::div).class(["card"])
}

pub fn empty_card(card: &Card, proxy: &Proxy) -> Element {
    blank_card()
        .class(card_css_class(card))
        .nodes(title_bar_div(card, proxy))
}

pub fn raw_card(card: &Card, proxy: &Proxy) -> Element {
    empty_card(card, proxy)
        .node(type_line_div(card, proxy))
        .nodes(card_art_img(card, proxy))
}

pub fn title_bar_div(card: &Card, proxy: &Proxy) -> Vec<Element> {
    let (name, alt) = card_name_spans(card, proxy);

    let mut res = vec![];
    res.push(
        Element::new(Tag::div)
            .class(["title", "bar"])
            .node(name)
            .node(mana_cost_span(card, proxy)),
    );
    if let Some(alt) = alt {
        res.push(Element::new(Tag::div).class(["alt-title", "bar"]).node(alt));
    }
    res
}

pub fn mana_cost_span(card: &Card, _proxy: &Proxy) -> Element {
    Element::new(Tag::span)
        .class(["cost"])
        .nodes(replace_symbols_with(&ManaFontSymbolics, &card.mana_cost))
}

pub fn corner_bubble<N>(content: N) -> Element
where
    N: Into<Node>,
{
    Element::new(Tag::div)
        .class(["bar", "corner-bubble"])
        .node(Element::new(Tag::span).node(content))
}

pub fn rules_text_paragraph<NS, N>(text: NS) -> Element
where
    NS: IntoIterator<Item = N>,
    N: Into<Node>,
{
    Element::new(Tag::p).class(["rules-text"]).nodes(text)
}

pub fn flavor_text_paragraphs(card: &Card, proxy: &Proxy) -> Vec<Element> {
    let Some(ForeignData { flavor_text, .. }) = get_side(card.side, &proxy.customize) else {
        return vec![];
    };

    if flavor_text.is_empty() {
        vec![]
    } else {
        flavor_text
            .lines()
            .map(|s| flavor_text_paragraph([s]))
            .collvect()
    }
}

pub fn flavor_text_paragraph<NS, N>(text: NS) -> Element
where
    NS: IntoIterator<Item = N>,
    N: Into<Node>,
{
    Element::new(Tag::p).class(["flavor-text"]).nodes(text)
}

pub fn card_name_spans(card: &Card, proxy: &Proxy) -> (Element, Option<Element>) {
    let mut name = card.face_name.clone();
    let mut alt_name = String::new();
    if name.is_empty() {
        name = card.name.clone();
    }

    if let Some(c) = get_side(card.side, &proxy.customize) {
        if !c.name.is_empty() {
            alt_name = name;
            name = c.name.clone();
        } else if !c.face_name.is_empty() {
            alt_name = name;
            name = c.face_name.clone();
        }
    }

    let res = Element::new(Tag::span).class(["name"]).node(name);
    if !alt_name.is_empty() {
        (
            res,
            Some(Element::new(Tag::span).class(["name"]).node(alt_name)),
        )
    } else {
        (res, None)
    }
}

pub fn card_art_img(card: &Card, proxy: &Proxy) -> Vec<Element> {
    let mut side = card.side;
    let mut classes = vec!["art"];

    if proxy.layout() == &CardLayout::Flip && card.side == Side::B {
        if let Some(Art { scryfall: true, .. }) = get_side(Side::A, &proxy.arts) {
            side = Side::A;
        }
    }

    let Some(art) = get_side(side, &proxy.arts) else {
        return vec![];
    };

    if art.full {
        classes.push("full-art");
    }
    if card.face_layout().is_vertical() {
        classes.push("vertical")
    }

    let mut res = vec![];

    if !art.url.is_empty() {
        res.push(Element::new(Tag::img).class(classes).attr("src", &art.url));
    }
    if !art.credit.is_empty() {
        res.push(
            Element::new(Tag::span)
                .class(["art-credits"])
                .node(&art.credit),
        );
    }

    res
}

pub fn type_line_div(card: &Card, proxy: &Proxy) -> Element {
    let mut classes = vec!["type-line", "bar"];

    if card.face_layout().is_vertical() {
        classes.push("bottom");
    }

    if let Some(Art { full: true, .. }) = get_side(card.side, &proxy.arts) {
        classes.push("bottom");
    }

    Element::new(Tag::div)
        .class(classes)
        .node(color_indicator_span(card, proxy))
        .node(type_line_span(card, proxy))
}

pub fn type_line_span(card: &Card, proxy: &Proxy) -> Element {
    let mut type_line = &card.type_line;

    if let Some(c) = get_side(card.side, &proxy.customize) {
        if !c.type_line.is_empty() {
            type_line = &c.type_line;
        }
    }

    Element::new(Tag::span).class(["type"]).node(type_line)
}

pub fn color_indicator_span(card: &Card, proxy: &Proxy) -> Element {
    Element::new(Tag::span).class(["indicator"]).nodes(
        WUBRG::render(&card.colors)
            .to_lowercase()
            .chars()
            .map(|c| Element::new(Tag::i).class(vec!["ms".s(), format!("ms-{}", c)]))
            .collvect(),
    )
}

pub fn anchor_words(words: &str) -> Vec<Node> {
    vec![
        Element::new(Tag::span)
            .class(["anchor-word"])
            .node(words)
            .into(),
        " \u{2014} ".into(),
    ]
}

pub fn card_css_class(card: &Card) -> Vec<&'static str> {
    if card.types.contains(&Type::Land) {
        card.color_identity
            .iter()
            .map(WUBRG::name)
            .chain(["colorless"])
            .collvect()
    } else {
        card.colors.iter().map(WUBRG::name).collvect()
    }
}

pub fn rules_text_filter(proxy: &Proxy) -> fn(&str) -> Vec<Node> {
    if proxy.reminder_text {
        |s| replace_symbols::<ReminderText>(s).concat()
    } else {
        |s| replace_symbols::<NoReminderText>(s).concat()
    }
}

pub fn get_side<T>(side: Side, v: &Vec<T>) -> Option<&T> {
    match side {
        Side::A => v.get(0),
        Side::B => v.get(1),
        _ => None,
    }
}
