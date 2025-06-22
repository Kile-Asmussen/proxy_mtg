use indexmap::IndexSet;
use itertools::Itertools;
use rusqlite::types::{ToSqlOutput, Value};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::atomic_cards::metadata::ForeignData_Keys;
use crate::atomic_cards::sqlite::{db_column, SqliteTable, SqliteTableImpl};

use crate::utils::ToS;
use rusqlite::ToSql;

use super::sqlite;
use super::types::LeadershipSkills;
use super::{
    is_default,
    metadata::{ForeignData, Legalities},
    types::{CardLayout, FaceLayout, Side, Supertype, Type, WUBRG},
};

use std::fmt::Display;

#[cfg(test)]
use crate::atomic_cards::metadata::Legality;
#[cfg(test)]
use crate::atomic_cards::types::Pie;
#[cfg(test)]
use std::collections::BTreeSet;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct Card {
    #[serde(default, skip_serializing_if = "is_default", rename = "colorIdentity")]
    pub color_identity: WUBRG,
    #[serde(default, skip_serializing_if = "is_default", rename = "colorIndicator")]
    pub color_indicator: WUBRG,
    pub colors: WUBRG,
    #[serde(default, skip_serializing_if = "is_default")]
    pub defense: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "faceManaValue")]
    pub face_mana_value: Option<f64>,
    #[serde(default, skip_serializing_if = "is_default", rename = "faceName")]
    pub face_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "foreignData")]
    pub foreign_data: Vec<ForeignData>,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "hasAlternativeDeckLimit"
    )]
    pub has_alternative_deck_limit: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub keywords: IndexSet<String>,
    pub layout: CardLayout,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "leadershipSkills"
    )]
    pub leadership_skills: LeadershipSkills,
    #[serde(default, skip_serializing_if = "is_default")]
    pub legalities: Legalities,
    #[serde(default, skip_serializing_if = "is_default")]
    pub loyalty: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "manaCost")]
    pub mana_cost: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "manaValue")]
    pub mana_value: f64,
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub power: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub side: Side,
    #[serde(default, skip_serializing_if = "is_default")]
    pub subtypes: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub supertypes: Vec<Supertype>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub text: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub toughness: String,
    #[serde(default, skip_serializing_if = "is_default", rename = "type")]
    pub type_line: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub types: Vec<Type>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card_Keys {
    pub cardoid: i64,
    pub legalities: Option<i64>,
}

impl Default for Card_Keys {
    fn default() -> Self {
        Self {
            cardoid: -1,
            legalities: None,
        }
    }
}

impl SqliteTable for Card {
    type Keys = Card_Keys;

