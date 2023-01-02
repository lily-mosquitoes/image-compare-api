mod common;

use common::OkResponse;
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
fn get_healthcheck_returns_ok() {
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
