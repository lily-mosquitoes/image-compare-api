mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};
use sqlx::sqlite::SqliteConnectOptions;

use crate::common::{
    get_asynchronous_api_client,
    OkResponse,
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[sqlx::test]
async fn get_healthcheck_returns_200_ok() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client.get(uri!("/api/healthcheck")).dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test]
async fn get_healthcheck_is_json_ok_response() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/healthcheck"))
        .dispatch()
        .await
        .into_json::<OkResponse<()>>()
        .await;

    assert!(body.is_some());
}
