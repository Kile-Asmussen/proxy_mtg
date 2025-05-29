use std::fmt::Display;

use crate::html::{Document, Element, Node, Tag};

#[test]
fn test_element() {
    let doc = Document::new()
        .head(
            Element::new(Tag::link)
                .attr("rel", "stylesheet")
                .attr("href", "foo.css"),
        )
        .body(Node::Element(Element::new(Tag::p).text("Hello, world!")));

    assert_eq!(format!("{}", doc), "<!DOCTYPE html><html><head><link rel='stylesheet' href='foo.css'></head><body><p>Hello, world!</p></body></html>")
}
