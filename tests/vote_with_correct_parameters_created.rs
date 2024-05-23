mod common;

use std::net::IpAddr;

use chrono::{
    DateTime,
    Utc,
};
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
    id: i64,
    comparison_id: Uuid,
    user_id: Uuid,
    image: String,
    created_at: DateTime<Utc>,
    ip_addr: IpAddr,
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn post_vote_with_correct_parameters_returns_201_created(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .post(uri!("/api/vote"))
        .remote("127.0.0.1:80".parse().expect("remote to be parseable"))
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
    scripts("admins", "users", "comparisons", "votes")
))]
async fn post_vote_with_correct_parameters_is_json_ok_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/vote"))
        .remote("127.0.0.1:80".parse().expect("remote to be parseable"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20A.png",
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
async fn post_vote_with_correct_parameters_returns_expected_vote(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/vote"))
        .remote("127.0.0.1:80".parse().expect("remote to be parseable"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20A.png",
        }))
        .dispatch()
        .await
        .into_json::<OkResponse<Vote>>()
        .await
        .expect("body to be present");

    let expected_vote = Vote {
        id: body.data.id,
        comparison_id: Uuid::parse_str("7d68f7e3-afe5-4d08-9d89-e6905f152eec")
            .unwrap(),
        user_id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6")
            .unwrap(),
        image: "/static/images/image%20A.png".to_string(),
        created_at: body.data.created_at,
        ip_addr: "127.0.0.1".parse().expect("IP to be parseable"),
    };

    assert_eq!(body.data, expected_vote);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn post_vote_with_correct_parameters_is_not_idempotent(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let request = client
        .post(uri!("/api/vote"))
        .remote("127.0.0.1:80".parse().expect("remote to be parseable"))
        .json(&json!({
            "comparison_id": "7d68f7e3-afe5-4d08-9d89-e6905f152eec",
            "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
            "image": "/static/images/image%20A.png",
        }));

    let body_0 = request
        .clone()
        .dispatch()
        .await
        .into_json::<OkResponse<Vote>>()
        .await
        .expect("body to be present");

    let body_1 = request
        .clone()
        .dispatch()
        .await
        .into_json::<OkResponse<Vote>>()
        .await
        .expect("body to be present");

    assert_ne!(body_0.data.id, body_1.data.id);
}
