mod common;

use rocket::{
    fs::relative,
    http::Status,
    serde::uuid::Uuid,
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
struct User {
    id: Uuid,
    comparisons: u64,
    average_lambda: f64,
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("users")))]
async fn get_user_with_correct_id_returns_200_ok(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("users")))]
async fn get_user_with_correct_id_is_json_ok_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        .dispatch()
        .await
        .into_json::<OkResponse<User>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("users")))]
async fn get_user_with_correct_id_returns_expected_user(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        .dispatch()
        .await
        .into_json::<OkResponse<User>>()
        .await
        .expect("body to be present");

    let expected_user = User {
        id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
        comparisons: 1,
        average_lambda: 0.1234,
    };

    assert_eq!(body.data, expected_user);
}

#[sqlx::test]
async fn generate_user_returns_201_created(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client.post(uri!("/api/user")).dispatch().await;

    assert_eq!(response.status(), Status::Created);
}

#[sqlx::test]
async fn generate_user_is_json_ok_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/user"))
        .dispatch()
        .await
        .into_json::<OkResponse<User>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test]
async fn generate_user_returns_new_user(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/user"))
        .dispatch()
        .await
        .into_json::<OkResponse<User>>()
        .await
        .expect("body to be present");

    let expected_user = User {
        id: body.data.id,
        comparisons: 0,
        average_lambda: 0.0,
    };

    assert_eq!(body.data, expected_user);
}

#[sqlx::test]
async fn generate_user_is_not_idempotent(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body_a = client
        .post(uri!("/api/user"))
        .dispatch()
        .await
        .into_json::<OkResponse<User>>()
        .await
        .expect("body to be present");

    let body_b = client
        .post(uri!("/api/user"))
        .dispatch()
        .await
        .into_json::<OkResponse<User>>()
        .await
        .expect("body to be present");

    assert_ne!(body_a.data.id, body_b.data.id);
}
