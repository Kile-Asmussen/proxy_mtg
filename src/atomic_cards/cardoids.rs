use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    atomic_cards::{
        cards::Card_Keys,
        sqlite::{db_column, SqliteTable},
        types::WUBRG,
    },
    proxy::deserializers::OneOrMany,
};

#[cfg(test)]
use crate::utils::ToS;

use super::{
    cards::Card,
    types::{CardLayout, Side},
};
use rusqlite::{Connection, ToSql};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[serde(transparent)]
pub struct Cardoid(Vec<Card>);

#[derive(Default, PartialEq, Eq, Clone, Debug)]
#[allow(non_camel_case_types, unused)]
pub struct Cardoid_Keys {
    pub card_name: String,
}

impl SqliteTable for Cardoid {
    type Keys = Cardoid_Keys;

    const COLUMNS: &'static [super::sqlite::DbColumn<Cardoid, Cardoid_Keys>] =
        &[db_column!(UNIQUE key.card_name "TEXT NOT NULL", val.as_str())];

    fn extra_setup(conn: &Connection) -> anyhow::Result<()> {
        Card::setup(conn)?;
        Ok(())
    }

    fn load(&mut self, id: i64, _key: &Self::Keys, conn: &Connection) -> anyhow::Result<()> {
        let cards = Card::load_keys(
            [&Card_Keys {
                legalities: None,
                cardoid: id,
            }],
            conn,
        )?;

        self.0 = cards.into_iter().map(|(_, c, _)| c).collect_vec();

        Ok(())
    }

    fn post_store(&self, id: i64, conn: &Connection) -> anyhow::Result<()> {
        let mut data = self
            .0
            .iter()
            .map(|c| {
                (
                    c,
                    Card_Keys {
                        legalities: None,
                        cardoid: id,
                    },
                )
            })
            .collect_vec();

        Card::store_rows(data.iter_mut().map(|(c, ck)| (*c, ck)), &conn)?;

        Ok(())
    }
}

#[test]
fn test_cardoid() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;
    Cardoid::setup(&conn)?;

    let mut data = vec![(
        Cardoid(vec![Card::default()]),
        Cardoid_Keys {
            card_name: "FooBar".s(),
        },
    )];

    let ids = Cardoid::store_rows(data.iter_mut().map(|(c, ck)| (&*c, ck)), &conn)?;

    let datas = Cardoid::load_rows(ids, &conn)?
        .into_iter()
        .map(|(_, c, ck)| (c, ck))
        .collect_vec();

    assert_eq!(data, datas);

    Ok(())
}

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
        self.0.iter().map(|c| c.side.clone()).collect_vec()
    }

    pub fn color_identity(&self) -> &WUBRG {
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

#[allow(unused)]
impl Cardoid {
    const CREATE_TABLE: &'static str = r##"
        CREATE TABLE Cardoid (
            id INTEGER NOT NULL PRIMARY KEY,
            name INTEGER NOT NULL,
            side_a INTEGER NOT NULL,
            side_b INTEGER,
            side_c INTEGER,
            side_d INTEGER,
            side_e INTEGER
        );
    "##;
}