    const COLUMNS: &'static [sqlite::DbColumn<Card, Card_Keys>] = &[
        db_column!(object.colors "TEXT NOT NULL", val -> val.as_str()?.into()),
        db_column!(object.color_identity "TEXT NOT NULL", val -> val.as_str()?.into()),
        db_column!(object.color_indicator "TEXT NOT NULL", val -> val.as_str()?.into()),
        db_column!(object.defense "TEXT NOT NULL", val.as_str()),
        db_column!(object.face_name "TEXT NOT NULL", val.as_str()),
        db_column!(object.face_mana_value "REAL", val.as_f64_or_null()),
        db_column!(
            object.has_alternative_deck_limit "INTEGER NOT NULL",
            val -> val.as_i64()? != 0
        ),
        db_column!(
            object.keywords "TEXT NOT NULL",
            val -> Card::stringlist_load(val.as_str()?),
            val <- Value::Text(Card::stringlist_store(val))
        ),
        db_column!(object.layout "INTEGER NOT NULL", val.as_i64()),
        db_column!(object.leadership_skills "BLOB NOT NULL", val.as_blob()),
        db_column!(object.mana_cost "TEXT NOT NULL", val.as_str()),
        db_column!(object.mana_value "REAL NOT NULL", val.as_f64()),
        db_column!(INDEX object.name "TEXT NOT NULL", val.as_str()),
        db_column!(object.power "TEXT NOT NULL", val.as_str()),
        db_column!(object.side "INTEGER NOT NULL", val.as_i64()),
        db_column!(object.subtypes "TEXT NOT NULL",
            val -> Card::stringlist_load(val.as_str()?),
            val <- Value::Text(Card::stringlist_store(val))
        ),
        db_column!(object.supertypes "BOLB NOT NULL",
            val -> val.as_blob()?.into_iter().map(|b| (*b).into()).collect_vec(),
            val <- Value::Blob(val.iter().map(|s| *s as u8).collect_vec())
        ),
        db_column!(object.text "TEXT NOT NULL", val.as_str()),
        db_column!(object.toughness "TEXT NOT NULL", val.as_str()),
        db_column!(object.type_line "TEXT NOT NULL", val.as_str()),
        db_column!(object.types "BLOB NOT NULL",
            val -> val.as_blob()?.into_iter().map(|b| (*b).into()).collect_vec(),
            val <- Value::Blob(val.iter().map(|s| *s as u8).collect_vec())
        ),
        db_column!(key.cardoid "INTEGER NOT NULL", val.as_i64()),
        db_column!(NOINDEX key.legalities "INTEGER NOT NULL", val.as_i64()),
    ];

    fn extra_setup(conn: &Connection) -> anyhow::Result<()> {
        ForeignData::setup(conn)?;
        Legalities::setup(conn)?;
        Ok(())
    }

    fn load(
        &mut self,
        id: i64,
        key: &Self::Keys,
        conn: &rusqlite::Connection,
    ) -> anyhow::Result<()> {
        if let Some(id) = key.legalities {
            Legalities::load_rows([id], conn, |_, l, _| Ok(self.legalities = l))?;
        }

        ForeignData::load_keys([&ForeignData_Keys { parent_card: id }], conn, |_, f, _| {
            Ok(self.foreign_data.push(f))
        })?;

        Ok(())
    }

    fn pre_store(&self, key: &mut Self::Keys, conn: &rusqlite::Connection) -> anyhow::Result<()> {
        Legalities::store_rows(conn, |mut s| {
            Ok(key.legalities = Some(s.store(&self.legalities, &mut ())?))
        })?;

        Ok(())
    }

    fn post_store(&self, id: i64, conn: &Connection) -> anyhow::Result<()> {
        ForeignData::store_rows(conn, |mut s| {
            for f in &self.foreign_data {
                s.store(f, &mut ForeignData_Keys { parent_card: id })?;
            }
            Ok(())
        })?;

        Ok(())
    }
}

#[test]
fn card_tests() -> anyhow::Result<()> {
    let conn = rusqlite::Connection::open_in_memory()?;
    Card::setup(&conn)?;

    let mut data = vec![(
        Card {
            color_identity: WUBRG::wubrg(),
            color_indicator: WUBRG::colorless(),
            colors: WUBRG(BTreeSet::from_iter([Pie::R, Pie::G])),
            defense: "".s(),
            face_mana_value: None,
            face_name: "".s(),
            foreign_data: vec![ForeignData {
                face_name: "Få".s(),
                flavor_text: "".s(),
                language: "Danish".s(),
                name: "Foo // Bar".s(),
                text: "".s(),
                type_line: "Land".s(),
            }],
            has_alternative_deck_limit: false,
            keywords: IndexSet::from_iter(["Flying".s()]),
            layout: CardLayout::Normal,
            leadership_skills: LeadershipSkills {
                brawl: false,
                commander: true,
                oathbreaker: false,
            },
            legalities: {
                let mut x = Legalities::default();
                x.legacy = Legality::Legal;
                x
            },
            loyalty: "".s(),
            mana_cost: "{1}{W}".s(),
            mana_value: 2.0,
            name: "Foo".s(),
            power: "1".s(),
            side: Side::B,
            subtypes: vec!["Borb".s()],
            supertypes: vec![Supertype::Legendary],
            text: "Flying\nVery good boy.".s(),
            toughness: "2".s(),
            type_line: "Legendary Creature - Borb".s(),
            types: vec![Type::Creature],
        },
        Card_Keys {
            cardoid: -1,
            legalities: None,
        },
    )];

    let mut ids = vec![];
    Card::store_rows(&conn, |mut s| {
        for (c, ck) in &mut data {
            ids.push(s.store(c, ck)?);
        }
        Ok(())
    })?;

    let mut data2 = vec![];
    Card::load_rows(ids, &conn, |_, c, ck| Ok(data2.push((c, ck))))?;

    assert_eq!(data, data2);

    let mut legalities = vec![];
    Legalities::load_all(&conn, |_, l, _| Ok(legalities.push(l)))?;

    assert_eq!(&legalities[0], &data[0].0.legalities);

    Ok(())
}

