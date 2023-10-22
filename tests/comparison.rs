mod common;

use std::path::PathBuf;

use common::{
    Comparison,
    OkResponse,
};
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
fn get_comparison_returns_200_ok() {
    let client = get_http_client();
    let response = client.get(uri!("/api/comparison")).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_comparison_is_json_ok_response() {
    let client = get_http_client();
    let response = client.get(uri!("/api/comparison")).dispatch();
    let body = response.into_json::<OkResponse<Comparison>>();
    assert!(body.is_some());
}

#[test]
fn get_comparison_returns_2_images() {
    let client = get_http_client();
    let response = client.get(uri!("/api/comparison")).dispatch();
    let body = response
        .into_json::<OkResponse<Comparison>>()
        .expect("body to be present");
    assert_eq!(body.data.images.len(), 2);
}

#[test]
fn get_comparison_returns_images_with_valid_origin() {
    let client = get_http_client();
    let response = client.get(uri!("/api/comparison")).dispatch();
    let body = response
        .into_json::<OkResponse<Comparison>>()
        .expect("body to be present");
    for image in body.data.images {
        let response = client.get(image).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
