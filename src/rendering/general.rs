use crate::{
    atomic_cards::{cards::Card, metadata::ForeignData, types::*},
    html::*,
    proxy::{Art, Proxy},
    rendering::{
        manafont::ManaFontSymbolics,
        reminders::{NoReminderText, ReminderText},
    },
    utils::{
        iter::IterExt,
        symbolics::{replace_symbols, replace_symbols_with},
    },
};

pub fn blank_card() -> Element {
    Element::new(Tag::div).class(["card"])
}

pub fn empty_card(card: &Card, proxy: &Proxy) -> Element {
    blank_card()
        .class(card_css_class(card))
        .node(title_bar_div(card, proxy))
}

pub fn raw_card(card: &Card, proxy: &Proxy) -> Element {
    empty_card(card, proxy)
        .node(type_line_div(card, proxy))
        .nodes(card_art_img(card, proxy))
        .node(type_line_div(card, proxy))
}

pub fn title_bar_div(card: &Card, proxy: &Proxy) -> Element {
    Element::new(Tag::div)
        .class(["title", "bar"])
        .node(card_name_span(card, proxy))
        .node(mana_cost_span(card, proxy))
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
        [Element::new(Tag::hr)]
            .into_iter()
            .chain(flavor_text.lines().map(|s| flavor_text_paragraph([s])))
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

pub fn card_name_span(card: &Card, proxy: &Proxy) -> Element {
    let mut name = card.face_name.clone();
    if name.is_empty() {
        name = card.name.clone();
    }

    if let Some(c) = get_side(card.side, &proxy.customize) {
        if !c.name.is_empty() {
            name = c.name.clone();
        } else if !c.face_name.is_empty() {
            name = c.name.clone();
        }
    }

    Element::new(Tag::span).class(["name"]).node(name)
}

pub fn card_art_img(card: &Card, proxy: &Proxy) -> Vec<Node> {
    if let Some(art) = get_side(card.side, &proxy.arts) {
        let mut classes = vec!["art"];
        if art.full {
            classes.push("full-art");
        }
        if card.face_layout().is_vertical() {
            classes.push("vertical")
        }

        vec![
            Node::Element(Element::new(Tag::img).class(classes).attr("src", &art.url)),
            Node::Element(
                Element::new(Tag::span)
                    .class(["art-credits"])
                    .node(&art.credit),
            ),
        ]
    } else {
        vec![]
    }
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
