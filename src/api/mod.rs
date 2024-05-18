use std::{
    error::Error,
    fmt::Display,
    ops::Deref,
};

use rocket::http::{
    uri::Origin,
    Status,
};
use serde::{
    Deserialize,
    Serialize,
    Serializer,
};
use uuid::Uuid;

use crate::STATIC_ROUTE;

pub(crate) mod admin;
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

#[derive(Serialize)]
pub(crate) struct SqliteArray<'a>(Vec<Origin<'a>>);

impl<'a> From<String> for SqliteArray<'a> {
    fn from(value: String) -> Self {
        Self(
            value
                .split("///")
                .map(str::to_string)
                .map(|filename| {
                    let path = format!("{STATIC_ROUTE}/{filename}");
                    Origin::parse_owned(path)
                        .expect("BUG: path should be parseable.")
                })
                .collect(),
        )
    }
}

impl<'a> Deref for SqliteArray<'a> {
    type Target = Vec<Origin<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub(crate) enum QueryError {
    Sqlx(sqlx::Error),
    RowNotFound(String),
    FileServerError(String),
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
            Self::FileServerError(message) => write!(f, "{}", message),
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
