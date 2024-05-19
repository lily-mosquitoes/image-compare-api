mod common;

use rocket::{
    fs::relative,
    http::Status,
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

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn get_comparison_for_user_when_none_available_returns_503_service_unavailable(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .get(uri!("/api/user/ac01a03d-75e3-4244-a33b-a2324b8784f1/comparison"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::ServiceUnavailable);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn get_comparison_for_user_when_none_available_is_json_err_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/user/ac01a03d-75e3-4244-a33b-a2324b8784f1/comparison"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
async fn get_comparison_for_user_when_none_available_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/user/ac01a03d-75e3-4244-a33b-a2324b8784f1/comparison"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to exist");

    assert_eq!(body.error, "No `comparison` available for `user`");
}