impl Card {
    fn stringlist_load<S>(val: &str) -> S
    where
        S: FromIterator<String>,
    {
        if val.is_empty() {
            FromIterator::from_iter([])
        } else {
            let mut res = val.split("\n").map(|s| s.s()).collect_vec();
            res.pop();
            FromIterator::from_iter(res)
        }
    }
    fn stringlist_store(val: impl IntoIterator<Item = impl ToString>) -> String {
        let val = val.into_iter().map(|s| s.s()).collect_vec();
        if val.is_empty() {
            "".s()
        } else {
            val.into_iter().join("\n") + "\n"
        }
    }

    pub fn is_land(&self) -> bool {
        self.is_type(Type::Land)
    }
    pub fn is_basic(&self) -> bool {
        self.is_supertype(Supertype::Basic) && self.is_land()
    }
    pub fn is_spell(&self) -> bool {
        !self.is_type(Type::Land) && self.layout != CardLayout::Token
    }
    pub fn is_permanent(&self) -> bool {
        !self.is_instant() && !self.is_sorcery()
    }
    pub fn is_instant(&self) -> bool {
        self.is_type(Type::Instant)
    }
    pub fn is_sorcery(&self) -> bool {
        self.is_type(Type::Sorcery)
    }
    pub fn is_type(&self, t: Type) -> bool {
        self.types.contains(&t)
    }
    pub fn is_supertype(&self, t: Supertype) -> bool {
        self.supertypes.contains(&t)
    }
    pub fn is_subtype(&self, t: &str) -> bool {
        self.subtypes.iter().any(|s| s == t)
    }

    pub fn face_layout(&self) -> FaceLayout {
        match &self.layout {
            CardLayout::Adventure => FaceLayout::Omenventure,
            CardLayout::Aftermath => FaceLayout::Aftermath,
            CardLayout::Case => FaceLayout::Case,
            CardLayout::Class => FaceLayout::Class,
            CardLayout::Flip => FaceLayout::Flip,
            CardLayout::Leveler => FaceLayout::Leveler,
            CardLayout::Meld => self.guess_face_layout(),
            CardLayout::ModalDfc => self.guess_face_layout(),
            CardLayout::Mutate => FaceLayout::Mutate,
            CardLayout::Normal => self.guess_face_layout(),
            CardLayout::Prototype => FaceLayout::Prototype,
            CardLayout::ReversibleCard => self.guess_face_layout(),
            CardLayout::Saga => FaceLayout::Saga,
            CardLayout::Split if self.text.contains("Fuse") => FaceLayout::Fuse,
            CardLayout::Split => FaceLayout::Split,
            CardLayout::Token => self.guess_face_layout(),
            CardLayout::Transform => self.guess_face_layout(),
            CardLayout::Unsupported => FaceLayout::Unsupported,
        }
    }

    pub fn guess_face_layout(&self) -> FaceLayout {
        if self.is_basic() {
            FaceLayout::Basic
        } else if self.is_type(Type::Creature) {
            FaceLayout::Creature
        } else if self.is_subtype("Saga") {
            FaceLayout::Saga
        } else if self.is_type(Type::Planeswalker) {
            FaceLayout::Planeswalker
        } else if self.is_type(Type::Battle) {
            FaceLayout::Battle
        } else {
            FaceLayout::Unadorned
        }
    }

    pub fn get_name(&self) -> String {
        if self.face_name.is_empty() {
            self.name.clone()
        } else {
            self.face_name.clone()
        }
    }

    pub fn get_mana_value(&self) -> usize {
        if let Some(n) = self.face_mana_value {
            n as usize
        } else {
            self.mana_value as usize
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = &self.face_name;
        if name.is_empty() {
            name = &self.name;
        }
        write!(f, "{} {}", &name, &self.mana_cost)?;
        write!(f, "\n({}) {}", self.colors.to_string(), self.type_line)?;
        for line in self.text.lines() {
            write!(f, "\n{}", line)?;
        }
        if self.types.contains(&Type::Planeswalker) {
            write!(f, "\n[{}]", self.loyalty)?;
        }
        if self.types.contains(&Type::Battle) {
            write!(f, "\n<{}>", self.defense)?;
        }
        if self.types.contains(&Type::Creature) {
            write!(f, "\n{}/{}", self.power, self.toughness)?;
        }
        Ok(())
    }
}
