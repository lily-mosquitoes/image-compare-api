mod common;

use std::path::{
    Path,
    PathBuf,
};

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
fn get_images_with_existing_filename_returns_200_ok() {
    let client = get_http_client();
    let response = client.get(uri!("/static/images/image%20A.png")).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_images_with_existing_filename_is_file_response() {
    let client = get_http_client();
    let response = client.get(uri!("/static/images/image%20A.png")).dispatch();
    let body = response.into_bytes();
    assert!(body.is_some());
}

#[test]
fn get_images_with_existing_filename_is_file_in_static_files_dir() {
    let client = get_http_client();
    let response = client.get(uri!("/static/images/image%20A.png")).dispatch();
    let body = response.into_bytes().unwrap();
    let file_path = Path::new(STATIC_DIR).join("image A.png");
    let file = std::fs::read(file_path).expect("file to be present");
    assert_eq!(body, file);
}

#[test]
fn get_images_with_nonexisting_filename_returns_404_not_found() {
    let client = get_http_client();
    let response = client
        .get(uri!("/static/images/does_not_exist.png"))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_images_with_nonexisting_filename_is_json_err_response() {
    let client = get_http_client();
    let response = client
        .get(uri!("/static/images/does_not_exist.png"))
        .dispatch();
    let body = response.into_json::<ErrResponse<String>>();
    assert!(body.is_some());
}
