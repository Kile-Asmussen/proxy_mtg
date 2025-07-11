#![allow(unused)]

use std::{any::Any, default, fmt::Display, marker::PhantomData, time::Instant};

use itertools::Itertools;
use rusqlite::{
    types::{FromSql, ToSqlOutput, Value, ValueRef},
    Connection, Params, Row, Statement, ToSql,
};

use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
pub struct DbColumn<O, K>
where
    O: SqliteTable<Keys = K> + 'static,
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
        crate::atomic_cards::sqlite::GetSetter::$obj_or_key(crate::atomic_cards::sqlite::GetSet {
            get: |thing| thing.$field.to_sql(),
            set: |thing, val| Ok(thing.$field = val.$method()?.into()),
        })
    };
    ($obj_or_key:ident.$field:ident, $val:ident -> $body:expr) => {
        crate::atomic_cards::sqlite::GetSetter::$obj_or_key(crate::atomic_cards::sqlite::GetSet {
            get: |thing| thing.$field.to_sql(),
            set: |thing, $val| Ok(thing.$field = $body),
        })
    };
    ($obj_or_key:ident.$field:ident,
        $val1:ident -> $body1:expr,
        $val2:ident <- $body2:expr) => {
        crate::atomic_cards::sqlite::GetSetter::$obj_or_key(crate::atomic_cards::sqlite::GetSet {
            get: |thing| {
                let $val2 = &thing.$field;
                Ok(ToSqlOutput::Owned($body2))
            },
            set: |thing, $val1| Ok(thing.$field = $body1),
        })
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

use crate::utils::ToS;

impl<O, K> DbColumn<O, K>
where
    O: SqliteTable<Keys = K>,
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
    Object(GetSet<O>),
    Key(GetSet<K>),
}

impl<O, K> GetSetter<O, K> {
    fn get<'b, 'a>(&'b self, object: &'a O, key: &'a K) -> rusqlite::Result<ToSqlOutput<'a>> {
        match self {
            GetSetter::Object(g) => g.get(object),
            GetSetter::Key(g) => g.get(key),
        }
    }

    fn set<'b, 'a>(
        &'b self,
        object: &'a mut O,
        key: &'a mut K,
        val: ValueRef<'a>,
    ) -> rusqlite::Result<()> {
        match self {
            GetSetter::Object(s) => s.set(object, val),
            GetSetter::Key(s) => s.set(key, val),
        }
    }

    fn key(&self) -> Option<GetSet<K>> {
        match self {
            GetSetter::Key(get_set) => Some(*get_set),
            _ => None,
        }
    }

    fn object(&self) -> Option<GetSet<O>> {
        match self {
            GetSetter::Object(get_set) => Some(*get_set),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct GetSet<X> {
    pub get: for<'a> fn(&'a X) -> rusqlite::Result<ToSqlOutput<'a>>,
    pub set: for<'a> fn(&'a mut X, ValueRef<'a>) -> rusqlite::Result<()>,
}

impl<X> Clone for GetSet<X> {
    fn clone(&self) -> Self {
        Self {
            get: self.get,
            set: self.set,
        }
    }
}
impl<X> Copy for GetSet<X> {}

impl<X> GetSet<X> {
    pub fn get<'a>(&self, x: &'a X) -> rusqlite::Result<ToSqlOutput<'a>> {
        (self.get)(x)
    }
    pub fn set<'a>(&self, x: &'a mut X, v: ValueRef<'a>) -> rusqlite::Result<()> {
        (self.set)(x, v)
    }
}

