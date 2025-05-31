use serde::{Deserialize, Serialize};

use crate::utils::iter::IterExt;

use super::{cards::*, metadata::*, types::*};

use std::{collections::BTreeSet, fmt::Display};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Cardoid(Vec<Card>);

impl Cardoid {
    pub fn iter(&self) -> <&Vec<Card> as IntoIterator>::IntoIter {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> <&mut Vec<Card> as IntoIterator>::IntoIter {
        self.0.iter_mut()
    }

    pub fn sides(&self) -> Vec<Side> {
        self.0.iter().map(|c| c.side.clone()).collvect()
    }

    pub fn side(&self, side: Side) -> Option<&Card> {
        self.0.iter().find(|c| c.side == side)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn face(&self) -> &Card {
        &self.0[0]
    }

    pub fn layout(&self) -> &CardLayout {
        &self.face().layout
    }

    pub fn printed_cards(&self) -> usize {
        match self.layout() {
            CardLayout::ModalDfc | CardLayout::Transform => 2,
            _ => 1,
        }
    }
}

impl Display for Cardoid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face = self.face();
        if let Some(b_side) = self.side(Side::B) {
            f.write_fmt(format_args!("> {}", face.name));

            f.write_str("\n> **SIDE A**\n")?;
            Display::fmt(&(&face), f)?;
            f.write_str("\n> **SIDE B**\n")?;
            Display::fmt(&b_side, f)?;
        } else {
            Display::fmt(&face, f)?;
        }
        return Ok(());
    }
}

impl IntoIterator for Cardoid {
    type Item = Card;

    type IntoIter = <Vec<Card> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Cardoid {
    type Item = &'a Card;

    type IntoIter = <&'a Vec<Card> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Cardoid {
    type Item = &'a mut Card;

    type IntoIter = <&'a mut Vec<Card> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}
