use rocket::{
    http::Status,
    serde::json::Json,
};

use crate::Response;

#[get("/healthcheck")]
pub(crate) async fn healthcheck() -> (Status, Json<Response<(), ()>>)
{
    let response = Response::from_result(Ok(()));

    (Status::Ok, Json(response))
}

#[cfg(test)]
mod test {
    use rocket::{
        http::Status,
        local::blocking::Client,
    };

    #[test]
    fn healthcheck() {
        let client = Client::tracked(crate::rocket())
            .expect("valid rocket instance");
        let response =
            client.get(uri!("/api", super::healthcheck)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_json::<crate::Response<(), ()>>();
        assert!(body.is_some());
    }
}
