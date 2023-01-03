mod common;

use common::{
    ErrResponse,
    NotFound,
    OkResponse,
};
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::blocking::Client,
    uri,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Image {
    #[allow(dead_code)]
    id: i64,
    src: String,
}

#[derive(Deserialize)]
struct TwoImages {
    image1: Image,
    image2: Image,
}

fn get_http_client() -> Client {
    std::env::set_var(
        "STATIC_FILES_DIR",
        relative!("tests/test_static_files_dirs/with_file"),
    );

    Client::tracked(image_compare_api::rocket())
        .expect("valid rocket instance")
}

#[test]
fn get_images_returns_ok() {
    let client = get_http_client();
    let response = client.get(uri!("/api/images")).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_images_is_json_ok_response() {
    let client = get_http_client();
    let response = client.get(uri!("/api/images")).dispatch();
    let body = response.into_json::<OkResponse<TwoImages>>();
    assert!(body.is_some());
}

#[test]
fn get_images_returns_image_in_static_src_folder() {
    let client = get_http_client();
    let response = client.get(uri!("/api/images")).dispatch();
    let body = response
        .into_json::<OkResponse<TwoImages>>()
        .expect("body to be present");
    let test_file = "test_file.png";
    assert_eq!(&body.data.image1.src, test_file);
    assert_eq!(&body.data.image2.src, test_file);
}

#[test]
fn get_images_with_existing_file_name_returns_ok() {
    let client = get_http_client();
    let response =
        client.get(uri!("/api/images/test_file.png")).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_images_with_existing_file_name_is_file_response() {
    let client = get_http_client();
    let response =
        client.get(uri!("/api/images/test_file.png")).dispatch();
    let body = response.into_bytes();
    assert!(body.is_some());
}

#[test]
fn get_images_with_nonexisting_file_name_returns_404() {
    let client = get_http_client();
    let response = client
        .get(uri!("/api/images/shouldnt_exist.png"))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_images_with_nonexisting_file_name_is_json_err_response() {
    let client = get_http_client();
    let response = client
        .get(uri!("/api/images/shouldnt_exist.png"))
        .dispatch();
    let body = response.into_json::<ErrResponse<NotFound>>();
    assert!(body.is_some());
}
