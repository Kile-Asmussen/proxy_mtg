use std::fmt::Display;

use crate::utils::{
    escape_html_attr,
    iter::IterExt,
    vec::{VecEntryMethods, VecExt},
    ToS,
};

use super::{Node, Tag};

#[derive(Clone)]
pub struct Element {
    pub tag: Tag,
    attributes: Vec<(&'static str, String)>,
    nodes: Vec<Node>,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tag_content = self
            .attributes
            .iter()
            .map(|(k, v)| {
                if k == v {
                    k.s()
                } else {
                    format!("{}='{}'", k, v)
                }
            })
            .collvect();

        tag_content.insert(0, self.tag.name.s());

        f.write_fmt(format_args!("<{}>", tag_content.join(" ")))?;

        if !self.tag.void {
            for node in &self.nodes {
                node.fmt(f)?
            }

            f.write_fmt(format_args!("</{}>", self.tag.name))?
        }

        Ok(())
    }
}

impl Element {
    pub fn new(tag: Tag) -> Self {
        Element {
            tag,
            attributes: vec![],
            nodes: vec![],
        }
    }

    pub fn attr<S>(mut self, k: &'static str, v: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attributes
            .entry(k)
            .insert_entry(escape_html_attr(v.as_ref()));
        self
    }

    pub fn flag(self, k: &'static str) -> Self {
        self.attr(k, k.s())
    }

    pub fn class<SS, S>(mut self, classes: SS) -> Self
    where
        SS: IntoIterator<Item = S>,
        S: ToString,
    {
        let entry = self.attributes.entry("class").or_insert("".s());

        if !entry.is_empty() {
            *entry += " ";
        }

        *entry += &classes.into_iter().map(|s| s.s()).collvect().join(" ");

        self
    }

    pub fn nodes<NS, N>(mut self, nodes: NS) -> Self
    where
        NS: IntoIterator<Item = N>,
        N: Into<Node>,
    {
        for e in nodes {
            self = self.node(e)
        }
        self
    }

    pub fn node<N>(mut self, node: N) -> Self
    where
        N: Into<Node>,
    {
        self.nodes.push(node.into());
        self
    }
}
