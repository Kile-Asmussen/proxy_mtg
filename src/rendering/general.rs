use crate::{
    atomic_cards::{cards::Card, types::*},
    html::*,
    proxy::{Art, Customization, Proxy, TextStyle},
    rendering::{
        manafont::ManaFontSymbolics,
        notation::{NoReminderText, PowerToughnessNobreak, ReminderText},
    },
    utils::{
        iter::IterExt,
        symbolics::{replace_symbols, replace_symbols_with, Symchain},
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
    let Some(Customization {
        flavor_text: Some(flavor_text),
        ..
    }) = get_side(card.side, &proxy.customize)
    else {
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
    let mut name = &card.face_name;
    let mut alt_name = &String::new();
    if name.is_empty() {
        name = &card.name;
    }

    if let Some(Customization {
        name: Some(cname), ..
    }) = get_side(card.side, &proxy.customize)
    {
        alt_name = name;
        name = cname;
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

pub fn type_line_div(card: &Card, proxy: &Proxy) -> Element {
    let mut classes = vec!["type-line".s(), "bar".s()];

    if card.face_layout().is_vertical() {
        classes.push("bottom".s());
    }

    if let Some(Art { full, .. }) = get_side(card.side, &proxy.arts) {
        if *full && !classes.contains(&"bottom".s()) {
            classes.push("bottom".s());
        }
    }

    Element::new(Tag::div)
        .class(classes)
        .node(color_indicator_span(card, proxy))
        .node(type_line_span(card, proxy))
}

pub fn type_line_span(card: &Card, proxy: &Proxy) -> Element {
    let mut type_line = &card.type_line;

    if let Some(Customization {
        type_line: Some(ctype_line),
        ..
    }) = get_side(card.side, &proxy.customize)
    {
        type_line = ctype_line;
    }

    Element::new(Tag::span).class(["type"]).node(type_line)
}

pub fn color_indicator_span(card: &Card, _proxy: &Proxy) -> Element {
    Element::new(Tag::span).class(["indicator"]).nodes(
        if !card.colors.iter().all(|c| card.mana_cost.contains(&c.s()))
            || card.layout == CardLayout::Token
        {
            Some(Element::new(Tag::i).class(vec![
                "ms".s(),
                "ms-ci".s(),
                format!("ms-ci-{}", card.colors.len()),
                format!("ms-ci-{}", WUBRG::render(&card.colors).to_lowercase()),
            ]))
        } else {
            None
        },
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
    let mut res = if card.face_layout().is_landscape() {
        vec!["landscape"]
    } else {
        vec!["portrait"]
    };

    if card.types.contains(&Type::Land) {
        res.append(&mut card.color_identity.iter().map(WUBRG::name).collvect());
    } else {
        res.append(&mut card.colors.iter().map(WUBRG::name).collvect());
    }

    res
}

pub fn text_style(card: &Card, proxy: &Proxy, default: Vec<TextStyle>) -> Vec<TextStyle> {
    if let Some(Customization {
        text_style: Some(text_style),
        ..
    }) = get_side(card.side, &proxy.customize)
    {
        text_style.clone()
    } else {
        default
    }
}

pub fn rules_text_filter(proxy: &Proxy) -> fn(&str) -> Vec<Node> {
    if proxy.reminder_text {
        replace_symbols::<Symchain<ReminderText, Symchain<PowerToughnessNobreak, ManaFontSymbolics>>>
    } else {
        replace_symbols::<
            Symchain<NoReminderText, Symchain<PowerToughnessNobreak, ManaFontSymbolics>>,
        >
    }
}

pub fn get_side<T>(side: Side, v: &Vec<T>) -> Option<&T> {
    match side {
        Side::A => v.get(0),
        Side::B => v.get(1),
        _ => None,
    }
}
