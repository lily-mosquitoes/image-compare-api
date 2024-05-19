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

static STATIC_DIR: &'static str = relative!("tests/static_dir/error");

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_error_folder_returns_500_internal_server_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let response = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::InternalServerError);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_error_folder_is_json_err_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
        ))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_error_folder_returns_expected_error(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_asynchronous_api_client(STATIC_DIR, db_options).await;

    let body = client
        .post(uri!("/api/admin/comparison"))
        .header(Header::new(
            "Authorization",
            "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
        ))
        .dispatch()
        .await
        .into_json::<ErrResponse<String>>()
        .await
        .expect("body to be present");

    let expected_error = "Not enough files in STATIC_DIR/folder_b (minimum 2 \
                          needed)"
        .to_string();

    assert_eq!(body.error, expected_error);
}
