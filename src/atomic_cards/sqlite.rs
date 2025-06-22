#![allow(unused)]

use std::{any::Any, default, fmt::Display, marker::PhantomData, time::Instant};

use itertools::Itertools;
use rusqlite::{
    types::{FromSql, ToSqlOutput, Value, ValueRef},
    Connection, Params, Row, ToSql,
};

use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
pub struct DbColumn<O, K>
where
    O: SqliteTable<ForeignKeys = K> + 'static,
    K: 'static,
{
    pub name: &'static str,
    pub param: &'static str,
    pub typ: &'static str,
    pub index: IsIndexed,
    pub get_set: GetSetter<O, K>,
}

macro_rules! getsetters {
    ($obj_or_key:ident.$field:ident, val.$method:ident()) => {
        crate::atomic_cards::sqlite::GetSetter::$obj_or_key {
            get: |thing| thing.$field.to_sql(),
            set: |thing, val| Ok(thing.$field = val.$method()?.into()),
        }
    };
    ($obj_or_key:ident.$field:ident, $val:ident -> $body:expr) => {
        crate::atomic_cards::sqlite::GetSetter::$obj_or_key {
            get: |thing| thing.$field.to_sql(),
            set: |thing, $val| Ok(thing.$field = $body),
        }
    };
    ($obj_or_key:ident.$field:ident,
        $val1:ident -> $body1:expr,
        $val2:ident <- $body2:expr) => {
        crate::atomic_cards::sqlite::GetSetter::$obj_or_key {
            get: |thing| {
                let $val2 = &thing.$field;
                Ok(ToSqlOutput::Owned($body2))
            },
            set: |thing, $val1| Ok(thing.$field = $body1),
        }
    };
}

pub(crate) use getsetters;

macro_rules! db_column {
    (object.$field:ident $typ:literal, $($getsetters:tt)*) => {
        crate::atomic_cards::sqlite::DbColumn {
            name: stringify!($field),
            param: concat!(":", stringify!($field)),
            typ: $typ,
            index: crate::atomic_cards::sqlite::IsIndexed::NOINDEX,
            get_set: crate::atomic_cards::sqlite::getsetters!(Object.$field, $($getsetters)*),
        }
    };
    (INDEX object.$field:ident $typ:literal, $($getsetters:tt)*) => {
        crate::atomic_cards::sqlite::DbColumn {
            name: stringify!($field),
            param: concat!(":", stringify!($field)),
            typ: $typ,
            index: crate::atomic_cards::sqlite::IsIndexed::INDEX,
            get_set: crate::atomic_cards::sqlite::getsetters!(Object.$field, $($getsetters)*),
        }
    };
    (UNIQUE object.$field:ident $typ:literal, $($getsetters:tt)*) => {
        crate::atomic_cards::sqlite::DbColumn {
            name: stringify!($field),
            param: concat!(":", stringify!($field)),
            typ: $typ,
            index: crate::atomic_cards::sqlite::IsIndexed::UNIQUE,
            get_set: crate::atomic_cards::sqlite::getsetters!(Object.$field, $($getsetters)*),
        }
    };
    (key.$field:ident $typ:literal, $($getsetters:tt)*) => {
        crate::atomic_cards::sqlite::DbColumn {
            name: stringify!($field),
            param: concat!(":", stringify!($field)),
            typ: $typ,
            index: crate::atomic_cards::sqlite::IsIndexed::INDEX,
            get_set: crate::atomic_cards::sqlite::getsetters!(Key.$field, $($getsetters)*),
        }
    };
    (NOINDEX key.$field:ident $typ:literal, $($getsetters:tt)*) => {
        crate::atomic_cards::sqlite::DbColumn {
            name: stringify!($field),
            param: concat!(":", stringify!($field)),
            typ: $typ,
            index: crate::atomic_cards::sqlite::IsIndexed::NOINDEX,
            get_set: crate::atomic_cards::sqlite::getsetters!(Key.$field, $($getsetters)*),
        }
    };
    (UNIQUE key.$field:ident $typ:literal, $($getsetters:tt)*) => {
        crate::atomic_cards::sqlite::DbColumn {
            name: stringify!($field),
            param: concat!(":", stringify!($field)),
            typ: $typ,
            index: crate::atomic_cards::sqlite::IsIndexed::UNIQUE,
            get_set: crate::atomic_cards::sqlite::getsetters!(Key.$field, $($getsetters)*),
        }
    };
}

pub(crate) use db_column;

