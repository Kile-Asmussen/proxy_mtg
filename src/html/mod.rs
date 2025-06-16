mod elements;
mod node;
mod tags;
mod tests;
pub use anyhow::anyhow;
// use css_minify::optimizations::{Level, Minifier};
pub use elements::*;
pub use node::*;
use std::{fmt::Display, path::Path};
pub use tags::*;

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

    pub fn head<N>(mut self, elem: N) -> Self
    where
        N: Into<Node>,
    {
        self.head.push(elem.into());
        self
    }

    pub fn title<S>(self, title: S) -> Self
    where
        S: AsRef<str>,
    {
        self.head(Element::new(Tag::title).node(title))
    }

    pub fn head_link<S>(self, rel: &'static str, href: S) -> Self
    where
        S: AsRef<str>,
    {
        self.head(
            Element::new(Tag::link)
                .attr("rel", rel)
                .attr("href", href.as_ref()),
        )
    }

    pub fn body<N>(mut self, node: N) -> Self
    where
        N: Into<Node>,
    {
        self.body.push(node.into());
        self
    }

    pub fn into_element(self) -> Element {
        Element::new(Tag::html)
            .node(Element::new(Tag::head).nodes(self.head))
            .node(Element::new(Tag::body).nodes(self.body))
    }

    pub fn inline_style<P>(self, path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let input =
            std::fs::read_to_string(&path).map_err(|e| anyhow!("{:?} - {}", path.as_ref(), e))?;

        // let minified = Minifier::default().minify(&input, Level::Zero);

        // let input = match minified {
        //     Ok(minified) => minified,
        //     Err(error) => {
        //         eprintln!(
        //             "Failure to minify {}: {}",
        //             path.as_ref().to_string_lossy(),
        //             error
        //         );
        //         input
        //     }
        // };

        Ok(self.head(
            Element::new(Tag::style)
                .node("/*<![CDATA[*/")
                .node(input)
                .node("/*]]>*/"),
        ))
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<!DOCTYPE html>")?;
        self.clone().into_element().fmt(f)
    }
}
