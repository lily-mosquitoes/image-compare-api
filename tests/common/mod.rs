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

#[derive(Deserialize)]
pub(crate) struct Image {
    pub(crate) id: i64,
    pub(crate) src: String,
}

#[derive(Deserialize)]
pub(crate) struct TwoImages {
    pub(crate) image1: Image,
    pub(crate) image2: Image,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) enum IoError {
    FileServerError(String),
}
