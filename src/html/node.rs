use std::fmt::Display;

use super::Element;

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
