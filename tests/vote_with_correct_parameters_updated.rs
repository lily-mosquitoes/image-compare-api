mod common;

use rocket::{
    fs::relative,
    http::Status,
    serde::{
        json::json,
        uuid::Uuid,
    },
    uri,
};
use serde::Deserialize;
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePoolOptions,
};

use crate::common::{
    get_asynchronous_api_client,
    OkResponse,
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[derive(Debug, PartialEq, Deserialize)]
struct Vote {
    comparison_id: Uuid,
    user_id: Uuid,
    image: String,
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn put_existing_vote_with_correct_parameters_returns_200_ok(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "33993492-d8ce-4248-a93d-caf88baed82e",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20B.png",
        }))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn put_existing_vote_with_correct_parameters_is_json_ok_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "33993492-d8ce-4248-a93d-caf88baed82e",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20B.png",
        }))
        .dispatch()
        .await
        .into_json::<OkResponse<Vote>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn put_existing_vote_with_correct_parameters_returns_expected_vote(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .put(uri!("/api/vote"))
        .json(&json!({
            "comparison_id": "33993492-d8ce-4248-a93d-caf88baed82e",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20B.png",
        }))
        .dispatch()
        .await
        .into_json::<OkResponse<Vote>>()
        .await
        .expect("body to be present");

    let expected_vote = Vote {
        comparison_id: Uuid::parse_str("33993492-d8ce-4248-a93d-caf88baed82e")
            .unwrap(),
        user_id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6")
            .unwrap(),
        image: "/static/images/image%20B.png".to_string(),
    };

    assert_eq!(body.data, expected_vote);
}
