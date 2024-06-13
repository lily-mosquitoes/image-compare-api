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

use crate::common::{
    make_api_test,
    ApiResponse,
};

#[derive(Debug, PartialEq, Deserialize)]
struct Comparison {
    dirname: String,
    images: Vec<Origin<'static>>,
    created_by: i64,
}

impl PartialOrd for Comparison {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let as_string =
            |origin: &Origin<'static>| origin.path().as_str().to_string();
        let a: String = self.images.iter().map(as_string).collect();
        let b: String = other.images.iter().map(as_string).collect();
        Some(a.cmp(&b))
    }
}

mod generate_comparisons_from_folder_ok {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins")]
        let request = |client| {
            client
                .post(uri!("/api/admin/comparison"))
                .header(Header::new(
                    "Authorization",
                    "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
                ))
        };

        #[test_request]
        let returns_201_created = |response| {
            assert_eq!(response.status(), Status::Created);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<Vec<Comparison>, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_expected_data = |response| {
            let json = response.into_json::<ApiResponse<Vec<Comparison>, ()>>()
                .await;
            let mut data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let mut expected_data = vec![
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

            data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            expected_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for (c, e) in std::iter::zip(data, expected_data) {
                assert_eq!(c, e);
            }
        };

        #[test_request]
        let returns_images_with_valid_origin = |response| {
            let json = response.into_json::<ApiResponse<Vec<Comparison>, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            for comparison in data {
                for image in comparison.images {
                    let response = client.get(image).dispatch().await;

                    assert_eq!(response.status(), Status::Ok);
                }
            }
        };
    }
}

mod generate_comparisons_from_folder_error {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/error"))]
        #[fixtures("admins")]
        let request = |client| {
            client
                .post(uri!("/api/admin/comparison"))
                .header(Header::new(
                    "Authorization",
                    "Bearer ef8a53f0b0cb43dd764fe16a442752d6",
                ))
        };

        #[test_request]
        let returns_500_internal_server_error = |response| {
            assert_eq!(response.status(), Status::InternalServerError);
        };

        #[test_request]
        let returns_json_err = |response| {
            let json = response.into_json::<ApiResponse<(), String>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_expected_error = |response| {
            let json = response.into_json::<ApiResponse<(), String>>()
                .await;
            let error = json
                .expect("json to be present")
                .error
                .expect("error to be present");

            assert_eq!(
                error,
                "Not enough files in STATIC_DIR/folder_b (minimum 2 needed)"
            );
        };
    }
}

mod generate_comparisons_unauthorized {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins")]
        let request = |client| {
            client
                .post(uri!("/api/admin/comparison"))
                .header(Header::new(
                    "Authorization",
                    "Bearer c3e3a2f7a4bb2f9d1a470660c6d68b09",
                ))
        };

        #[test_request]
        let returns_401_unauthorized = |response| {
            assert_eq!(response.status(), Status::Unauthorized);
        };

        #[test_request]
        let returns_json_err = |response| {
            let json = response.into_json::<ApiResponse<(), String>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_expected_error = |response| {
            let json = response.into_json::<ApiResponse<(), String>>()
                .await;
            let error = json
                .expect("json to be present")
                .error
                .expect("error to be present");

            assert_eq!(error, "Unauthorized");
        };

        #[test_request]
        let returns_expected_header = |response| {
            let www_authenticate = response
                .headers()
                .get_one("WWW-Authenticate");

            assert_eq!(www_authenticate, Some("Bearer"));
        };
    }
}
