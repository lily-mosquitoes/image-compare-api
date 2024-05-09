mod common;

use std::path::PathBuf;

use common::ErrResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::asynchronous::Client,
    uri,
};
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePoolOptions,
};

static STATIC_DIR: &'static str =
    relative!("tests/test_static_dirs/with_2_files");

async fn get_http_client(db_options: SqliteConnectOptions) -> Client {
    let static_dir = PathBuf::from(STATIC_DIR);
    Client::untracked(image_compare_api::rocket(static_dir, db_options))
        .await
        .expect("valid rocket instance")
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn get_comparison_for_user_when_none_available_returns_503_service_unavailable(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/ac01a03d-75e3-4244-a33b-a2324b8784f1/comparison"))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::ServiceUnavailable);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn get_comparison_for_user_when_none_available_is_json_err_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/ac01a03d-75e3-4244-a33b-a2324b8784f1/comparison"))
        .dispatch()
        .await;
    let body = response.into_json::<ErrResponse<String>>().await;
    assert!(body.is_some());
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn get_comparison_for_user_when_none_available_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/ac01a03d-75e3-4244-a33b-a2324b8784f1/comparison"))
        .dispatch()
        .await;
    let body = response
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to exist");
    assert_eq!(body.error, "No `comparison` available for `user`");
}
