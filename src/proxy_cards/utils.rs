use std::{marker::PhantomData, ops::IndexMut};

use build_html::HtmlElement;
use clap::builder::Str;

use crate::atomic_cards::{Card, CardType, WUBRG};

pub fn card_css_class(card: &Card) -> Vec<&str> {
    let (colors, extra) = if card.types.contains(&CardType::Land) {
        (&card.color_identity, vec!["colorless", "card"])
    } else {
        (&card.colors, vec!["card"])
    };
    return colors
        .iter()
        .map(WUBRG::name)
        .chain(extra.into_iter())
        .collect::<Vec<_>>();
}

pub trait HtmlElementExt: Sized {
    fn with_classses<SS, S>(mut self, ss: SS) -> Self
    where
        SS: IntoIterator<Item = S>,
        S: ToString,
    {
        self.add_classes(ss);
        self
    }

    fn add_classes<SS, S>(&mut self, ss: SS)
    where
        SS: IntoIterator<Item = S>,
        S: ToString;
}

impl HtmlElementExt for HtmlElement {
    fn add_classes<SS, S>(&mut self, ss: SS)
    where
        SS: IntoIterator<Item = S>,
        S: ToString,
    {
        let strings = ss.into_iter().map(|s| s.to_string());

        if let Some((_, v)) = self.attributes.iter().find(|(k, v)| k == "class") {
            self.attributes.
        } else {
            self.add_attribute("class", strings.collect::<Vec<_>>().join(" "));
        }
    }
}
