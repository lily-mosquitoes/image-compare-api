mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};
use serde::Deserialize;
use uuid::{
    uuid,
    Uuid,
};

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

mod get_user_with_correct_id {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("users")]
        let request = |client| {
            client.get(uri!("/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6"))
        };

        #[test_request]
        let returns_200_ok = |response| {
            assert_eq!(response.status(), Status::Ok);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<User, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_expected_user = |response| {
            let json = response.into_json::<ApiResponse<User, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            let expected_user = User {
                id: uuid!("3fa85f64-5717-4562-b3fc-2c963f66afa6"),
                comparisons: 1,
                average_lambda: 0.1234,
            };

            assert_eq!(data, expected_user);
        };
    }
}

mod get_user_with_incorrect_id {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("users")]
        let request = |client| {
            client.get(uri!("/api/user/a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"))
        };

        #[test_request]
        let returns_404_not_found = |response| {
            assert_eq!(response.status(), Status::NotFound);
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

mod get_user_with_incorrect_parameter {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("users")]
        let request = |client| {
            client.get(uri!("/api/user/not-a-uuid"))
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

            assert_eq!(
                error,
                "Semantic error in request: /api/user/not-a-uuid"
            );
        };
    }
}
