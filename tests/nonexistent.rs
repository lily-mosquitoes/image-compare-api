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

mod nonexistent {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures()]
        let request = |client| {
            client.get(uri!("/non/existent/path"))
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
                .expect("json to be present")
                .error
                .expect("error to be present");

            assert_eq!(error, "Resource not found");
        };
    }
}
