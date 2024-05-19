mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};
use sqlx::sqlite::SqliteConnectOptions;

use crate::common::{
    get_asynchronous_api_client,
    ErrResponse,
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[sqlx::test]
async fn get_nonexistent_returns_404_not_found() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client.get(uri!("/non/existent/path")).dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test]
async fn get_nonexistent_is_json_error_response() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/non/existent/path"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test]
async fn get_nonexistent_returns_expected_error() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/non/existent/path"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");

    assert_eq!(body.error, "Resource not found");
}
