mod common;

use std::path::PathBuf;

use common::OkResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::Status,
    local::asynchronous::Client,
    serde::{
        json::json,
        uuid::Uuid,
    },
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
struct Vote {
    comparison_id: Uuid,
    user_id: Uuid,
    image: String,
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn put_new_vote_with_correct_parameters_returns_201_created(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20A.png",
        }))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Created);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn put_new_vote_with_correct_parameters_is_json_ok_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20A.png",
        }))
        .dispatch()
        .await;
    let body = response.into_json::<OkResponse<Vote>>().await;
    assert!(body.is_some());
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("users", "comparisons", "votes")
))]
async fn put_new_vote_with_correct_parameters_returns_expected_vote(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20A.png",
        }))
        .dispatch()
        .await;
    let body = response
        .into_json::<OkResponse<Vote>>()
        .await
        .expect("body to be present");
    let expected_vote = Vote {
        comparison_id: Uuid::parse_str("7d68f7e3-afe5-4d08-9d89-e6905f152eec")
            .unwrap(),
        user_id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6")
            .unwrap(),
        image: "/static/images/image%20A.png".to_string(),
    };
    assert_eq!(body.data, expected_vote);
}
