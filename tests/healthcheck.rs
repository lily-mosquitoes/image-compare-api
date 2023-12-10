mod common;

use std::path::PathBuf;

use common::OkResponse;
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
fn get_healthcheck_returns_200_ok() {
    let client = get_http_client();
    let response = client.get(uri!("/api/healthcheck")).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_healthcheck_is_json_ok_response() {
    let client = get_http_client();
    let response = client.get(uri!("/api/healthcheck")).dispatch();
    let body = response.into_json::<OkResponse<()>>();
    assert!(body.is_some());
}
