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

static STATIC_DIR: &'static str =
    relative!("tests/test_static_dirs/with_2_files");

fn get_http_client() -> Client {
    let static_dir = PathBuf::from(STATIC_DIR);
    Client::tracked(image_compare_api::rocket(static_dir))
        .expect("valid rocket instance")
}

#[test]
fn get_user_with_incorrect_id_returns_404_not_found() {
    let client = get_http_client();
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_user_with_incorrect_id_is_json_err_response() {
    let client = get_http_client();
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        .dispatch();
    let body = response.into_json::<ErrResponse<String>>();
    assert!(body.is_some());
}

#[test]
fn get_user_with_incorrect_id_returns_expected_error() {
    let client = get_http_client();
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        .dispatch();
    let body = response
        .into_json::<ErrResponse<String>>()
        .expect("body to be present");
    let expected_error = "Resource not found".to_string();
    assert_eq!(body.error, expected_error);
}
