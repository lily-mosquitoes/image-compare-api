#![allow(dead_code)]

use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ErrResponse<E> {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) error: E,
}

#[derive(Deserialize)]
pub(crate) enum NotFound {
    ResourceNotFound(String),
}

#[derive(Deserialize)]
pub(crate) struct OkResponse<T> {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) data: T,
}
