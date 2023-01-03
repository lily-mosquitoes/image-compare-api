mod common;

use common::{
    ErrResponse,
    NotFound,
};
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::blocking::Client,
    uri,
};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
enum IoError {
    FileServerError(String),
}

fn get_http_client() -> Client {
    std::env::set_var(
        "STATIC_FILES_DIR",
        relative!("tests/test_static_files_dirs/empty"),
    );

    Client::tracked(image_compare_api::rocket())
        .expect("valid rocket instance")
}

#[test]
fn get_images_returns_500() {
    let client = get_http_client();
    let response = client.get(uri!("/api/images")).dispatch();
    assert_eq!(response.status(), Status::InternalServerError);
}

#[test]
fn get_images_is_json_err_response() {
    let client = get_http_client();
    let response = client.get(uri!("/api/images")).dispatch();
    let body = response.into_json::<ErrResponse<IoError>>();
    assert!(body.is_some());
}

#[test]
fn get_images_returns_message_of_empty_static_files_dir() {
    let client = get_http_client();
    let response = client.get(uri!("/api/images")).dispatch();
    let body = response
        .into_json::<ErrResponse<IoError>>()
        .expect("body to be present");
    assert_eq!(
        body.error,
        IoError::FileServerError(
            "Empty STATIC_FILES_DIR".to_string()
        )
    );
}

#[test]
fn get_images_with_file_name_returns_404() {
    let client = get_http_client();
    let response =
        client.get(uri!("/api/images/any_name.png")).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_images_with_file_name_is_json_err_response() {
    let client = get_http_client();
    let response =
        client.get(uri!("/api/images/any_name.png")).dispatch();
    let body = response.into_json::<ErrResponse<NotFound>>();
    assert!(body.is_some());
}
