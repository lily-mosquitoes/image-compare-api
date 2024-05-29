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

mod get_image_with_existing_filename {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures()]
        let request = |client| {
            client.get(uri!("/static/images/image%20A.png"))
        };

        #[test_request]
        let returns_200_ok = |response| {
            assert_eq!(response.status(), Status::Ok);
        };

        #[test_request]
        let returns_bytes = |response| {
            let bytes = response.into_bytes().await;

            assert!(bytes.is_some());
        };

        #[test_request]
        let returns_expected_image = |response| {
            let bytes = response
                .into_bytes()
                .await
                .expect("bytes to be present");
            let file_path = std::path::Path::new(static_dir)
                .join("image A.png");
            let file = std::fs::read(file_path)
                .expect("file to be present");

            assert_eq!(bytes, file);
        };
    }
}

mod get_image_with_nonexisting_filename {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures()]
        let request = |client| {
            client.get(uri!("/static/images/does_not_exist.png"))
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
