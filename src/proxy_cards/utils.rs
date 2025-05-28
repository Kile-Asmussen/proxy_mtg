use crate::atomic_cards::{Card, CardType};

pub fn color_css_class(card: &Card) -> String {
    if card.types.contains(&CardType::Land) {
        return card
            .color_identity
            .iter()
            .map(|s| map_color(s))
            .chain(["colorless".to_string()])
            .collect::<Vec<_>>()
            .join(" ");
    } else {
        return card
            .colors
            .iter()
            .map(|s| map_color(s))
            .collect::<Vec<_>>()
            .join(" ");
    }

    fn map_color(initial: &str) -> String {
        match initial {
            "W" => "white".to_owned(),
            "U" => "blue".to_owned(),
            "B" => "black".to_owned(),
            "R" => "red".to_owned(),
            "G" => "green".to_owned(),
            _ => panic!(),
        }
    }
}
