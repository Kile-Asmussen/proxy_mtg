pub mod iter;
pub mod symbolics;
pub mod vec;

pub trait ToS: ToString {
    fn s(&self) -> String {
        self.to_string()
    }
}

impl<S> ToS for S where S: ToString {}

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
