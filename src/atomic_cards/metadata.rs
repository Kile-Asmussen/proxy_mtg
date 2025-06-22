#![allow(unused)]

use crate::{
    atomic_cards::sqlite::{db_column, DbColumn, SqliteTable},
    utils::ToS,
};

use super::is_default;
use itertools::Itertools;
use rusqlite::{
    params,
    types::{self, ToSqlOutput, Value},
    Connection, ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct MetaData {
    pub date: String,
    pub version: String,
}

impl SqliteTable for MetaData {
    type Keys = ();

    const COLUMNS: &'static [DbColumn<MetaData, ()>] = &[
        db_column!(object.date "TEXT NOT NULL", val.as_str()),
        db_column!(object.version "TEXT NOT NULL", val.as_str()),
    ];

    fn insert_row_stmt() -> String {
        let table_name = Self::table_name();
        format!("INSERT OR REPLACE INTO {table_name} (rowid, date, version) VALUES (1, :date, :version);")
    }

    fn select_row_stmt() -> String {
        let table_name = Self::table_name();
        format!(
            "SELECT rowid, date, version FROM {table_name} WHERE rowid = 1 AND :rowid = :rowid;"
        )
    }
}

#[test]
fn test_metadata() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;
    MetaData::setup(&conn)?;

    let m = MetaData {
        date: "today".s(),
        version: "0.9b".s(),
    };

    let r = MetaData::store_rows([(&m, &mut ())], &conn)?;
    assert_eq!(r, vec![1]);

    let ms = MetaData::load_rows(r, &conn)?;
    assert_eq!(ms, vec![(1, m, ())]);

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ForeignData {
    #[serde(default, rename = "faceName")]
    pub face_name: String,
    #[serde(default, rename = "flavorText")]
    pub flavor_text: String,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub text: String,
    #[serde(default, rename = "type")]
    pub type_line: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForeignData_Keys {
    pub parent_card: i64,
}

impl Default for ForeignData_Keys {
    fn default() -> Self {
        Self { parent_card: -1 }
    }
}

impl SqliteTable for ForeignData {
    type Keys = ForeignData_Keys;

    const COLUMNS: &'static [DbColumn<Self, Self::Keys>] = &[
        db_column!(key.parent_card "INTEGER NOT NULL", val.as_i64()),
        db_column!(object.face_name "TEXT NOT NULL", val.as_str()),
        db_column!(object.flavor_text "TEXT NOT NULL", val.as_str()),
        db_column!(object.language "TEXT NOT NULL", val.as_str()),
        db_column!(object.name "TEXT NOT NULL", val.as_str()),
        db_column!(object.text "TEXT NOT NULL", val.as_str()),
        db_column!(object.type_line "TEXT NOT NULL", val.as_str()),
    ];
}

#[test]
fn test_foreign_data() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;
    println!("{}", ForeignData::full_setup().join("\n"));
    ForeignData::setup(&conn)?;

    let mut data = vec![
        (
            ForeignData {
                face_name: "Få".s(),
                flavor_text: "".s(),
                language: "Danish".s(),
                name: "Foo // Bar".s(),
                text: "".s(),
                type_line: "Land".s(),
            },
            ForeignData_Keys { parent_card: 22 },
        ),
        (
            ForeignData {
                face_name: "Foue".s(),
                flavor_text: "".s(),
                language: "French".s(),
                name: "Foue // Barre".s(),
                text: "".s(),
                type_line: "Terre".s(),
            },
            ForeignData_Keys { parent_card: 22 },
        ),
        (
            ForeignData {
                face_name: "Queux".s(),
                flavor_text: "".s(),
                language: "French".s(),
                name: "Bazé // Queux".s(),
                text: "".s(),
                type_line: "Terre".s(),
            },
            ForeignData_Keys { parent_card: 23 },
        ),
    ];

    println!("{}", ForeignData::insert_row_stmt());

    let ids = ForeignData::store_rows(data.iter_mut().map(|(o, k)| (&*o, k)), &conn)?;

    println!("{}", ForeignData::select_row_stmt());

    let data2 = ForeignData::load_rows([1], &conn)?;

    assert_eq!(&data[0].0, &data2[0].1);
    assert_eq!(&data[0].1, &data2[0].2);
    assert_eq!(data2.len(), 1);

    println!("{}", ForeignData::select_all_stmt());

    let data3 = ForeignData::load_all(&conn)?
        .into_iter()
        .map(|(_, f, fk)| (f, fk))
        .collect_vec();

    assert_eq!(data, data3);

    println!("{}", ForeignData::select_keyed_stmt());

    let data4 = ForeignData::load_keys([&ForeignData_Keys { parent_card: 22 }], &conn)?;

    assert_eq!(
        &data[0..=1],
        data4.into_iter().map(|(_, f, fk)| (f, fk)).collect_vec()
    );

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Legalities {
    #[serde(default, skip_serializing_if = "is_default")]
    pub alchemy: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub brawl: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub commander: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub duel: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub explorer: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub future: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub gladiator: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub historic: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub historicbrawl: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub legacy: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub modern: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub oldschool: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pauper: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub penny: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pioneer: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub predh: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub premodern: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub standard: Legality,
    #[serde(default, skip_serializing_if = "is_default")]
    pub vintage: Legality,
}

impl SqliteTable for Legalities {
    type Keys = ();

