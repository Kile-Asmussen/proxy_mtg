use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    #[default]
    None,
    One(T),
    Many(Vec<T>),
}

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::None => vec![],
            OneOrMany::One(item) => vec![item],
            OneOrMany::Many(items) => items,
        }
    }
}

impl<T> From<OneOrMany<T>> for Option<Vec<T>> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::None => None,
            OneOrMany::One(item) => Some(vec![item]),
            OneOrMany::Many(items) => Some(items),
        }
    }
}

impl<T> OneOrMany<T> {
    pub fn one_or_many<'de, D>(de: D) -> Result<Vec<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Ok(OneOrMany::<T>::deserialize(de)?.into())
    }

    pub fn none_or_one_or_many<'de, D>(de: D) -> Result<Option<Vec<T>>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Ok(OneOrMany::<T>::deserialize(de)?.into())
    }
}
