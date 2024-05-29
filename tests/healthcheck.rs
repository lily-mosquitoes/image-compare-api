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

mod healthcheck {
    use super::*;

    make_api_test! {
        #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
        #[fixtures()]
        let request = |client| {
            client.get(uri!("/api/healthcheck"))
        };

        #[test_request]
        let returns_200_ok = |response| {
            assert_eq!(response.status(), Status::Ok);
        };

        #[test_request]
        let returns_json_ok = |response| {
            let json = response.into_json::<ApiResponse<(), ()>>()
                .await;

            assert!(json.is_some());
        };
    }
}
