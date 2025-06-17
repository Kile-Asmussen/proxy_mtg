use std::fmt::Display;

use crate::utils::escape_html_text;

use super::Element;

#[derive(Clone)]
pub enum Node {
    _Element { element: Element },
    _Text { text: String },
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::_Element { element } => element.fmt(f),
            Node::_Text { text } => f.write_str(text),
        }
    }
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::_Element { element }
    }
}

impl<S> From<S> for Node
where
    S: AsRef<str>,
{
    fn from(text: S) -> Self {
        Self::_Text {
            text: escape_html_text(text.as_ref()),
        }
    }
}
