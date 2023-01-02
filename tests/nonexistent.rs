mod common;

use common::{
    ErrResponse,
    NotFound,
};
use image_compare_api;
use rocket::{
    http::Status,
    local::blocking::Client,
    uri,
};

fn get_http_client() -> Client {
    Client::tracked(image_compare_api::rocket())
        .expect("valid rocket instance")
}

#[test]
fn get_nonexistent_returns_404() {
    let client = get_http_client();
    let response = client.get(uri!("/kjfkas")).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_nonexistent_is_json_error_response() {
    let client = get_http_client();
    let response = client.get(uri!("/kjfkas")).dispatch();
    let body = response.into_json::<ErrResponse<NotFound>>();
    assert!(body.is_some());
}
