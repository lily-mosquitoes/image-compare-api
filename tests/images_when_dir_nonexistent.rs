mod common;

use std::path::PathBuf;

use common::{
    generate_random_hex_string,
    ErrResponse,
};
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::blocking::Client,
    uri,
};

static STATIC_DIR: &'static str = relative!("tests/test_static_dirs");

fn get_http_client() -> Client {
    let temp_dir = generate_random_hex_string(8);
    let mut static_dir = PathBuf::from(STATIC_DIR);
    static_dir.push(temp_dir);
    std::fs::create_dir(&static_dir).unwrap();
    dbg!(&static_dir);
    let client = Client::tracked(image_compare_api::rocket(static_dir.clone()))
        .expect("valid rocket instance");
    std::fs::remove_dir(&static_dir).unwrap();
    client
}

#[test]
fn get_images_with_any_file_name_returns_404_not_found() {
    let client = get_http_client();
    let response = client.get(uri!("/static/images/imageA.png")).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_images_with_any_file_name_is_json_err_response() {
    let client = get_http_client();
    let response = client.get(uri!("/static/images/imageA.png")).dispatch();
    let body = response.into_json::<ErrResponse<String>>();
    assert!(body.is_some());
}
