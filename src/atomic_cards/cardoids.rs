use serde::{Deserialize, Deserializer, Serialize};

use crate::proxy::deserializers::OneOrMany;
use crate::utils::iter::IterExt;

use super::{
    cards::Card,
    types::{CardLayout, Side, WUBRG},
};

use std::{collections::BTreeSet, fmt::Display};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Cardoid(Vec<Card>);

impl Cardoid {
    pub fn one_or_many<'de, D>(de: D) -> Result<Cardoid, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Cardoid(OneOrMany::<Card>::deserialize(de)?.into()))
    }
}

impl From<Vec<Card>> for Cardoid {
    fn from(value: Vec<Card>) -> Self {
        Self(value)
    }
}

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

    pub fn color_identity(&self) -> &BTreeSet<WUBRG> {
        &self.face().color_identity
    }

    pub fn side(&self, side: Side) -> Option<&Card> {
        self.0.iter().find(|c| c.side == side)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn name(&self) -> &str {
        &self.face().name
    }

    pub fn face(&self) -> &Card {
        &self.0[0]
    }

    pub fn layout(&self) -> &CardLayout {
        &self.face().layout
    }

    pub fn printed_cards(&self) -> usize {
        match self.layout() {
            CardLayout::ModalDfc | CardLayout::Transform | CardLayout::Flip => 2,
            _ => 1,
        }
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

impl Display for Cardoid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face = self.face();
        if let Some(b_side) = self.side(Side::B) {
            write!(f, "{}", &face.name)?;
            write!(f, "\nSIDE A\n")?;
            face.fmt(f)?;
            write!(f, "\nSIDE B\n")?;
            b_side.fmt(f)?;
        } else {
            face.fmt(f)?;
        }
        Ok(())
    }
}
