use clap::builder::Str;

use crate::atomic_cards::{Card, CardType, WUBRG};

pub fn card_css_class(card: &Card) -> String {
    let (colors, extra) = if card.types.contains(&CardType::Land) {
        (&card.color_identity, vec!["colorless", "card"])
    } else {
        (&card.colors, vec!["card"])
    };
    return colors
        .iter()
        .map(WUBRG::name)
        .chain(extra.into_iter())
        .collect::<Vec<_>>()
        .join(" ");
}
