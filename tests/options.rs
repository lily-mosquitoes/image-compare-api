mod common;

use rocket::{
    fs::relative,
    http::Status,
    uri,
};

use crate::common::make_api_test;

macro_rules! make_api_preflight_test {
    ($name:ident, $uri:literal) => {
        mod $name {
            use super::*;

            make_api_test! {
                #[fileserver(static_dir = relative!("tests/static_dir/ok"))]
                #[fixtures()]
                let request = |client| {
                    client.options(uri!($uri))
                };

                #[test_request]
                let returns_204_no_content = |response| {
                    assert_eq!(response.status(), Status::NoContent);
                };

                #[test_request]
                let returns_no_body = |response| {
                    assert!(response.body().is_none());
                };

                #[test_request]
                let returns_expected_allow_methods_header = |response| {
                    let allow_methods =
                        response.headers().get_one("Access-Control-Allow-Methods");

                    assert_eq!(allow_methods, Some("OPTIONS, POST, DELETE, GET"));
                };

                #[test_request]
                let returns_expected_allow_headers_header = |response| {
                    let allow_headers =
                        response.headers().get_one("Access-Control-Allow-Headers");

                    assert_eq!(allow_headers, Some("Content-Type, Authorization"));
                };
            }
        }
    };
}

make_api_preflight_test!(
    options_preflight_for_generate_comparisons,
    "/api/admin/comparison"
);
make_api_preflight_test!(options_preflight_for_vote, "/api/vote");
