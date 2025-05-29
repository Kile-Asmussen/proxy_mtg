use std::{marker::PhantomData, mem, ops::IndexMut};

use build_html::HtmlElement;
use clap::builder::Str;

use crate::{
    atomic_cards::{cards::Card, types::*},
    vec_entry::{IterExt, VecEntryExt, VecEntryMethods},
};

pub fn card_css_class(card: &Card) -> Vec<&str> {
    let (colors, extra) = if card.types.contains(&Type::Land) {
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
        let mut strings = ss
            .into_iter()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collvect();

        let class = self.attributes.entry("class".to_string()).or_default();

        if !class.is_empty() {
            strings.push(mem::replace(class, String::new()))
        }

        *class = strings.join(" ");
    }
}
