use std::{
    error::Error,
    fmt::Display,
    ops::Deref,
};

use rocket::http::Status;
use serde::{
    Deserialize,
    Serialize,
    Serializer,
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

#[derive(Debug)]
pub(crate) enum QueryError {
    Sqlx(sqlx::Error),
    RowNotFound(String),
}

impl From<sqlx::Error> for QueryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Sqlx(error)
    }
}

impl Serialize for QueryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        println!("SERIALIZING");
        let s = serializer.serialize_str(&self.to_string());
        println!("DONE");
        s
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlx(error) => write!(f, "{}", error),
            Self::RowNotFound(message) => write!(f, "{}", message),
        }
    }
}

impl Error for QueryError {}

impl QueryError {
    pub(crate) fn default_status(&self) -> Status {
        match self {
            Self::RowNotFound(_) => Status::NotFound,
            Self::Sqlx(sqlx::Error::RowNotFound) => Status::NotFound,
            _ => Status::InternalServerError,
        }
    }
}
