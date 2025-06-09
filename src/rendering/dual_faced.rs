use crate::{
    atomic_cards::types::{FaceLayout, Side},
    html::Element,
    proxy::Proxy,
    rendering::{
        general::blank_card,
        normal::{creature_card, raw_card, unadorned_card},
    },
};

pub fn flip_layout_card(proxy: &Proxy) -> Vec<Element> {
    let (Some(up), Some(down)) = (proxy.cardoid.side(Side::A), proxy.cardoid.side(Side::B)) else {
        return vec![blank_card(), blank_card()];
    };

    println!("{} {}", &up.face_layout(), &down.face_layout());

    vec![
        match up.face_layout() {
            FaceLayout::Creature => creature_card(up, proxy),
            FaceLayout::Unadorned => unadorned_card(up, proxy),
            _ => raw_card(up, proxy),
        },
        match down.face_layout() {
            FaceLayout::Creature => creature_card(down, proxy),
            FaceLayout::Unadorned => unadorned_card(down, proxy),
            _ => raw_card(down, proxy),
        },
    ]
}
