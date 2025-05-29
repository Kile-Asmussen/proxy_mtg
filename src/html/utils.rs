use std::{marker::PhantomData, mem, ops::IndexMut};

use build_html::{HtmlChild, HtmlElement};
use clap::builder::Str;

use crate::{
    atomic_cards::{cards::Card, types::*},
    utils::{
        iter::IterExt,
        vec::{VecEntryMethods, VecExt},
    },
};

pub trait HtmlExt: Sized {
    fn with_classes<SS, S>(mut self, ss: SS) -> Self
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

    fn with_child_element(mut self, ele: HtmlElement) -> Self {
        self.add_child_element(ele);
        self
    }

    fn add_child_element(&mut self, ele: HtmlElement);
}

impl HtmlExt for HtmlElement {
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

    fn add_child_element(&mut self, ele: HtmlElement) {
        self.add_child(HtmlChild::Element(ele));
    }
}
