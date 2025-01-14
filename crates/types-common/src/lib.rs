pub use email_address::*;
pub use macro_derive::*;
pub use macro_derive::{EnumDeserialize, TagType};
use serde::Serialize;
use serde::{de, de::DeserializeOwned, de::Unexpected, Deserializer};
use std::fmt;
use traits as types_common;
pub use traits::*;

mod traits;

#[derive(Clone, Debug)]
pub enum OneOrMany<T> {
    One(Box<T>),
    Many(Vec<T>),
}

impl<'de, T: DeserializeOwned + fmt::Debug> de::Deserialize<'de> for OneOrMany<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let serde_value = serde_json::Value::deserialize(deserializer)?;

        if let serde_json::Value::Array(arr) = serde_value {
            let val = serde_json::Value::Array(arr);
            let out = Vec::deserialize(val).map_err(de::Error::custom)?;

            Ok(Self::Many(out))
        } else {
            let out = serde_json::from_value(serde_value).map_err(de::Error::custom)?;

            Ok(Self::One(out))
        }
    }
}

impl<T: Serialize> Serialize for OneOrMany<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OneOrMany::One(obj) => obj.serialize(serializer),
            OneOrMany::Many(vec) => vec.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, Serialize, GenPaths)]
pub struct PositiveInteger(pub u32);
impl std::ops::Deref for PositiveInteger {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> de::Deserialize<'de> for PositiveInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let number: i64 = i64::deserialize(deserializer)?;

        if 0 <= number {
            Ok(PositiveInteger(number as u32))
        } else {
            Err(<D::Error as de::Error>::invalid_value(
                Unexpected::Signed(number),
                &"A positive integer",
            ))
        }
    }
}

#[derive(Clone, Debug, GenPaths)]
pub struct DurationType(iso8601_duration::Duration);

impl DurationType {
    pub fn new(duration: iso8601_duration::Duration) -> Self {
        DurationType(duration)
    }
}

impl std::ops::Deref for DurationType {
    type Target = iso8601_duration::Duration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for DurationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> de::Deserialize<'de> for DurationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        let duration: iso8601_duration::Duration = str
            .parse()
            .map_err(|e: iso8601_duration::ParseDurationError| serde::de::Error::custom(e.input))?;

        Ok(DurationType(duration))
    }
}
