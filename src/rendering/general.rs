use crate::{
    atomic_cards::{cards::Card, metadata::ForeignData, types::*},
    html::*,
    proxy::Proxy,
    rendering::manafont::ManaFontSymbolics,
    utils::{iter::IterExt, symbolics::replace_symbols},
};

pub fn empty_card(card: &Card, proxy: &Proxy) -> Element {
    Element::new(Tag::div)
        .class(card_css_class(card))
        .node(title_bar_div(card, proxy))
}

pub fn title_bar_div(card: &Card, proxy: &Proxy) -> Element {
    Element::new(Tag::div)
        .class(["title", "bar"])
        .node(card_name_span(card, proxy))
        .node(mana_cost_span(card))
}

pub fn mana_cost_span(card: &Card) -> Element {
    Element::new(Tag::span)
        .class(["cost"])
        .nodes(replace_symbols(&ManaFontSymbolics, &card.mana_cost))
}

pub fn card_name_span(card: &Card, proxy: &Proxy) -> Element {
    let mut name = card.name.clone();

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
        vec![
            Node::Element(
                Element::new(Tag::img)
                    .class(if art.full {
                        vec!["art", "full-art"]
                    } else {
                        vec!["art"]
                    })
                    .attr("src", &art.url),
            ),
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
    let on_bottom = match card.face_layout() {
        FaceLayout::SagaCreature | FaceLayout::Saga | FaceLayout::Case | FaceLayout::Class => true,
        _ => false,
    } || get_side(card.side, &proxy.arts)
        .map(|a| a.full)
        .unwrap_or(false);

    Element::new(Tag::div)
        .class(if on_bottom {
            vec!["type-line", "bar", "bottom"]
        } else {
            vec!["type-line", "bar"]
        })
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

pub fn card_css_class(card: &Card) -> Vec<&'static str> {
    if card.types.contains(&Type::Land) {
        card.color_identity
            .iter()
            .map(WUBRG::name)
            .chain(["colorless", "card"].into_iter())
            .collvect()
    } else {
        card.colors
            .iter()
            .map(WUBRG::name)
            .chain(["card"].into_iter())
            .collvect()
    }
}

pub fn flavor_text(card: &Card, proxy: &Proxy) -> Vec<Element> {
    let Some(ForeignData { flavor_text, .. }) = get_side(card.side, &proxy.customize) else {
        return vec![];
    };

    flavor_text
        .lines()
        .map(|s| Element::new(Tag::p).class(["flavor-text"]).node(s))
        .collvect()
}

pub fn get_side<T>(side: Side, v: &Vec<T>) -> Option<&T> {
    match side {
        Side::A => v.get(0),
        Side::B => v.get(1),
        _ => None,
    }
}

pub fn cost_symbol<S>(class: S) -> Element
where
    S: AsRef<str>,
{
    Element::new(Tag::i).class(["ms", class.as_ref(), "ms-cost", "ms-shadow"])
}
