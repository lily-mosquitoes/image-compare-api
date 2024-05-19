mod common;

use rocket::{
    fs::relative,
    http::{
        uri::Origin,
        Header,
        Status,
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

#[derive(Debug, PartialEq, Deserialize)]
struct Comparison {
    dirname: String,
    images: Vec<Origin<'static>>,
    created_by: i64,
}

static STATIC_DIR: &'static str = relative!("tests/static_dir/ok");

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_ok_folder_returns_200_ok(
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

    assert_eq!(response.status(), Status::Created);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_ok_folder_is_json_ok_response(
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
        .into_json::<OkResponse<Vec<Comparison>>>()
        .await;

    assert!(body.is_some());
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_ok_folder_returns_expected_comparisons(
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
        .into_json::<OkResponse<Vec<Comparison>>>()
        .await
        .expect("body to be present");

    let expected_comparisons = vec![
        // root comparisons (AB, BA)
        Comparison {
            dirname: "".to_string(),
            images: vec![
                uri!("/static/images/image%20A.png"),
                uri!("/static/images/image%20B.png"),
            ],
            created_by: 1,
        },
        Comparison {
            dirname: "".to_string(),
            images: vec![
                uri!("/static/images/image%20B.png"),
                uri!("/static/images/image%20A.png"),
            ],
            created_by: 1,
        },
        // folder_a comparisons (12, 21, 13, 31, 23, 32)
        Comparison {
            dirname: "folder_a".to_string(),
            images: vec![
                uri!("/static/images/folder_a/image%201.png"),
                uri!("/static/images/folder_a/image%202.png"),
            ],
            created_by: 1,
        },
        Comparison {
            dirname: "folder_a".to_string(),
            images: vec![
                uri!("/static/images/folder_a/image%202.png"),
                uri!("/static/images/folder_a/image%201.png"),
            ],
            created_by: 1,
        },
        Comparison {
            dirname: "folder_a".to_string(),
            images: vec![
                uri!("/static/images/folder_a/image%201.png"),
                uri!("/static/images/folder_a/image%203.png"),
            ],
            created_by: 1,
        },
        Comparison {
            dirname: "folder_a".to_string(),
            images: vec![
                uri!("/static/images/folder_a/image%203.png"),
                uri!("/static/images/folder_a/image%201.png"),
            ],
            created_by: 1,
        },
        Comparison {
            dirname: "folder_a".to_string(),
            images: vec![
                uri!("/static/images/folder_a/image%202.png"),
                uri!("/static/images/folder_a/image%203.png"),
            ],
            created_by: 1,
        },
        Comparison {
            dirname: "folder_a".to_string(),
            images: vec![
                uri!("/static/images/folder_a/image%203.png"),
                uri!("/static/images/folder_a/image%202.png"),
            ],
            created_by: 1,
        },
        // folder_b/folder_c comparisons (45, 54)
        Comparison {
            dirname: "folder_b/folder_c".to_string(),
            images: vec![
                uri!("/static/images/folder_b/folder_c/image%204.png"),
                uri!("/static/images/folder_b/folder_c/image%205.png"),
            ],
            created_by: 1,
        },
        Comparison {
            dirname: "folder_b/folder_c".to_string(),
            images: vec![
                uri!("/static/images/folder_b/folder_c/image%205.png"),
                uri!("/static/images/folder_b/folder_c/image%204.png"),
            ],
            created_by: 1,
        },
    ];

    assert_eq!(body.data, expected_comparisons);
}

#[sqlx::test(fixtures(path = "./../fixtures", scripts("admins")))]
async fn generate_comparisons_from_ok_folder_returns_images_with_valid_origin(
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
        .into_json::<OkResponse<Vec<Comparison>>>()
        .await
        .expect("body to be present");

    for comparison in body.data {
        for image in comparison.images {
            let response = client.get(image).dispatch().await;

            assert_eq!(response.status(), Status::Ok);
        }
    }
}
