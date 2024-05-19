use std::path::Path;

use chrono::{
    DateTime,
    Utc,
};
use image_compare_api;
use rocket::local::asynchronous;
use serde::Deserialize;
use sqlx::sqlite::SqliteConnectOptions;

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

pub(crate) async fn get_asynchronous_api_client<P: AsRef<Path>>(
    static_dir: P,
    db_options: SqliteConnectOptions,
) -> asynchronous::Client {
    asynchronous::Client::untracked(image_compare_api::rocket(
        static_dir, db_options,
    ))
    .await
    .expect("valid rocket instance")
}