impl<O, K> DbColumn<O, K>
where
    O: SqliteTable<ForeignKeys = K>,
{
    fn column_spec(&self) -> String {
        let DbColumn { name, typ, .. } = self;
        let typ = typ.trim();
        format!("{name} {typ}")
    }
    fn create_index(&self) -> Option<String> {
        let DbColumn { name, index, .. } = self;
        let table = O::table_name();
        if index == &IsIndexed::NOINDEX {
            return None;
        }

        Some(format!(
            "CREATE {index} IF NOT EXISTS {table}__{name} ON {table} ({name});"
        ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum IsIndexed {
    NOINDEX,
    INDEX,
    UNIQUE,
}

impl Display for IsIndexed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            IsIndexed::INDEX => "INDEX",
            IsIndexed::UNIQUE => "UNIQUE INDEX",
            IsIndexed::NOINDEX => "",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GetSetter<O, K> {
    Object {
        get: for<'a> fn(&'a O) -> rusqlite::Result<ToSqlOutput<'a>>,
        set: for<'a> fn(&'a mut O, ValueRef<'a>) -> rusqlite::Result<()>,
    },
    Key {
        get: for<'a> fn(&'a K) -> rusqlite::Result<ToSqlOutput<'a>>,
        set: for<'a> fn(&'a mut K, ValueRef<'a>) -> rusqlite::Result<()>,
    },
    None,
}

impl<O, K> GetSetter<O, K> {
    fn get<'b, 'a>(&'b self, object: &'a O, key: &'a K) -> rusqlite::Result<ToSqlOutput<'a>> {
        match self {
            GetSetter::Object { get, .. } => get(object),
            GetSetter::Key { get, .. } => get(key),
            GetSetter::None => Ok(ToSqlOutput::Owned(Value::Null)),
        }
    }

    fn set<'b, 'a>(
        &'b self,
        object: &'a mut O,
        key: &'a mut K,
        val: ValueRef<'a>,
    ) -> rusqlite::Result<()> {
        match self {
            GetSetter::Object { set, .. } => set(object, val),
            GetSetter::Key { set, .. } => set(key, val),
            GetSetter::None => Ok(()),
        }
    }
}

pub trait SqliteTable: Sized + Default + 'static {
    type ForeignKeys: Default + 'static;

    const COLUMNS: &'static [DbColumn<Self, Self::ForeignKeys>];

    fn into_params<'a>(
        &'a self,
        keys: &'a Self::ForeignKeys,
    ) -> anyhow::Result<Vec<(&'static str, ToSqlOutput<'a>)>> {
        let mut res = vec![];

        for DbColumn { param, get_set, .. } in Self::COLUMNS {
            res.push((*param, get_set.get(self, keys)?));
        }

        Ok(res)
    }

    fn from_row(row: &Row) -> rusqlite::Result<(i64, Self, Self::ForeignKeys)> {
        let mut res_self = Self::default();
        let mut res_keys = Self::ForeignKeys::default();

        for (i, DbColumn { get_set, .. }) in Self::COLUMNS.iter().enumerate() {
            get_set.set(&mut res_self, &mut res_keys, row.get_ref(i + 1)?)?;
        }

        Ok((row.get(0)?, res_self, res_keys))
    }

    fn into_keys<'a>(
        keys: &'a Self::ForeignKeys,
    ) -> anyhow::Result<Vec<(&'static str, ToSqlOutput<'a>)>> {
        let mut res = vec![];

        for column in Self::COLUMNS {
            let DbColumn {
                param,
                index: IsIndexed::INDEX | IsIndexed::UNIQUE,
                get_set: GetSetter::Key { get, .. },
                ..
            } = column
            else {
                continue;
            };
            res.push((*param, get(keys)?))
        }

        Ok(res)
    }

    fn table_name() -> &'static str {
        std::any::type_name::<Self>().split("::").last().unwrap()
    }

    fn column_names() -> Vec<&'static str> {
        Self::COLUMNS
            .iter()
            .map(|DbColumn { name, .. }| *name)
            .collect_vec()
    }

    fn create_table_stmt() -> Vec<String> {
        let table_name = Self::table_name();

        let column_specs = Self::COLUMNS.iter().map(DbColumn::column_spec).join(", ");

        let mut res = vec![format!(
            "CREATE TABLE IF NOT EXISTS {table_name} ({column_specs});"
        )];

        let mut indexes = Self::COLUMNS
            .iter()
            .map(DbColumn::create_index)
            .filter_map(|x| x)
            .collect_vec();

        res.append(&mut indexes);

        res
    }

    fn create_extras() -> Vec<String> {
        vec![]
    }

    fn param_names() -> Vec<&'static str> {
        Self::COLUMNS
            .iter()
            .map(|DbColumn { param, .. }| *param)
            .collect_vec()
    }

    fn key_matches() -> Vec<String> {
        Self::COLUMNS
            .iter()
            .filter_map(|c| {
                if let DbColumn {
                    name,
                    index: IsIndexed::INDEX | IsIndexed::UNIQUE,
                    get_set: GetSetter::Key { .. },
                    ..
                } = c
                {
                    Some(format!("{name} = :{name}"))
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn insert_row_stmt() -> String {
        let table_name = Self::table_name();
        let column_names = Self::column_names().join(", ");
        let params = Self::param_names().join(", ");
        format!("INSERT INTO {table_name} ({column_names}) VALUES ({params});")
    }

    fn select_row_stmt() -> String {
        let table_name = Self::table_name();
        let column_names = Self::column_names().join(", ");
        format!("SELECT rowid, {column_names} FROM {table_name} WHERE rowid = :rowid;")
    }

    fn select_keyed_stmt() -> String {
        let table_name = Self::table_name();
        let column_names = Self::column_names().join(", ");
        let key_matches = Self::key_matches().join(" AND ");
        format!("SELECT rowid, {column_names} FROM {table_name} WHERE {key_matches};")
    }

    fn select_all_stmt() -> String {
        let table_name = Self::table_name();
        let column_names = Self::column_names().join(", ");
        format!("SELECT rowid, {column_names} FROM {table_name};")
    }

    fn full_setup() -> Vec<String> {
        let mut res = Self::create_table_stmt();
        res.append(&mut Self::create_extras());
        res
    }

    fn setup(conn: &Connection) -> anyhow::Result<()> {
        for stmt in Self::full_setup() {
            conn.execute(&stmt, [])?;
        }
        Self::extra_setup(conn)?;
        Ok(())
    }

    fn extra_setup(conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }

    fn pre_store(&self, key: &mut Self::ForeignKeys, conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }

    fn post_store(&self, id: i64, conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }

    fn store_rows<'a>(
        data: impl IntoIterator<Item = (&'a Self, &'a mut Self::ForeignKeys)>,
        conn: &Connection,
    ) -> anyhow::Result<Vec<i64>>
    where
        Self: 'a,
    {
        let mut res = vec![];
        let mut stmt = conn.prepare(&Self::insert_row_stmt())?;

        for (obj, keys) in data {
            obj.pre_store(keys, &conn)?;
            let id = stmt.insert(&obj.into_params(keys)?[..])?;
            obj.post_store(id, conn)?;
            res.push(id);
        }

        Ok(res)
    }

    fn load(&mut self, id: i64, key: &Self::ForeignKeys, conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }

    fn load_rows(
        ids: impl IntoIterator<Item = i64>,
        conn: &Connection,
    ) -> anyhow::Result<Vec<(i64, Self, Self::ForeignKeys)>> {
        let mut res = vec![];
        let mut stmt = conn.prepare(&Self::select_row_stmt())?;

        for id in ids {
            let (id, mut object, key) =
                stmt.query_one(&[(":rowid", &id)], |row| Self::from_row(row))?;

            object.load(id, &key, conn)?;

            res.push((id, object, key));
        }
        Ok(res)
    }

    fn load_keys<'a>(
        keys: impl IntoIterator<Item = &'a Self::ForeignKeys>,
        conn: &Connection,
    ) -> anyhow::Result<Vec<(i64, Self, Self::ForeignKeys)>> {
        let mut res = vec![];
        let mut stmt = conn.prepare(&Self::select_keyed_stmt())?;

        for key in keys {
            let params = Self::into_keys(key);
            for row in stmt.query_and_then(&params?[..], |r| Self::from_row(r))? {
                let (id, mut object, key) = row?;
                object.load(id, &key, &conn);
                res.push((id, object, key));
            }
        }

        Ok(res)
    }

    fn load_all(conn: &Connection) -> anyhow::Result<Vec<(i64, Self, Self::ForeignKeys)>> {
        let mut res = vec![];
        let select_all = Self::select_all_stmt();
        let mut stmt = conn.prepare(&select_all)?;

        for row in stmt.query_and_then([], |r| Self::from_row(r))? {
            let (id, mut object, key) = row?;
            object.load(id, &key, &conn);
            res.push((id, object, key));
        }

        Ok(res)
    }
}
