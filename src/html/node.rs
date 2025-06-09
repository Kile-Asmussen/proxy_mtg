use std::fmt::Display;

use crate::utils::ToS;

use super::Element;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

impl Node {
    pub fn text_len(&self) -> usize {
        match self {
            Node::Element(element) => element.text_len(),
            Node::Text(string) => string.len(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Node::Element(element) => element.len(),
            Node::Text(text) => text.len(),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Element(element) => element.fmt(f),
            Node::Text(text) => f.write_str(text),
        }
    }
}

impl From<Element> for Node {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}

impl<S> From<S> for Node
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self::Text(value.as_ref().s())
    }
}
