use std::path::Path;

use chrono::{
    DateTime,
    Utc,
};
use image_compare_api;
use rocket::local::asynchronous;
use serde::Deserialize;
use sqlx::sqlite::SqliteConnectOptions;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct ApiResponse<T, E> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) data: Option<T>,
    pub(crate) error: Option<E>,
}

pub(crate) async fn get_api_client<P: AsRef<Path>>(
    static_dir: P,
    db_options: SqliteConnectOptions,
) -> asynchronous::Client {
    asynchronous::Client::untracked(image_compare_api::rocket(
        "*", static_dir, db_options,
    ))
    .await
    .expect("valid rocket instance")
}

#[allow(unused_macros)]
macro_rules! make_api_test {
    (
        #[fileserver($static_dir:ident = $static_dir_path:expr)] #[fixtures($($fixture:literal),*)]
        let request = |$client:ident| {$request:expr};

        #[test_request]
        let $function:ident = |$response:ident| {
            $($statement:stmt)+
        };

        $($rest:tt)+
    ) => {
        make_api_test! {
            #[fileserver($static_dir = $static_dir_path)] #[fixtures($($fixture),*)]
            let request = |$client| {$request};

            #[test_request]
            let $function = |$response| {
                $($statement)+
            };
        }

        make_api_test! {
            #[fileserver($static_dir = $static_dir_path)] #[fixtures($($fixture),*)]
            let request = |$client| {$request};

            $($rest)+
        }
    };

    (
        #[fileserver($static_dir:ident = $static_dir_path:expr)] #[fixtures($($fixture:literal),*)]
        let request = |$client:ident| {$request:expr};

        #[test_request]
        let $function:ident = |$response:ident| {
            $($statement:stmt)+
        };
    ) => {
        #[sqlx::test(fixtures(path = "./../fixtures", scripts($($fixture),*),))]
        async fn $function(
            _: sqlx::sqlite::SqlitePoolOptions,
            db_options: sqlx::sqlite::SqliteConnectOptions,
        ) {
            let $static_dir = $static_dir_path;
            let $client = $crate::common::get_api_client($static_dir, db_options).await;

            let $response = $request.dispatch().await;

            $($statement)*
        }
    };
}

#[allow(unused_imports)]
pub(crate) use make_api_test;
