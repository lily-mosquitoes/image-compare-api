mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePoolOptions,
};

use crate::common::get_asynchronous_api_client;

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[sqlx::test]
async fn preflight_request_returns_204_no_content(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .options(uri!("/api/admin/comparison"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);
}

#[sqlx::test]
async fn preflight_request_returns_no_content(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .options(uri!("/api/admin/comparison"))
        .dispatch()
        .await;

    assert!(response.body().is_none());
}

#[sqlx::test]
async fn preflight_request_returns_expected_headers(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .options(uri!("/api/admin/comparison"))
        .dispatch()
        .await;

    let allow_methods =
        response.headers().get_one("Access-Control-Allow-Methods");
    let allow_headers =
        response.headers().get_one("Access-Control-Allow-Headers");

    assert_eq!(allow_methods, Some("OPTIONS, POST, PUT, DELETE, GET"));
    assert_eq!(allow_headers, Some("Authorization"));
}