pub trait SqliteTable: Sized + Default + 'static {
    type Keys: Default + 'static;

    const COLUMNS: &'static [DbColumn<Self, Self::Keys>];

    fn table_name() -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap()
            .s()
    }

    fn create_extras() -> Vec<String> {
        vec![]
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
        let key_matches = Self::key_comparisons().join(" AND ");
        format!("SELECT rowid, {column_names} FROM {table_name} WHERE {key_matches};")
    }

    fn select_all_stmt() -> String {
        let table_name = Self::table_name();
        let column_names = Self::column_names().join(", ");
        format!("SELECT rowid, {column_names} FROM {table_name};")
    }

    fn extra_setup(conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }

    fn pre_store(&self, key: &mut Self::Keys, conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }

    fn post_store(&self, id: i64, conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }

    fn load(&mut self, id: i64, key: &Self::Keys, conn: &Connection) -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait SqliteTableImpl: SqliteTable {
    fn into_params<'a>(
        &'a self,
        keys: &'a Self::Keys,
    ) -> anyhow::Result<Vec<(&'static str, ToSqlOutput<'a>)>> {
        let mut res = vec![];

        for DbColumn { param, get_set, .. } in Self::COLUMNS {
            res.push((*param, get_set.get(self, keys)?));
        }

        Ok(res)
    }

    fn from_row(row: &Row) -> rusqlite::Result<(i64, Self, Self::Keys)> {
        let mut res_self = Self::default();
        let mut res_keys = Self::Keys::default();

        for DbColumn { name, get_set, .. } in Self::COLUMNS.iter() {
            get_set.set(&mut res_self, &mut res_keys, row.get_ref(*name)?)?;
        }

        Ok((row.get("rowid")?, res_self, res_keys))
    }

    fn key_comparisons() -> Vec<String> {
        Self::generalized_keying_builder(&(), |(), DbColumn { name, param, .. }| {
            Ok(format!("{name} = {param}"))
        })
        .unwrap()
    }

    fn key_column_names() -> Vec<String> {
        Self::generalized_keying_builder(&(), |(), DbColumn { name, .. }| Ok(format!("{name}")))
            .unwrap()
    }

    fn column_names() -> Vec<&'static str> {
        Self::COLUMNS
            .iter()
            .map(|DbColumn { name, .. }| *name)
            .collect_vec()
    }

    fn param_names() -> Vec<&'static str> {
        Self::COLUMNS
            .iter()
            .map(|DbColumn { param, .. }| *param)
            .collect_vec()
    }

    fn create_table_script() -> Vec<String> {
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

    fn full_setup_script() -> Vec<String> {
        let mut res = Self::create_table_script();
        res.append(&mut Self::create_extras());
        res
    }

    fn setup(conn: &Connection) -> anyhow::Result<()> {
        for stmt in Self::full_setup_script() {
            conn.execute(&stmt, [])?;
        }
        Self::extra_setup(conn)?;
        Ok(())
    }

    fn generalized_keying_builder<K, F, X>(k: &K, mut f: F) -> anyhow::Result<Vec<X>>
    where
        F: FnMut(&K, &DbColumn<Self, Self::Keys>) -> anyhow::Result<X>,
    {
        let mut res = vec![];

        for c in Self::COLUMNS {
            let DbColumn {
                index: IsIndexed::INDEX | IsIndexed::UNIQUE,
                get_set: GetSetter::Key { .. },
                ..
            } = c
            else {
                continue;
            };

            res.push(f(k, c)?);
        }

        Ok(res)
    }

    fn into_key_params<'a>(
        keys: &'a Self::Keys,
    ) -> anyhow::Result<Vec<(&'static str, ToSqlOutput<'a>)>> {
        Self::generalized_keying_builder(keys, |k, c| {
            Ok((c.param, c.get_set.key().unwrap().get(keys)?))
        })
    }

    fn load_rows<F>(
        ids: impl IntoIterator<Item = i64>,
        conn: &Connection,
        mut mapper: F,
    ) -> anyhow::Result<()>
    where
        F: FnMut(i64, Self, Self::Keys) -> anyhow::Result<()>,
    {
        let mut stmt = conn.prepare(&Self::select_row_stmt())?;

        for id in ids {
            let (id, mut object, key) =
                stmt.query_one(&[(":rowid", &id)], |row| Self::from_row(row))?;

            object.load(id, &key, conn)?;

            mapper(id, object, key)?;
        }
        Ok(())
    }

    fn load_keys<'a, F>(
        keys: impl IntoIterator<Item = &'a Self::Keys>,
        conn: &Connection,
        mut mapper: F,
    ) -> anyhow::Result<()>
    where
        F: FnMut(i64, Self, Self::Keys) -> anyhow::Result<()>,
    {
        let mut stmt = conn.prepare(&Self::select_keyed_stmt())?;

        for key in keys {
            let params = Self::into_key_params(key);
            for row in stmt.query_and_then(&params?[..], |r| Self::from_row(r))? {
                let (id, mut object, key) = row?;
                object.load(id, &key, &conn);
                mapper(id, object, key)?;
            }
        }

        Ok(())
    }

    fn load_all<F>(conn: &Connection, mut mapper: F) -> anyhow::Result<()>
    where
        F: FnMut(i64, Self, Self::Keys) -> anyhow::Result<()>,
    {
        let select_all = Self::select_all_stmt();
        let mut stmt = conn.prepare(&select_all)?;

        for row in stmt.query_and_then([], |r| Self::from_row(r))? {
            let (id, mut object, key) = row?;
            object.load(id, &key, &conn);
            mapper(id, object, key)?;
        }

        Ok(())
    }

    fn store_rows<'a, F>(conn: &'a Connection, data_provider: F) -> anyhow::Result<()>
    where
        F: FnOnce(Store<'a, Self, Self::Keys>) -> anyhow::Result<()>,
    {
        let mut stmt = conn.prepare(&Self::insert_row_stmt())?;

        data_provider(Store {
            stmt,
            conn,
            _marker: PhantomData,
        })?;

        Ok(())
    }
}

pub struct Store<'a, O: SqliteTable<Keys = K>, K> {
    stmt: Statement<'a>,
    conn: &'a Connection,
    _marker: PhantomData<(O, K)>,
}

impl<'a, O: SqliteTable<Keys = K>, K> Store<'a, O, K> {
    pub fn store(&mut self, obj: &O, key: &mut K) -> anyhow::Result<i64> {
        obj.pre_store(key, &self.conn)?;
        let id = self.stmt.insert(&obj.into_params(key)?[..])?;
        obj.post_store(id, &self.conn)?;
        Ok(id)
    }
}

impl<T: SqliteTable> SqliteTableImpl for T {}
