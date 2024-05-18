mod common;

use std::path::PathBuf;

use common::ErrResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::asynchronous::Client,
    serde::json::json,
    uri,
};
use sqlx::{
    self,
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

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
async fn put_vote_with_incorrect_image_filename_returns_422_unprocessable_entity(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "non-existing-image.png",
        }))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn put_vote_with_incorrect_image_filename_is_json_err_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "non-existing-image.png",
        }))
        .dispatch()
        .await;
    let body = response.into_json::<ErrResponse<String>>().await;
    assert!(body.is_some());
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn put_vote_with_incorrect_image_filename_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "non-existing-image.png",
        }))
        .dispatch()
        .await;
    let body = response
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");
    let expected_error = "`image` with requested name not found".to_string();
    assert_eq!(body.error, expected_error);
}
