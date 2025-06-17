use std::fmt::Display;

use crate::utils::ToS;

use super::Element;

#[derive(Clone)]
pub enum Node {
    _Element { element: Element },
    _Text { text: String },
}

impl Node {
    pub fn is_text(&self) -> bool {
        if let Self::_Text { .. } = self {
            true
        } else {
            false
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::_Element { element } => element.fmt(f),
            Node::_Text { text } => {
                if f.alternate() {
                    f.write_str(text)
                } else {
                    f.write_str(&escape_html_text(text))
                }
            }
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
            text: text.as_ref().s(),
        }
    }
}

pub fn escape_html_text(s: &str) -> String {
    s.replace("<", "&lt;").replace(">", "&gt;")
}

pub fn escape_html_attr(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
