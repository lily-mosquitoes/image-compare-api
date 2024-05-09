mod common;

use std::path::PathBuf;

use common::ErrResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::asynchronous::Client,
    uri,
};
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePoolOptions,
};

static STATIC_DIR: &'static str =
    relative!("tests/test_static_dirs/with_2_files");

async fn get_http_client(db_options: SqliteConnectOptions) -> Client {
    let static_dir = PathBuf::from(STATIC_DIR);
    Client::untracked(image_compare_api::rocket(static_dir, db_options))
        .await
        .expect("valid rocket instance")
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("users")))]
async fn get_user_with_incorrect_id_returns_404_not_found(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("users")))]
fn get_user_with_incorrect_id_is_json_err_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"))
        .dispatch()
        .await;
    let body = response.into_json::<ErrResponse<String>>().await;
    assert!(body.is_some());
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("users")))]
fn get_user_with_incorrect_id_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"))
        .dispatch()
        .await;
    let body = response
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");
    let expected_error = "`user` with requested id not found".to_string();
    assert_eq!(body.error, expected_error);
}
