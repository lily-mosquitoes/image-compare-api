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
    scripts("admins", "users", "comparisons")
))]
async fn get_comparison_for_user_with_incorrect_id_returns_404_not_found(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .get(uri!("/api/user/f628fe0a-a3aa-4883-98f2-714c1e81cc3e/comparison"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons")
))]
async fn get_comparison_for_user_with_incorrect_id_is_json_error_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/user/f628fe0a-a3aa-4883-98f2-714c1e81cc3e/comparison"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons")
))]
async fn get_comparison_for_user_with_incorrect_id_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .get(uri!("/api/user/f628fe0a-a3aa-4883-98f2-714c1e81cc3e/comparison"))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to exist");

    assert_eq!(body.error, "`user` with requested id not found");
}
