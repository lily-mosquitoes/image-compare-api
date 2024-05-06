use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ErrResponse<E> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) error: E,
}

#[derive(Deserialize)]
pub(crate) struct OkResponse<T> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) data: T,
}
