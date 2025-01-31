use chrono::{NaiveDate, Utc};
use email_address::EmailAddress;
use std::{fmt, path::PathBuf};

#[derive(PartialEq, Eq)]
pub enum Multiplicity {
    One,
    Many,
    OneOrMany,
}

impl fmt::Display for Multiplicity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => f.write_str("1"),
            Self::Many => f.write_str("*"),
            Self::OneOrMany => f.write_str("1|*"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct SchemaData {
    pub src_schema: String,
    pub src_field: String,
    pub tgt_schema: String,
    pub multiplicity: Multiplicity,
    pub required: bool,
}

impl PartialOrd for SchemaData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SchemaData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.src_schema.cmp(&other.src_schema)
    }
}

impl fmt::Display for SchemaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}, {}, {}, {}, {}",
            self.src_schema, self.src_field, self.tgt_schema, self.multiplicity, self.required
        ))
    }
}

#[allow(unused_variables)]
pub trait AddSchemaTypes {
    fn add_schema_types(data: &mut Vec<SchemaData>) {}

    fn add_schema() -> bool {
        true
    }
}

pub trait SchemaList {
    fn contains_schema(&self, src_schema: &str) -> bool;

    /// This will merge two multiplicity variants to one:
    /// Contact, phone, String, 1, true
    /// Contact, phone, String, \*, true
    /// Into:
    /// Contact, phone, String, 1|\*, true
    fn merge_multiplicity(&mut self);
}

impl SchemaList for Vec<SchemaData> {
    fn contains_schema(&self, src_schema: &str) -> bool {
        self.iter().any(|data| data.src_schema == src_schema)
    }

    fn merge_multiplicity(&mut self) {
        self.dedup_by(|a, b| {
            if a.src_schema == b.src_schema && a.src_field == b.src_field && a.tgt_schema == b.tgt_schema {
                b.multiplicity = Multiplicity::OneOrMany;
                true
            } else {
                false
            }
        });
    }
}

// Add empty traits impl for all external types
macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl AddSchemaTypes for $t {
            fn add_schema_types(_data: &mut Vec<SchemaData>) {}
        })*
    }
}

impl_T!(for usize, u8, u16, u32, u64, u128);
impl_T!(for isize, i8, i16, i32, i64, i128);
impl_T!(for f32, f64);
impl_T!(for String, bool, PathBuf);
impl_T!(for Utc, NaiveDate, EmailAddress);
impl_T!(for serde_json::Value);
