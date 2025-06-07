mod elements;
mod node;
mod tags;
mod tests;
pub use elements::*;
pub use node::*;
use std::{collections::BTreeMap, fmt::Display, path::Path};
pub use tags::*;

use crate::utils::iter::IterExt;

#[derive(Clone)]
pub struct Document {
    head: Vec<Node>,
    body: Vec<Node>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            head: vec![],
            body: vec![],
        }
    }

    pub fn head(mut self, elem: Element) -> Self {
        self.head.push(Node::Element(elem));
        self
    }

    pub fn title<S>(mut self, title: S) -> Self
    where
        S: ToString,
    {
        self.head(Element::new(Tag::title).text(title))
    }

    pub fn head_link<S>(mut self, rel: &'static str, href: S) -> Self
    where
        S: ToString,
    {
        self.head(Element::new(Tag::link).attr("rel", rel).attr("href", href))
    }

    pub fn body(mut self, node: Node) -> Self {
        self.body.push(node);
        self
    }

    pub fn into_element(self) -> Element {
        Element::new(Tag::html)
            .elem(Element::new(Tag::head).nodes(self.head))
            .elem(Element::new(Tag::body).nodes(self.body))
    }

    pub fn inline_style<P>(self, path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(self.head(
            Element::new(Tag::style)
                .text("<![CDATA[\n")
                .text(std::fs::read_to_string(path)?)
                .text("\n]]>"),
        ))
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<!DOCTYPE html>")?;
        self.clone().into_element().fmt(f)
    }
}
