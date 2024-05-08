use std::ops::Deref;

use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

pub(crate) mod comparison;
pub(crate) mod healthcheck;
pub(crate) mod user;
pub(crate) mod vote;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct SqliteUuid(Uuid);

impl From<Vec<u8>> for SqliteUuid {
    fn from(mut value: Vec<u8>) -> Self {
        value.truncate(16);

        let uuid = Uuid::from_slice(&value)
            .expect("BUG: 16 byte array should be a valid UUID.");

        Self(uuid)
    }
}

impl Deref for SqliteUuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct SqliteArray(Vec<String>);

impl From<String> for SqliteArray {
    fn from(value: String) -> Self {
        Self(value.split("/").map(str::to_string).collect())
    }
}

impl Deref for SqliteArray {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
