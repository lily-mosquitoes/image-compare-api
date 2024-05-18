mod common;

use std::path::PathBuf;

use common::ErrResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::{
        uri::Origin,
        Header,
        Status,
    },
    local::asynchronous::Client,
    uri,
};
use serde::Deserialize;
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePoolOptions,
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/error");

async fn get_http_client(db_options: SqliteConnectOptions) -> Client {
    let static_dir = PathBuf::from(STATIC_DIR);
    Client::untracked(image_compare_api::rocket(static_dir, db_options))
        .await
        .expect("valid rocket instance")
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_error_folder_returns_500_internal_server_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
        ))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::InternalServerError);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_error_folder_is_json_err_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
        ))
        .dispatch()
        .await;
    let body = response.into_json::<ErrResponse<String>>().await;
    assert!(body.is_some());
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_error_folder_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
        ))
        .dispatch()
        .await;
    let body = response
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");

    let expected_error = "Not enough files in STATIC_DIR/folder_b (minimum 2 \
                          needed)"
        .to_string();

    assert_eq!(body.error, expected_error);
}
