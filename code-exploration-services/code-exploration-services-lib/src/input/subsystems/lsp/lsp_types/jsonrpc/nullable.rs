extern crate serde;

use serde::{Deserialize, Deserializer};
use self::serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum Nullable<T> {
    None,
    Some(T),
}

impl<T> Nullable<T> {}

impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Nullable::Some(value) = &self {
            serializer.serialize_some::<T>(value)
        } else {
            serializer.serialize_unit()
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(match Option::<T>::deserialize(deserializer).unwrap_or(None) {
            None => Self::None,
            Some(i) => Self::Some(i),
        })
    }
}