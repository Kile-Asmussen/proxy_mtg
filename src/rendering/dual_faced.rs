use crate::{
    atomic_cards::{
        cards::Card,
        types::{CardLayout, FaceLayout, Side, Type},
    },
    html::{Element, Tag},
    proxy::Proxy,
    rendering::{
        general::{blank_card, empty_card},
        landscapes::battle_card,
        normal::{creature_card, planeswalker_card, unadorned_card},
        verticalia::{class_card, saga_card},
    },
};

pub fn flip_layout_proxy(proxy: &Proxy) -> Vec<Element> {
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

pub fn genuine_dual_face_proxy(proxy: &Proxy) -> Vec<Element> {
    let (Some(up), Some(down)) = (proxy.cardoid.side(Side::A), proxy.cardoid.side(Side::B)) else {
        return vec![blank_card(), blank_card()];
    };

    let layout = match proxy.cardoid.layout() {
        CardLayout::ModalDfc => "modal",
        CardLayout::Transform => "transform",
        _ => "",
    };

    vec![
        card_by_face_layout(up, proxy)
            .class(["obverse", layout])
            .node(dual_face_indicator(&down.face_name)),
        card_by_face_layout(down, proxy)
            .class(["reverse", layout])
            .node(dual_face_indicator(&up.face_name)),
    ]
}

fn card_by_face_layout(card: &Card, proxy: &Proxy) -> Element {
    match card.face_layout() {
        FaceLayout::Creature => creature_card(card, proxy),
        FaceLayout::Planeswalker => planeswalker_card(card, proxy),
        FaceLayout::Unadorned => unadorned_card(card, proxy),
        FaceLayout::Battle => battle_card(card, proxy),
        FaceLayout::Class => class_card(card, proxy),
        FaceLayout::Saga => saga_card(card, proxy),
        _ => empty_card(card, proxy),
    }
}

fn dual_face_indicator(name: &str) -> Element {
    Element::new(Tag::div)
        .class(["dual-face-indicator", "bar"])
        .node(Element::new(Tag::span).node(name))
}
