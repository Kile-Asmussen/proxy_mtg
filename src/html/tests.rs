#[test]
fn test_element() {
    use crate::html::{Document, Element, Tag};
    let doc = Document::new()
        .head(
            Element::new(Tag::link)
                .attr("rel", "stylesheet")
                .attr("href", "foo.css"),
        )
        .body(Element::new(Tag::p).node("Hello, world!"));

    assert_eq!(format!("{}", doc), "<!DOCTYPE html><html><head><link rel='stylesheet' href='foo.css'></head><body><p>Hello, world!</p></body></html>")
}
