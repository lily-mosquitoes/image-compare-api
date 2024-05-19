mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};
use sqlx::sqlite::SqliteConnectOptions;

use crate::common::{
    get_asynchronous_api_client,
    ErrResponse,
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[sqlx::test]
async fn get_images_with_existing_filename_returns_200_ok() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .get(uri!("/static/images/image%20A.png"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test]
async fn get_images_with_existing_filename_is_file_response() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/static/images/image%20A.png"))
        .dispatch()
        .await
        .into_bytes()
        .await;

    assert!(body.is_some());
}

#[sqlx::test]
async fn get_images_with_existing_filename_is_file_in_static_files_dir() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/static/images/image%20A.png"))
        .dispatch()
        .await
        .into_bytes()
        .await
        .expect("body to be present");

    let file_path = std::path::Path::new(STATIC_DIR).join("image A.png");
    let file = std::fs::read(file_path).expect("file to be present");

    assert_eq!(body, file);
}

#[sqlx::test]
fn get_images_with_nonexisting_filename_returns_404_not_found() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .get(uri!("/static/images/does_not_exist.png"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test]
async fn get_images_with_nonexisting_filename_is_json_err_response() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/static/images/does_not_exist.png"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test]
async fn get_images_with_nonexisting_filename_returns_expected_error() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/static/images/does_not_exist.png"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");

    assert_eq!(body.error, "Resource not found");
}
