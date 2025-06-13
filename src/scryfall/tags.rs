use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Serialize};

pub trait DeserializeAsTag: Default {
    const TAG: &'static str;
}

#[derive(Default)]
pub struct Tag<T: DeserializeAsTag>(PhantomData<T>);

impl<'de, T> Visitor<'de> for Tag<T>
where
    T: DeserializeAsTag,
{
    type Value = Self;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expectign string literal \"{}\"", T::TAG)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == T::TAG {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("Expected \"{}\"", T::TAG)))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de, T: DeserializeAsTag> Deserialize<'de> for Tag<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Self::default())
    }
}

impl<T: DeserializeAsTag> Serialize for Tag<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(T::TAG)
    }
}
