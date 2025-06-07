use std::{collections::BTreeMap, fmt::Display};

use crate::utils::{
    iter::IterExt,
    vec::{VecEntryMethods, VecExt},
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
                    k.to_string()
                } else {
                    format!("{}='{}'", k, v)
                }
            })
            .collvect();

        tag_content.insert(0, self.tag.name.to_string());

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

    pub fn text_len(&self) -> usize {
        self.nodes.iter().map(Node::text_len).sum()
    }

    pub fn len(&self) -> usize {
        self.tag.len()
            + self
                .attributes
                .iter()
                .map(|(k, v)| {
                    if k == v {
                        k.len() + 1
                    } else {
                        k.len() + v.len() + 4
                    }
                })
                .sum::<usize>()
            + self.nodes.iter().map(Node::len).sum::<usize>()
    }

    pub fn children(&self) -> &[Node] {
        &self.nodes[..]
    }

    pub fn atttributes(&self) -> &[(&'static str, String)] {
        &self.attributes[..]
    }

    pub fn attr<S>(mut self, k: &'static str, v: S) -> Self
    where
        S: ToString,
    {
        self.attributes.entry(k).insert_entry(v.to_string());
        self
    }

    pub fn flag(mut self, k: &'static str) -> Self {
        self.attr(k, k.to_string())
    }

    pub fn class<SS, S>(mut self, cls: SS) -> Self
    where
        SS: IntoIterator<Item = S>,
        S: ToString,
    {
        self.attr(
            "class",
            cls.into_iter().map(|s| s.to_string()).collvect().join(" "),
        )
    }

    pub fn text<S>(mut self, text: S) -> Self
    where
        S: ToString,
    {
        self.node(Node::Text(text.to_string()))
    }

    pub fn elem(mut self, elem: Element) -> Self {
        self.node(Node::Element(elem))
    }

    pub fn nodes<ES>(mut self, nodes: ES) -> Self
    where
        ES: IntoIterator<Item = Node>,
    {
        for e in nodes {
            self = self.node(e)
        }
        self
    }

    pub fn node(mut self, node: Node) -> Self {
        self.nodes.push(node);
        self
    }
}
