mod common;

use std::path::PathBuf;

use common::ErrResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::blocking::Client,
    uri,
};
use sqlx::sqlite::SqliteConnectOptions;

static STATIC_DIR: &'static str =
    relative!("tests/test_static_dirs/with_2_files");

fn get_http_client() -> Client {
    let static_dir = PathBuf::from(STATIC_DIR);
    let db_options = SqliteConnectOptions::new();
    Client::untracked(image_compare_api::rocket(static_dir, db_options))
        .expect("valid rocket instance")
}

#[test]
fn get_nonexistent_returns_404_not_found() {
    let client = get_http_client();
    let response = client.get(uri!("/non/existent/path")).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_nonexistent_is_json_error_response() {
    let client = get_http_client();
    let response = client.get(uri!("/non/existent/path")).dispatch();
    let body = response.into_json::<ErrResponse<String>>();
    assert!(body.is_some());
}

#[test]
fn get_nonexistent_returns_expected_error() {
    let client = get_http_client();
    let response = client.get(uri!("/non/existent/path")).dispatch();
    let body = response
        .into_json::<ErrResponse<String>>()
        .expect("body to be present");
    assert_eq!(body.error, "Resource not found");
}
