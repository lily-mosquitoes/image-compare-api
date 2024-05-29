mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::common::{
    make_api_test,
    ApiResponse,
};

#[derive(Debug, PartialEq, Deserialize)]
struct User {
    id: Uuid,
    comparisons: u64,
    average_lambda: f64,
}

mod generate_user {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures()]
        let request = |client| {
            client.post(uri!("/api/user"))
        };

        #[test_request]
        let returns_201_created = |response| {
            assert_eq!(response.status(), Status::Created);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<User, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_new_user = |response| {
            let json = response.into_json::<ApiResponse<User, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let expected_new_user = User {
                id: data.id,
                comparisons: 0,
                average_lambda: 0.0,
            };

            assert_eq!(data, expected_new_user);
        };

        #[test_request]
        let returns_new_user_wich_can_be_retrieved = |response| {
            let json = response.into_json::<ApiResponse<User, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let response = client
                .get(format!("/api/user/{}", data.id))
                .dispatch()
                .await;

            assert_eq!(response.status(), Status::Ok);
        };

        #[test_request]
        let is_not_idempotent = |response| {
            let json = response.into_json::<ApiResponse<User, ()>>()
                .await;
            let data_a = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let data_b = client
                .post(format!("/api/user"))
                .dispatch()
                .await
                .into_json::<ApiResponse<User, ()>>()
                .await
                .expect("json to be preset")
                .data
                .expect("data to be present");

            assert_ne!(data_a.id, data_b.id);
        };
    }
}
