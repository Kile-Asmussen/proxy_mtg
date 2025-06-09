use crate::{
    atomic_cards::types::FaceLayout,
    html::Element,
    proxy::Proxy,
    rendering::{
        general::raw_card,
        normal::{creature_card, unadorned_card},
    },
};

pub fn token_proxy(proxy: &Proxy) -> Vec<Element> {
    let card = proxy.cardoid.face();

    vec![match card.face_layout() {
        FaceLayout::Creature => creature_card(card, proxy),
        FaceLayout::Unadorned => unadorned_card(card, proxy),
        _ => raw_card(card, proxy),
    }
    .class(["token"])]
}
