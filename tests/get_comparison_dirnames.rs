mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};

use crate::common::{
    make_api_test,
    ApiResponse,
};

mod get_comparison_dirnames {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures("admins", "users", "comparisons")]
        let request = |client| {
            client.get(uri!(
                "/api/comparison/dirnames"
            ))
        };

        #[test_request]
        let returns_200_ok = |response| {
            assert_eq!(response.status(), Status::Ok);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<Vec<String>, ()>>()
                .await;

            assert!(json.is_some());
        };

        #[test_request]
        let returns_comparison_dirnames = |response| {
            let json = response.into_json::<ApiResponse<Vec<String>, ()>>()
                .await;
            let data = json
                .expect("json to be preset")
                .data
                .expect("data to be present");

            assert_eq!(data, ["", "folder_b/folder_c"]);
        };
    }
}

mod get_comparison_dirnames_when_none_available {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures()]
        let request = |client| {
            client.get(uri!(
                "/api/comparison/dirnames"
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

            assert_eq!(error, "No `comparison`s available");
        };
    }
}
