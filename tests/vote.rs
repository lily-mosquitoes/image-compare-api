mod common;

use std::net::IpAddr;

use chrono::{
    DateTime,
    Utc,
};
use rocket::{
    fs::relative,
    http::Status,
    serde::{
        json::json,
        uuid::{
            uuid,
            Uuid,
        },
    },
    uri,
};
use serde::Deserialize;

use crate::common::{
    make_api_test,
    ApiResponse,
};

#[derive(Debug, PartialEq, Deserialize)]
struct Vote {
    id: i64,
    comparison_id: Uuid,
    user_id: Uuid,
    vote_value: String,
    created_at: DateTime<Utc>,
    ip_addr: IpAddr,
}

mod vote_with_correct_parameters_and_vote_value_is_equal {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client
                .post(uri!("/api/vote"))
                .remote("127.0.0.1:80".parse().unwrap())
                .json(&json!({
                    "comparison_id": "33993492-d8ce-4248-a93d-caf88baed82e",
                    "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "vote_value": "equal",
                }))
        };

        #[test_request]
        let returns_201_created = |response| {
            assert_eq!(response.status(), Status::Created);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_expected_created_vote = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let expected_created_vote = Vote {
                id: data.id,
                comparison_id: uuid!("33993492-d8ce-4248-a93d-caf88baed82e"),
                user_id: uuid!("3fa85f64-5717-4562-b3fc-2c963f66afa6"),
                vote_value: "equal".to_string(),
                created_at: data.created_at,
                ip_addr: "127.0.0.1".parse().unwrap(),
            };

            assert_eq!(data, expected_created_vote);
        };

        #[test_request]
        let always_creates_a_new_vote = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            assert_ne!(data.id, 42);
        };
    }
}

mod vote_with_correct_parameters_and_vote_value_is_different {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client
                .post(uri!("/api/vote"))
                .remote("127.0.0.1:80".parse().unwrap())
                .json(&json!({
                    "comparison_id": "33993492-d8ce-4248-a93d-caf88baed82e",
                    "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "vote_value": "different",
                }))
        };

        #[test_request]
        let returns_201_created = |response| {
            assert_eq!(response.status(), Status::Created);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_expected_created_vote = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let expected_created_vote = Vote {
                id: data.id,
                comparison_id: uuid!("33993492-d8ce-4248-a93d-caf88baed82e"),
                user_id: uuid!("3fa85f64-5717-4562-b3fc-2c963f66afa6"),
                vote_value: "different".to_string(),
                created_at: data.created_at,
                ip_addr: "127.0.0.1".parse().unwrap(),
            };

            assert_eq!(data, expected_created_vote);
        };

        #[test_request]
        let always_creates_a_new_vote = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            assert_ne!(data.id, 42);
        };
    }
}

mod vote_with_correct_parameters_and_vote_value_is_image {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client
                .post(uri!("/api/vote"))
                .remote("127.0.0.1:80".parse().unwrap())
                .json(&json!({
                    "comparison_id": "33993492-d8ce-4248-a93d-caf88baed82e",
                    "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "vote_value": "/static/images/image%20B.png",
                }))
        };

        #[test_request]
        let returns_201_created = |response| {
            assert_eq!(response.status(), Status::Created);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_expected_created_vote = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let expected_created_vote = Vote {
                id: data.id,
                comparison_id: uuid!("33993492-d8ce-4248-a93d-caf88baed82e"),
                user_id: uuid!("3fa85f64-5717-4562-b3fc-2c963f66afa6"),
                vote_value: "/static/images/image%20B.png".to_string(),
                created_at: data.created_at,
                ip_addr: "127.0.0.1".parse().unwrap(),
            };

            assert_eq!(data, expected_created_vote);
        };

        #[test_request]
        let always_creates_a_new_vote = |response| {
            let json = response.into_json::<ApiResponse<Vote, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            assert_ne!(data.id, 42);
        };
    }
}

mod vote_with_incorrect_comparison_id {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client
                .post(uri!("/api/vote"))
                .remote("127.0.0.1:80".parse().unwrap())
                .json(&json!({
                    "comparison_id": "44444444-4444-4444-4444-444444444444",
                    "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "vote_value": "/static/images/image%20B.png",
                }))
        };

        #[test_request]
        let returns_422_unprocessable_entity = |response| {
            assert_eq!(response.status(), Status::UnprocessableEntity);
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
                .expect("json to be preset")
                .error
                .expect("error to be present");

            assert_eq!(error, "`comparison` with requested id not found");
        };
    }
}

mod vote_with_incorrect_user_id {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client
                .post(uri!("/api/vote"))
                .remote("127.0.0.1:80".parse().unwrap())
                .json(&json!({
                    "comparison_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "user_id": "44444444-4444-4444-4444-444444444444",
                    "vote_value": "/static/images/image%20B.png",
                }))
        };

        #[test_request]
        let returns_422_unprocessable_entity = |response| {
            assert_eq!(response.status(), Status::UnprocessableEntity);
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
                .expect("json to be preset")
                .error
                .expect("error to be present");

            assert_eq!(error, "`user` with requested id not found");
        };
    }
}

mod vote_with_incorrect_vote_value_image {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client
                .post(uri!("/api/vote"))
                .remote("127.0.0.1:80".parse().unwrap())
                .json(&json!({
                    "comparison_id": "33993492-d8ce-4248-a93d-caf88baed82e",
                    "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "vote_value": "/non/existing/image.png",
                }))
        };

        #[test_request]
        let returns_422_unprocessable_entity = |response| {
            assert_eq!(response.status(), Status::UnprocessableEntity);
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
                .expect("json to be preset")
                .error
                .expect("error to be present");

            assert_eq!(error, "`image` not found for requested `comparison`");
        };
    }
}
