use crate::{
    atomic_cards::types::{Side, Type},
    html::{Element, Tag},
    proxy::Proxy,
    rendering::{
        general::blank_card,
        normal::{creature_card, unadorned_card},
    },
};

pub fn flip_layout_card(proxy: &Proxy) -> Vec<Element> {
    let (Some(up), Some(down)) = (proxy.cardoid.side(Side::A), proxy.cardoid.side(Side::B)) else {
        return vec![blank_card(), blank_card()];
    };

    vec![
        if up.types.contains(&Type::Creature) {
            creature_card(up, proxy)
        } else {
            unadorned_card(up, proxy)
        }
        .class(["obverse", "flip"])
        .node(dual_face_indicator(&down.face_name)),
        if down.types.contains(&Type::Creature) {
            creature_card(down, proxy)
        } else {
            unadorned_card(down, proxy)
        }
        .class(["reverse", "flip"])
        .node(dual_face_indicator(&up.face_name)),
    ]
}

fn dual_face_indicator(name: &str) -> Element {
    Element::new(Tag::div)
        .class(["dual-face-indicator", "bar"])
        .node(Element::new(Tag::span).node(name))
}
