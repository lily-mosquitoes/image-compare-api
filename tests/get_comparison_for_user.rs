mod common;

use rocket::{
    fs::relative,
    http::{
        uri::Origin,
        Status,
    },
    uri,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::common::{
    make_api_test,
    ApiResponse,
};

#[derive(Deserialize)]
struct Comparison {
    id: Uuid,
    images: Vec<Origin<'static>>,
}

mod get_comparison_for_user_with_correct_id {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client.get(uri!(
                "/api/user/3fa85f64-5717-4562-b3fc-2c963f66afa6/comparison"
            ))
        };

        #[test_request]
        let returns_200_ok = |response| {
            assert_eq!(response.status(), Status::Ok);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<Comparison, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_comparison_without_a_vote_for_this_user = |response| {
            let json = response.into_json::<ApiResponse<Comparison, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            assert_eq!(data.id.to_string(), "7d68f7e3-afe5-4d08-9d89-e6905f152eec");
        };

        #[test_request]
        let returns_comparison_with_2_images = |response| {
            let json = response.into_json::<ApiResponse<Comparison, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            assert_eq!(data.images.len(), 2);
        };

        #[test_request]
        let returns_images_with_valid_origin = |response| {
            let json = response.into_json::<ApiResponse<Comparison, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            for image in data.images {
                let response = client.get(image).dispatch().await;

                assert_eq!(response.status(), Status::Ok);
            }
        };
    }
}

mod get_comparison_for_user_with_incorrect_id {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons")]
        let request = |client| {
            client.get(uri!(
                "/api/user/f628fe0a-a3aa-4883-98f2-714c1e81cc3e/comparison"
            ))
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

mod get_comparison_for_user_with_incorrect_parameter {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons")]
        let request = |client| {
            client.get(uri!("/api/user/not-a-uuid/comparison"))
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
                "Semantic error in request: /api/user/not-a-uuid/comparison"
            );
        };
    }
}

mod get_comparison_for_user_when_none_available {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons", "votes")]
        let request = |client| {
            client.get(uri!(
                "/api/user/ac01a03d-75e3-4244-a33b-a2324b8784f1/comparison"
            ))
        };

        #[test_request]
        let returns_503_service_unavailable = |response| {
            assert_eq!(response.status(), Status::ServiceUnavailable);
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

            assert_eq!(error, "No `comparison` available for `user`");
        };
    }
}
