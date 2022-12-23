use chrono::Utc;
use rocket::{
    http::Status,
    serde::json::Json,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::Response;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Healthcheck {
    message: String,
}

#[get("/healthcheck")]
pub(crate) async fn healthcheck(
) -> (Status, Json<Response<Healthcheck>>) {
    let response = Response {
        timestamp: Utc::now(),
        data: None,
    };

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
        let body = response
            .into_json::<crate::Response<super::Healthcheck>>();
        assert!(body.is_some());
    }
}
