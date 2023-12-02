mod common;

use std::path::PathBuf;

use common::OkResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::asynchronous::Client,
    serde::uuid::Uuid,
    uri,
};
use serde::Deserialize;
use sqlx::{
    self,
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
};

static STATIC_DIR: &'static str =
    relative!("tests/test_static_dirs/with_2_files");

async fn get_http_client(db_options: SqliteConnectOptions) -> Client {
    let static_dir = PathBuf::from(STATIC_DIR);
    Client::untracked(image_compare_api::rocket(static_dir, db_options))
        .await
        .expect("valid rocket instance")
}

#[derive(Debug, PartialEq, Deserialize)]
struct User {
    id: Uuid,
    comparisons: u64,
    average_lambda: f64,
}

#[sqlx::test]
async fn get_user_with_correct_id_returns_200_ok(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);
}

// #[test]
// fn get_user_with_correct_id_is_json_ok_response() {
//     let client = get_http_client();
//     let response = client
//         .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
//         .dispatch();
//     let body = response.into_json::<OkResponse<User>>();
//     assert!(body.is_some());
// }
//
// #[test]
// fn get_user_with_correct_id_returns_expected_user() {
//     let client = get_http_client();
//     let response = client
//         .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
//         .dispatch();
//     let body = response
//         .into_json::<OkResponse<User>>()
//         .expect("body to be present");
//     let expected_user = User {
//         id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
//         comparisons: 7,
//         average_lambda: 0.1234,
//     };
//     assert_eq!(body.data, expected_user);
// }
//
// #[test]
// fn generate_user_returns_200_ok() {
//     let client = get_http_client();
//     let response = client.post(uri!("/api/user")).dispatch();
//     assert_eq!(response.status(), Status::Ok);
// }
//
// #[test]
// fn generate_user_is_json_ok_response() {
//     let client = get_http_client();
//     let response = client.post(uri!("/api/user")).dispatch();
//     let body = response.into_json::<OkResponse<User>>();
//     assert!(body.is_some());
// }
//
// #[test]
// fn generate_user_returns_new_user() {
//     let client = get_http_client();
//     let response = client.post(uri!("/api/user")).dispatch();
//     let body = response
//         .into_json::<OkResponse<User>>()
//         .expect("body to be present");
//     let expected_user = User {
//         id: body.data.id,
//         comparisons: 0,
//         average_lambda: 0.0,
//     };
//     assert_eq!(body.data, expected_user);
// }
