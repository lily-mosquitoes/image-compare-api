mod common;

use rocket::{
    fs::relative,
    uri,
};
use sqlx::sqlite::SqliteConnectOptions;

use crate::common::{
    get_asynchronous_api_client,
    ErrResponse,
    OkResponse,
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[sqlx::test]
async fn json_ok_response_has_request_id() {
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

#[sqlx::test]
async fn json_err_response_has_request_id() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/does_not_exist"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test]
async fn different_requests_have_different_request_ids() {
    let db_options = SqliteConnectOptions::new();
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body_0 = client
        .get(uri!("/api/healthcheck"))
        .dispatch()
        .await
        .into_json::<OkResponse<()>>()
        .await
        .expect("body to be present");

    let body_1 = client
        .get(uri!("/non/existent/path"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");

    assert_ne!(body_0.request_id, body_1.request_id);
}
