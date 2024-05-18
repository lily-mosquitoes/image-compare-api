mod common;

use std::path::PathBuf;

use common::OkResponse;
use image_compare_api;
use rocket::{
    fs::relative,
    http::{
        uri::Origin,
        Status,
    },
    local::asynchronous::Client,
    uri,
};
use serde::Deserialize;
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePoolOptions,
};
use uuid::Uuid;

#[derive(Deserialize)]
struct Comparison {
    id: Uuid,
    images: Vec<Origin<'static>>,
}

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

async fn get_http_client(db_options: SqliteConnectOptions) -> Client {
    let static_dir = PathBuf::from(STATIC_DIR);
    Client::untracked(image_compare_api::rocket(static_dir, db_options))
        .await
        .expect("valid rocket instance")
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons")
))]
async fn get_comparison_for_user_with_correct_id_returns_200_ok(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6/comparison"))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons")
))]
async fn get_comparison_for_user_with_correct_id_is_json_ok_response(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6/comparison"))
        .dispatch()
        .await;
    let body = response.into_json::<OkResponse<Comparison>>().await;
    assert!(body.is_some());
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons")
))]
fn get_comparison_for_user_with_correct_id_returns_2_images(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6/comparison"))
        .dispatch()
        .await;
    let body = response
        .into_json::<OkResponse<Comparison>>()
        .await
        .expect("body to be present");
    assert_eq!(body.data.images.len(), 2);
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons")
))]
fn get_comparison_for_user_with_correct_id_returns_images_with_valid_origin(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6/comparison"))
        .dispatch()
        .await;
    let body = response
        .into_json::<OkResponse<Comparison>>()
        .await
        .expect("body to be present");
    for image in body.data.images {
        let response = client.get(image).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }
}

#[sqlx::test(fixtures(
    path = "./../fixtures",
    scripts("admins", "users", "comparisons", "votes")
))]
fn get_comparison_for_user_with_correct_id_returns_comparison_without_vote(
    _: SqlitePoolOptions,
    db_options: SqliteConnectOptions,
) {
    let client = get_http_client(db_options).await;
    let response = client
        .get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6/comparison"))
        .dispatch()
        .await;
    let body = response
        .into_json::<OkResponse<Comparison>>()
        .await
        .expect("body to be present");
    assert_eq!(
        body.data.id,
        Uuid::parse_str("7d68f7e3-afe5-4d08-9d89-e6905f152eec").unwrap()
    );
}
