mod elements;
mod tags;
mod tests;
pub use elements::*;
use std::{collections::BTreeMap, fmt::Display};
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

    pub fn body(mut self, node: Node) -> Self {
        self.body.push(node);
        self
    }

    pub fn into_element(self) -> Element {
        Element::new(Tag::html)
            .elem(Element::new(Tag::head).nodes(self.head))
            .elem(Element::new(Tag::body).nodes(self.body))
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<!DOCTYPE html>")?;
        self.clone().into_element().fmt(f)
    }
}

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Element(element) => element.fmt(f),
            Node::Text(text) => f.write_str(text),
        }
    }
}
