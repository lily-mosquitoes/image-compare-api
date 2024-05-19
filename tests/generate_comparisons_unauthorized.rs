mod common;

use rocket::{
    fs::relative,
    http::{
        Header,
        Status,
    },
    uri,
};
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePoolOptions,
};

use crate::common::{
    get_asynchronous_api_client,
    ErrResponse,
};

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_with_incorrect_token_returns_401_unauthorized(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer c3e3a2f7a4bb2f9d1a470660c6d68b09",
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_with_incorrect_token_is_json_err_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer c3e3a2f7a4bb2f9d1a470660c6d68b09",
        ))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_with_incorrect_token_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer c3e3a2f7a4bb2f9d1a470660c6d68b09",
        ))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");

    let expected_error = "Unauthorized".to_string();

    assert_eq!(body.error, expected_error);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_with_incorrect_token_returns_expected_header(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer c3e3a2f7a4bb2f9d1a470660c6d68b09",
        ))
        .dispatch()
        .await;

    let www_authenticate = response.headers().get_one("WWW-Authenticate");

    assert_eq!(www_authenticate, Some("Bearer"));
}