    const COLUMNS: &'static [DbColumn<Self, Self::Keys>] = &[
        db_column!(object.alchemy "      INTEGER NOT NULL", val.as_i64()),
        db_column!(object.brawl "        INTEGER NOT NULL", val.as_i64()),
        db_column!(object.commander "    INTEGER NOT NULL", val.as_i64()),
        db_column!(object.duel "         INTEGER NOT NULL", val.as_i64()),
        db_column!(object.explorer "     INTEGER NOT NULL", val.as_i64()),
        db_column!(object.future "       INTEGER NOT NULL", val.as_i64()),
        db_column!(object.gladiator "    INTEGER NOT NULL", val.as_i64()),
        db_column!(object.historic "     INTEGER NOT NULL", val.as_i64()),
        db_column!(object.historicbrawl "INTEGER NOT NULL", val.as_i64()),
        db_column!(object.legacy "       INTEGER NOT NULL", val.as_i64()),
        db_column!(object.modern "       INTEGER NOT NULL", val.as_i64()),
        db_column!(object.oldschool "    INTEGER NOT NULL", val.as_i64()),
        db_column!(object.pauper "       INTEGER NOT NULL", val.as_i64()),
        db_column!(object.penny "        INTEGER NOT NULL", val.as_i64()),
        db_column!(object.pioneer "      INTEGER NOT NULL", val.as_i64()),
        db_column!(object.predh "        INTEGER NOT NULL", val.as_i64()),
        db_column!(object.premodern "    INTEGER NOT NULL", val.as_i64()),
        db_column!(object.standard "     INTEGER NOT NULL", val.as_i64()),
        db_column!(object.vintage "      INTEGER NOT NULL", val.as_i64()),
    ];

    fn insert_row_stmt() -> String {
        let table_name = Self::table_name();
        let params = Self::param_names().join(", ");
        let columns = Self::column_names().join(", ");
        format!("INSERT INTO {table_name} ({columns}) VALUES ({params}) ON CONFLICT ({columns}) DO UPDATE SET alchemy = excluded.alchemy;")
    }

    fn create_extras() -> Vec<String> {
        let table_name = Self::table_name();
        let params = Self::column_names().join(", ");
        vec![format!(
            "CREATE UNIQUE INDEX {table_name}_dedup ON {table_name} ({params});"
        )]
    }
}

#[test]
fn test_legalities() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;
    println!("{}", Legalities::full_setup().join("\n"));
    Legalities::setup(&conn);

    let mut data = vec![(
        Legalities {
            alchemy: Legality::Legal,
            brawl: Legality::Legal,
            commander: Legality::Legal,
            duel: Legality::Legal,
            explorer: Legality::Legal,
            future: Legality::Legal,
            gladiator: Legality::Legal,
            historic: Legality::Legal,
            historicbrawl: Legality::Legal,
            legacy: Legality::Legal,
            modern: Legality::Legal,
            oldschool: Legality::Legal,
            pauper: Legality::Legal,
            penny: Legality::Legal,
            pioneer: Legality::Legal,
            predh: Legality::Legal,
            premodern: Legality::Legal,
            standard: Legality::Legal,
            vintage: Legality::Legal,
        },
        (),
    )];

    let ids = Legalities::store_rows(data.iter_mut().map(move |(l, q)| (&*l, q)), &conn)?;
    let ids2 = Legalities::store_rows(data.iter_mut().map(move |(l, q)| (&*l, q)), &conn)?;

    assert_eq!(ids, ids2);
    Ok(())
}

impl Legalities {
    #[allow(unused)]
    fn join(&self, other: &Legalities) -> Legalities {
        Legalities {
            alchemy: Ord::min(self.alchemy, other.alchemy),
            brawl: Ord::min(self.brawl, other.brawl),
            commander: Ord::min(self.commander, other.commander),
            duel: Ord::min(self.duel, other.duel),
            explorer: Ord::min(self.explorer, other.explorer),
            future: Ord::min(self.future, other.future),
            gladiator: Ord::min(self.gladiator, other.gladiator),
            historic: Ord::min(self.historic, other.historic),
            historicbrawl: Ord::min(self.historicbrawl, other.historicbrawl),
            legacy: Ord::min(self.legacy, other.legacy),
            modern: Ord::min(self.modern, other.modern),
            oldschool: Ord::min(self.oldschool, other.oldschool),
            pauper: Ord::min(self.pauper, other.pauper),
            penny: Ord::min(self.penny, other.penny),
            pioneer: Ord::min(self.pioneer, other.pioneer),
            predh: Ord::min(self.predh, other.predh),
            premodern: Ord::min(self.premodern, other.premodern),
            standard: Ord::min(self.standard, other.standard),
            vintage: Ord::min(self.vintage, other.vintage),
        }
    }
}

#[derive(
    Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(i8)]
pub enum Legality {
    Legal = 2,
    Restricted = 1,
    Banned = -1,
    #[default]
    #[serde(other)]
    NotLegal = 0,
}

impl ToSql for Legality {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i8 as i64)))
    }
}

impl Into<bool> for Legality {
    fn into(self) -> bool {
        self == Legality::Legal
    }
}

impl From<bool> for Legality {
    fn from(value: bool) -> Self {
        if value {
            Legality::Legal
        } else {
            Legality::NotLegal
        }
    }
}

impl From<i64> for Legality {
    fn from(value: i64) -> Self {
        match value {
            ..=-1 => Self::Banned,
            0 => Self::NotLegal,
            1 => Self::Restricted,
            2.. => Self::Legal,
        }
    }
}
